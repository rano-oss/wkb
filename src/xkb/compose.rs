//! Compose-file loading and caching for layouts.

use crate::composer::Token;
use crate::Composer;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use super::keymap;

pub(crate) struct ComposeTableData {
    entries: Vec<keymap::ComposeEntry>,
    composer: Composer,
    filtered: Mutex<Vec<(Vec<char>, Arc<Composer>)>>,
}

pub(crate) type ComposeTable = Arc<ComposeTableData>;
type ComposeTableCache = Vec<(PathBuf, ComposeTable)>;

static COMPOSE_TABLE_CACHE: OnceLock<Mutex<ComposeTableCache>> = OnceLock::new();

fn compose_table_cache() -> MutexGuard<'static, ComposeTableCache> {
    COMPOSE_TABLE_CACHE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn cached_compose_table(cache: &ComposeTableCache, path: &Path) -> Option<ComposeTable> {
    cache
        .iter()
        .find(|(cached_path, _)| cached_path == path)
        .map(|(_, table)| table.clone())
}

fn parse_compose_table(path: &Path) -> (ComposeTableData, bool) {
    let mut entries = Vec::new();
    let complete = keymap::parse_compose_file_impl(path, &mut |entry| entries.push(entry));
    let composer = build_composer(&entries, None);
    (
        ComposeTableData {
            entries,
            composer,
            filtered: Mutex::new(Vec::new()),
        },
        complete,
    )
}

fn build_composer(entries: &[keymap::ComposeEntry], reachable: Option<&[char]>) -> Composer {
    use arrayvec::ArrayVec;

    let mut composer = Composer::new();
    for entry in entries {
        if let Some(chars) = reachable {
            if !entry.keys.iter().all(|ch| chars.binary_search(ch).is_ok()) {
                continue;
            }
        }
        let mut tokens: ArrayVec<Token, 9> = ArrayVec::new();
        let mk_idx = entry.multi_key_index;

        for (i, ch) in entry.keys.iter().enumerate() {
            if let Some(idx) = mk_idx {
                if idx == i {
                    tokens.push(Token::Compose);
                }
            }
            tokens.push(Token::Char(*ch));
        }
        composer.insert(&tokens, entry.output);
    }
    composer
}

pub(crate) fn layout_composer(table: &ComposeTable, reachable: &[char]) -> Composer {
    let mut cache = table
        .filtered
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some((_, composer)) = cache.iter().find(|(chars, _)| chars == reachable) {
        return composer.as_ref().clone();
    }
    let composer = Arc::new(build_composer(&table.entries, Some(reachable)));
    cache.push((reachable.to_vec(), composer.clone()));
    composer.as_ref().clone()
}

pub(crate) fn load_compose_entries(path: &Path) -> ComposeTable {
    let requested_path = path.to_path_buf();
    if let Some(table) = cached_compose_table(&compose_table_cache(), path) {
        return table;
    }
    let Ok(canonical_path) = std::fs::canonicalize(path) else {
        return Arc::new(parse_compose_table(path).0);
    };
    if canonical_path != requested_path {
        let mut cache = compose_table_cache();
        if let Some(table) = cached_compose_table(&cache, &canonical_path) {
            cache.push((requested_path, table.clone()));
            return table;
        }
    }

    let (entries, complete) = parse_compose_table(&canonical_path);
    if !complete {
        return Arc::new(entries);
    }

    let requested_is_alias = requested_path != canonical_path;
    let parsed = Arc::new(entries);
    let mut cache = compose_table_cache();
    let table = cached_compose_table(&cache, &canonical_path).unwrap_or_else(|| {
        cache.push((canonical_path, parsed.clone()));
        parsed
    });
    if requested_is_alias && cached_compose_table(&cache, &requested_path).is_none() {
        cache.push((requested_path, table.clone()));
    }
    table
}

/// Load a Compose file (via its parsed entries cached by canonical path) into
/// a full composer trie. Bench-only helper, not part of the public API.
#[doc(hidden)]
pub fn load_compose_from_path(path: &Path) -> Composer {
    load_compose_entries(path).composer.clone()
}

/// Parse a Compose file into a full composer trie without consulting the
/// canonical-path cache. Bench-only helper, not part of the public API.
#[doc(hidden)]
pub fn load_compose_from_path_uncached(path: &Path) -> Composer {
    parse_compose_table(path).0.composer
}
