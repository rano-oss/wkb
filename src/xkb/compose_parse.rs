use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use crate::composer::{ListComposer, Token};

use super::xkb_compose_map::XKB_COMPOSE_MAP;
use xkb_parser::parse_compose_file;

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

    if let Some(resolved) = lookup_locale_file("locale.alias", 0, 1, locale) {
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

pub fn load_compose_from_path(path: &Path) -> ListComposer {
    let mut regular = ListComposer::new();

    // Delegate parsing to xkb-parser. The parser resolves keysyms to chars
    // and preserves the Multi_key index in each returned entry.
    let entries = parse_compose_file(path);

    for entry in entries {
        // Build a token sequence for insertion. The parser returns `keys`
        // (a Vec<char>) and an optional `multi_key_index` which indicates
        // where a `Multi_key` appeared in the original sequence. The
        // composer now understands `Token::Compose` inline, so insert
        // `Token::Compose` at the reported index and otherwise `Token::Char`.
        let mut tokens: Vec<Token> = Vec::new();
        let mk_idx = entry.multi_key_index;

        for (i, ch) in entry.keys.iter().enumerate() {
            if let Some(idx) = mk_idx {
                if idx == i {
                    tokens.push(Token::Compose);
                }
            }
            tokens.push(Token::Char(*ch));
        }
        // Insert the constructed token sequence into the single composer.
        regular.insert(&tokens, entry.output);
    }
    regular
}
