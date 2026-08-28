//! Compose-file loading and caching for layouts.

use super::keymap;
use super::keysym;
use super::parser::XkbKeymap;
use crate::composer::Token;
use crate::flat_keymap::FlatKeymap;
use crate::Composer;
use arrayvec::ArrayVec;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

struct ComposeTableData {
    entries: Vec<keymap::ComposeEntry>,
    filtered: Mutex<HashMap<ComposeFilterKey, Arc<Composer>>>,
}

type ComposeTable = Arc<ComposeTableData>;
type ComposeTableCache = Vec<(PathBuf, ComposeTable)>;

#[derive(Clone, Eq)]
struct ComposeFilterKey {
    reachable: Vec<char>,
    dead_keysyms: Vec<u32>,
}

impl PartialEq for ComposeFilterKey {
    fn eq(&self, other: &Self) -> bool {
        self.reachable == other.reachable && self.dead_keysyms == other.dead_keysyms
    }
}

impl Hash for ComposeFilterKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reachable.hash(state);
        self.dead_keysyms.hash(state);
    }
}

static COMPOSE_TABLE_CACHE: OnceLock<Mutex<ComposeTableCache>> = OnceLock::new();

/// Characters a compiled XKB layout can emit, for filtering locale compose tables.
pub(crate) fn reachable_chars(
    state_keymap: &FlatKeymap,
    caps_lock_keymap: &FlatKeymap,
    num_lock_keys: &FlatKeymap,
    caps_num_lock_keys: &FlatKeymap,
) -> Vec<char> {
    let mut reachable: Vec<char> = state_keymap
        .data
        .iter()
        .chain(&caps_lock_keymap.data)
        .chain(&num_lock_keys.data)
        .chain(&caps_num_lock_keys.data)
        .filter_map(|ch| *ch)
        .collect();
    reachable.sort_unstable();
    reachable.dedup();
    reachable
}

/// Dead-key keysyms present on a layout group.
pub(crate) fn layout_dead_keysyms(
    keymap: &XkbKeymap,
    layout_idx: usize,
    min_keycode: u32,
    max_keycode: u32,
) -> Vec<u32> {
    let mut dead = Vec::new();
    for (kc, key) in keymap.keys.iter().enumerate() {
        let kc = kc as u32;
        if kc < min_keycode || kc > max_keycode {
            continue;
        }
        if key.groups.is_empty() {
            continue;
        }
        let group = &key.groups[layout_idx % key.groups.len()];
        for level in &group.levels {
            if level.sym != 0 && keysym::is_dead_keysym(level.sym) {
                dead.push(level.sym);
            }
        }
    }
    dead.sort_unstable();
    dead.dedup();
    dead
}

fn table_cache() -> MutexGuard<'static, ComposeTableCache> {
    COMPOSE_TABLE_CACHE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn parse_table(path: &Path, locale: &str) -> (ComposeTableData, bool) {
    let mut entries = Vec::new();

    let complete = keymap::parse_compose_file_impl(path, locale, &mut |entry| {
        entries.push(entry);
    });

    (
        ComposeTableData {
            entries,
            filtered: Mutex::new(HashMap::new()),
        },
        complete,
    )
}

fn load_table(path: &Path, locale: &str) -> ComposeTable {
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    if let Some((_, table)) = table_cache().iter().find(|(cached, _)| cached == &path) {
        return table.clone();
    }

    let (table, complete) = parse_table(&path, locale);
    let table = Arc::new(table);

    if !complete {
        return table;
    }

    let mut cache = table_cache();

    // Another thread may have inserted it while this thread parsed.
    if let Some((_, existing)) = cache.iter().find(|(cached, _)| cached == &path) {
        return existing.clone();
    }

    cache.push((path, table.clone()));
    table
}

fn entry_matches(entry: &keymap::ComposeEntry, filter: &ComposeFilterKey) -> bool {
    for key in &entry.keys {
        if let Some(ks) = key.dead_keysym {
            if filter.dead_keysyms.binary_search(&ks).is_err() {
                return false;
            }
        } else if filter.reachable.binary_search(&key.ch).is_err() {
            return false;
        }
    }
    true
}

fn build(entries: &[keymap::ComposeEntry], filter: Option<&ComposeFilterKey>) -> Composer {
    let mut composer = Composer::new();

    for entry in entries {
        if filter.is_some_and(|filter| !entry_matches(entry, filter)) {
            continue;
        }

        let mut tokens: ArrayVec<Token, 9> = ArrayVec::new();

        for (index, key) in entry.keys.iter().enumerate() {
            if entry.multi_key_index == Some(index) {
                tokens.push(Token::Compose);
            }

            tokens.push(Token::Char(key.ch));
        }

        composer.insert(&tokens, entry.output);
    }

    composer
}

pub(crate) fn layout_composer(
    path: &Path,
    locale: &str,
    reachable: &[char],
    dead_keysyms: &[u32],
) -> Composer {
    let filter_key = ComposeFilterKey {
        reachable: reachable.to_vec(),
        dead_keysyms: dead_keysyms.to_vec(),
    };
    let table = load_table(path, locale);
    let mut filtered = table
        .filtered
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    if let Some(composer) = filtered.get(&filter_key) {
        return composer.as_ref().clone();
    }

    let composer = Arc::new(build(&table.entries, Some(&filter_key)));

    filtered.insert(filter_key, composer.clone());
    composer.as_ref().clone()
}

#[doc(hidden)]
pub fn load_compose_from_path(path: &Path) -> Composer {
    build(
        &load_table(path, "en_US.UTF-8").entries,
        None,
    )
}

#[doc(hidden)]
pub fn load_compose_from_path_uncached(path: &Path) -> Composer {
    build(&parse_table(path, "en_US.UTF-8").0.entries, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xkb::keymap::{ComposeEntry, ComposeKey};

    #[test]
    fn dead_key_rule_requires_layout_dead_keysym() {
        let mut keys = ArrayVec::<ComposeKey, 8>::new();
        keys.push(ComposeKey {
            ch: '¤',
            dead_keysym: Some(0xfe6f),
        });
        keys.push(ComposeKey {
            ch: 'a',
            dead_keysym: None,
        });
        let entry = ComposeEntry {
            keys,
            multi_key_index: None,
            output: '؋',
        };
        let no_dead = ComposeFilterKey {
            reachable: vec!['¤', 'a'],
            dead_keysyms: vec![],
        };
        let with_dead = ComposeFilterKey {
            reachable: vec!['a'],
            dead_keysyms: vec![0xfe6f],
        };
        assert!(!entry_matches(&entry, &no_dead));
        assert!(entry_matches(&entry, &with_dead));
    }

    #[test]
    fn multi_key_rule_uses_reachable_chars() {
        let mut keys = ArrayVec::<ComposeKey, 8>::new();
        keys.push(ComposeKey {
            ch: 'a',
            dead_keysym: None,
        });
        keys.push(ComposeKey {
            ch: 'e',
            dead_keysym: None,
        });
        let entry = ComposeEntry {
            keys,
            multi_key_index: Some(0),
            output: 'æ',
        };
        let filter = ComposeFilterKey {
            reachable: vec!['a', 'e'],
            dead_keysyms: vec![],
        };
        assert!(entry_matches(&entry, &filter));
        let missing = ComposeFilterKey {
            reachable: vec!['a'],
            dead_keysyms: vec![],
        };
        assert!(!entry_matches(&entry, &missing));
    }

    #[test]
    fn compose_include_merges_locale_file() {
        let path = crate::xkb::compose_paths::compose_locale_dir().join("fi_FI.UTF-8/Compose");
        if !path.is_file() {
            return;
        }
        let mut count = 0usize;
        keymap::parse_compose_file_impl(&path, "fi_FI.UTF-8", &mut |_| count += 1);
        assert!(
            count > 3_000,
            "fi_FI compose should pull in included en_US rules, got {count}"
        );
    }
}
