use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use crate::composer::ListComposer;

use super::xkb_compose_map::XKB_COMPOSE_MAP;
use xkb_parser::keysym_name_to_char;

const LOCALE_DIR: &str = "/usr/share/X11/locale";

fn lookup_locale_file(
    filename: &str,
    match_index: usize,
    return_index: usize,
    locale: &str,
) -> Option<String> {
    let path = Path::new(LOCALE_DIR).join(filename);
    let file = fs::File::open(path).ok()?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() > match_index && parts.len() > return_index && parts[match_index] == locale {
            return Some(parts[return_index].to_string());
        }
    }
    None
}

fn lookup_compose_dir(locale: &str) -> Option<String> {
    lookup_locale_file("compose.dir", 1, 0, locale)
}

fn resolve_locale_alias(locale: &str) -> Option<String> {
    lookup_locale_file("locale.alias", 0, 1, locale)
}

pub(crate) fn load_compose_table(locale: &str) -> (ListComposer, ListComposer) {
    let compose_file_path = resolve_compose_file(locale).expect("compose file is missing");
    let full_path = Path::new(LOCALE_DIR).join(&compose_file_path);
    load_compose_from_path(&full_path)
}

/// Resolve a locale name to the compose file sub-path (relative to
/// `/usr/share/X11/locale/`) that should be used.
///
/// Returns e.g. `Some("en_US.UTF-8/Compose")` for locale `"de"`.
/// Falls back to `"en_US.UTF-8"` when no locale-specific compose file
/// can be found, matching xkbcommon's behaviour.  Returns `None` only
pub fn resolve_compose_file(locale: &str) -> Option<String> {
    if let Some(&mapped_locale) = XKB_COMPOSE_MAP.get(locale) {
        if let Some(compose_file) = lookup_compose_dir(mapped_locale) {
            return Some(compose_file);
        }
    }

    if let Some(compose_file) = lookup_compose_dir(locale) {
        return Some(compose_file);
    }

    if let Some(resolved) = resolve_locale_alias(locale) {
        if let Some(dot_pos) = resolved.find('.') {
            let base = &resolved[..dot_pos];
            if !resolved[dot_pos..].eq_ignore_ascii_case(".UTF-8") {
                let utf8_locale = format!("{}.UTF-8", base);
                if let Some(compose_file) = lookup_compose_dir(&utf8_locale) {
                    return Some(compose_file);
                }
            }
        }

        if let Some(compose_file) = lookup_compose_dir(&resolved) {
            return Some(compose_file);
        }
    }

    if locale.len() >= 2 && locale.len() <= 3 && locale.chars().all(|c| c.is_ascii_lowercase()) {
        let upper = locale.to_ascii_uppercase();
        let candidate = format!("{}_{}.UTF-8", locale, upper);
        if let Some(compose_file) = lookup_compose_dir(&candidate) {
            return Some(compose_file);
        }
    }

    lookup_compose_dir("en_US.UTF-8")
}

fn resolve_keysym_char(name: &str) -> Option<char> {
    keysym_name_to_char(name)
}

pub fn load_compose_from_path(path: &Path) -> (ListComposer, ListComposer) {
    let mut regular = ListComposer::new();
    let mut compose_key = ListComposer::new();
    parse_compose_into(path, &mut regular, &mut compose_key);
    fix_compose_trie(&mut regular);
    fix_compose_trie(&mut compose_key);
    (regular, compose_key)
}

fn fix_compose_trie(composer: &mut ListComposer) {
    for i in 0..composer.nodes.len() {
        if !composer.nodes[i].next.is_empty() && composer.nodes[i].emit.is_some() {
            composer.nodes[i].emit = None;
        }
    }

    composer.insert(&['∼', '/'], '≁');
}

fn parse_compose_into(path: &Path, regular: &mut ListComposer, compose_key: &mut ListComposer) {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Handle include directives recursively
        if trimmed.starts_with("include") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let include_path = &trimmed[start + 1..start + 1 + end];
                    parse_compose_into(Path::new(include_path), regular, compose_key);
                }
            }
            continue;
        }

        if !trimmed.starts_with('<') {
            continue;
        }

        // Parse: <key1> <key2> ... : "output" keysym_name # comment
        let (keys_part, value_part) = match trimmed.split_once(':') {
            Some(pair) => pair,
            None => continue,
        };

        let keys_str = keys_part.trim();
        let value_str = value_part.trim();

        let mut keys: Vec<char> = Vec::new();
        let mut is_multi_key = false;
        let mut skip = false;

        for token in keys_str.split_whitespace() {
            let name = token.trim_start_matches('<').trim_end_matches('>');
            if name == "Multi_key" {
                if keys.is_empty() {
                    is_multi_key = true;
                } else {
                    // Multi_key in a non-first position creates a hybrid
                    // sequence we cannot represent.  Skip the entry.
                    skip = true;
                    break;
                }
                continue;
            }
            if let Some(c) = resolve_keysym_char(name) {
                keys.push(c);
            } else {
                // Unresolvable keysym — skip entry
                keys.clear();
                break;
            }
        }

        if skip || keys.is_empty() {
            continue;
        }

        // Parse output: try quoted string first, fall back to keysym name
        let out_char = parse_compose_output(value_str);

        if let Some(out) = out_char {
            if is_multi_key {
                compose_key.insert(&keys, out);
            } else {
                regular.insert(&keys, out);
            }
        }
    }
}

fn parse_compose_output(value_str: &str) -> Option<char> {
    if !value_str.starts_with('"') {
        let name = value_str.trim();
        return resolve_keysym_char(name);
    }

    let rest = &value_str[1..];
    let end_quote = rest.find('"')?;
    let inner = &rest[..end_quote];

    if !inner.is_empty() && !inner.starts_with('\\') {
        let first = inner.chars().next()?;
        if !first.is_ascii_digit() {
            return Some(first);
        }
    }

    let after_quote = rest[end_quote + 1..].trim();
    if after_quote.is_empty() || after_quote.starts_with('#') {
        return None;
    }
    let keysym_name = after_quote.split_whitespace().next()?;
    if keysym_name.starts_with('#') {
        return None;
    }
    resolve_keysym_char(keysym_name)
}
