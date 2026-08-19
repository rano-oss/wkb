//! Compose-file loading for layouts.
use super::keymap;
use crate::composer::Token;
use crate::Composer;
use arrayvec::ArrayVec;
use std::path::Path;

fn load(path: &Path, reachable: Option<&[char]>) -> Composer {
    let mut composer = Composer::new();
    keymap::parse_compose_file_impl(path, &mut |entry| {
        if reachable
            .is_some_and(|chars| !entry.keys.iter().all(|ch| chars.binary_search(ch).is_ok()))
        {
            return;
        }
        let mut tokens: ArrayVec<Token, 9> = ArrayVec::new();
        for (i, ch) in entry.keys.into_iter().enumerate() {
            if entry.multi_key_index == Some(i) {
                tokens.push(Token::Compose);
            }
            tokens.push(Token::Char(ch));
        }
        composer.insert(&tokens, entry.output);
    });
    composer
}

pub(crate) fn layout_composer(path: &Path, reachable: &[char]) -> Composer {
    load(path, Some(reachable))
}

#[doc(hidden)]
pub fn load_compose_from_path(path: &Path) -> Composer {
    load(path, None)
}

#[doc(hidden)]
pub fn load_compose_from_path_uncached(path: &Path) -> Composer {
    load(path, None)
}
