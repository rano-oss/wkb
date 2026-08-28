//! Compose file path resolution (mirrors libxkbcommon search order).

use std::path::{Path, PathBuf};

pub const DFLT_XLOCALEDIR: &str = "/usr/share/X11/locale";
const FALLBACK_LOCALE: &str = "en_US.UTF-8";
const FALLBACK_COMPOSE: &str = "en_US.UTF-8/Compose";

pub(crate) fn compose_locale_dir() -> PathBuf {
    PathBuf::from(std::env::var("XLOCALEDIR").unwrap_or_else(|_| DFLT_XLOCALEDIR.into()))
}

/// Resolve the Compose file for `locale`, checking user overrides first.
pub(crate) fn resolve_compose_path(locale: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("XCOMPOSEFILE") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }

    if let Some(path) = xdg_xcompose_path() {
        if path.is_file() {
            return Some(path);
        }
    }

    if let Some(path) = home_xcompose_path() {
        if path.is_file() {
            return Some(path);
        }
    }

    locale_compose_path(locale)
}

fn xdg_xcompose_path() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        if !dir.is_empty() && Path::new(&dir).is_absolute() {
            return Some(PathBuf::from(dir).join("XCompose"));
        }
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join(".config").join("XCompose"))
}

fn home_xcompose_path() -> Option<PathBuf> {
    Some(PathBuf::from(std::env::var("HOME").ok()?).join(".XCompose"))
}

fn locale_compose_path(locale: &str) -> Option<PathBuf> {
    let locale = if locale == "C" { FALLBACK_LOCALE } else { locale };
    let resolved = resolve_locale_alias(locale).unwrap_or_else(|| locale.to_string());

    if let Some(rel) = lookup_compose_dir(&resolved) {
        let path = compose_locale_dir().join(&rel);
        if path.is_file() {
            return Some(path);
        }
    }

    let fallback = lookup_compose_dir(FALLBACK_LOCALE).unwrap_or_else(|| FALLBACK_COMPOSE.into());
    let path = compose_locale_dir().join(fallback);
    path.is_file().then_some(path)
}

fn read_locale_text(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn resolve_locale_alias(locale: &str) -> Option<String> {
    let locale = locale.trim_end_matches(':');
    let content = read_locale_text(&compose_locale_dir().join("locale.alias"))?;
    for line in content.lines() {
        let Some(head) = line.split('#').next() else {
            continue;
        };
        let line = head.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(alias) = parts.next() else {
            continue;
        };
        let Some(canonical) = parts.next() else {
            continue;
        };
        if alias.trim_end_matches(':') == locale {
            return Some(canonical.trim_end_matches(':').to_string());
        }
    }
    None
}

fn lookup_compose_dir(locale: &str) -> Option<String> {
    let content = read_locale_text(&compose_locale_dir().join("compose.dir"))?;
    for line in content.lines() {
        let Some(head) = line.split('#').next() else {
            continue;
        };
        let line = head.trim();
        if line.is_empty() {
            continue;
        }
        let Some((file, name)) = parse_compose_dir_line(line) else {
            continue;
        };
        if name == locale {
            return Some(file);
        }
    }
    None
}

fn parse_compose_dir_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.split_whitespace();
    let file = parts.next()?.trim_end_matches(':').to_string();
    let locale = parts.next()?.trim_end_matches(':').to_string();
    Some((file, locale))
}

/// Expand `include` path specifiers (`%L`, `%H`, `%S`) and resolve relative paths.
pub(crate) fn expand_include_path(spec: &str, locale: &str, current: &Path) -> Option<PathBuf> {
    let mut out = String::new();
    let mut chars = spec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('L') => {
                    let path = resolve_compose_path(locale)?;
                    out.push_str(&path.to_string_lossy());
                }
                Some('H') => out.push_str(&std::env::var("HOME").ok()?),
                Some('S') => out.push_str(&compose_locale_dir().to_string_lossy()),
                Some(other) => {
                    out.push('%');
                    out.push(other);
                }
                None => out.push('%'),
            }
        } else {
            out.push(c);
        }
    }

    let path = PathBuf::from(out);
    if path.is_absolute() {
        Some(path)
    } else {
        Some(current.parent()?.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_dir_line_parse() {
        let (file, locale) =
            parse_compose_dir_line("en_US.UTF-8/Compose\t\tnb_NO.UTF-8").unwrap();
        assert_eq!(file, "en_US.UTF-8/Compose");
        assert_eq!(locale, "nb_NO.UTF-8");

        let (file, locale) =
            parse_compose_dir_line("iso8859-1/Compose:\t\tfi_FI.ISO8859-1").unwrap();
        assert_eq!(file, "iso8859-1/Compose");
        assert_eq!(locale, "fi_FI.ISO8859-1");
    }

    #[test]
    fn locale_alias_resolves_utf8_variant() {
        if !compose_locale_dir().join("locale.alias").is_file() {
            return;
        }
        assert_eq!(
            resolve_locale_alias("nb_NO.utf8").as_deref(),
            Some("nb_NO.UTF-8")
        );
    }

    #[test]
    fn nb_no_utf8_maps_to_compose_file() {
        if !compose_locale_dir().join("compose.dir").is_file() {
            return;
        }
        let rel = lookup_compose_dir("nb_NO.UTF-8").expect("nb_NO.UTF-8 in compose.dir");
        assert!(
            rel.contains("Compose"),
            "unexpected compose path: {rel}"
        );
    }

    #[test]
    fn fi_fi_has_locale_compose_file() {
        if !compose_locale_dir().join("compose.dir").is_file() {
            return;
        }
        let path = locale_compose_path("fi_FI.UTF-8").expect("fi_FI.UTF-8 compose");
        assert!(
            path.to_string_lossy().contains("fi_FI"),
            "expected fi_FI compose, got {}",
            path.display()
        );
    }
}
