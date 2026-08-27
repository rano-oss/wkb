//! Compose-file loading and caching for layouts.

use super::keymap;
use crate::composer::Token;
use crate::flat_keymap::FlatKeymap;
use crate::Composer;
use arrayvec::ArrayVec;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

struct ComposeTableData {
    entries: Vec<keymap::ComposeEntry>,
    filtered: Mutex<HashMap<Vec<char>, Arc<Composer>>>,
}

type ComposeTable = Arc<ComposeTableData>;
type ComposeTableCache = Vec<(PathBuf, ComposeTable)>;

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

fn table_cache() -> MutexGuard<'static, ComposeTableCache> {
    COMPOSE_TABLE_CACHE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn parse_table(path: &Path) -> (ComposeTableData, bool) {
    let mut entries = Vec::new();

    let complete = keymap::parse_compose_file_impl(path, &mut |entry| {
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

fn load_table(path: &Path) -> ComposeTable {
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    if let Some((_, table)) = table_cache().iter().find(|(cached, _)| cached == &path) {
        return table.clone();
    }

    let (table, complete) = parse_table(&path);
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

fn build(entries: &[keymap::ComposeEntry], reachable: Option<&[char]>) -> Composer {
    let mut composer = Composer::new();

    for entry in entries {
        if reachable
            .is_some_and(|chars| !entry.keys.iter().all(|ch| chars.binary_search(ch).is_ok()))
        {
            continue;
        }

        let mut tokens: ArrayVec<Token, 9> = ArrayVec::new();

        for (index, &ch) in entry.keys.iter().enumerate() {
            if entry.multi_key_index == Some(index) {
                tokens.push(Token::Compose);
            }

            tokens.push(Token::Char(ch));
        }

        composer.insert(&tokens, entry.output);
    }

    composer
}

pub(crate) fn layout_composer(path: &Path, reachable: &[char]) -> Composer {
    let table = load_table(path);
    let mut filtered = table
        .filtered
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    if let Some(composer) = filtered.get(reachable) {
        return composer.as_ref().clone();
    }

    let composer = Arc::new(build(&table.entries, Some(reachable)));

    filtered.insert(reachable.to_vec(), composer.clone());
    composer.as_ref().clone()
}

#[doc(hidden)]
pub fn load_compose_from_path(path: &Path) -> Composer {
    build(&load_table(path).entries, None)
}

#[doc(hidden)]
pub fn load_compose_from_path_uncached(path: &Path) -> Composer {
    build(&parse_table(path).0.entries, None)
}
