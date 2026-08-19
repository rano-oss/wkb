//! Compose-file loading and caching for layouts.
use super::keymap;
use crate::composer::Token;
use crate::Composer;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
pub(crate) struct ComposeTableData {
    entries: Vec<keymap::ComposeEntry>,
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
fn parse_compose_table(path: &Path) -> (ComposeTableData, bool) {
    let mut entries = Vec::new();
    let complete = keymap::parse_compose_file_impl(path, &mut |entry| entries.push(entry));
    (
        ComposeTableData {
            entries,
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
        for (i, ch) in entry.keys.iter().enumerate() {
            if entry.multi_key_index == Some(i) {
                tokens.push(Token::Compose);
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
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    if let Some((_, table)) = compose_table_cache()
        .iter()
        .find(|(cached, _)| cached == &path)
    {
        return table.clone();
    }
    let (entries, complete) = parse_compose_table(&path);
    let parsed = Arc::new(entries);
    if !complete {
        return parsed;
    }
    let mut cache = compose_table_cache();
    if let Some((_, table)) = cache.iter().find(|(cached, _)| cached == &path) {
        return table.clone();
    }
    cache.push((path, parsed.clone()));
    parsed
}
#[doc(hidden)]
pub fn load_compose_from_path(path: &Path) -> Composer {
    build_composer(&load_compose_entries(path).entries, None)
}
#[doc(hidden)]
pub fn load_compose_from_path_uncached(path: &Path) -> Composer {
    build_composer(&parse_compose_table(path).0.entries, None)
}
