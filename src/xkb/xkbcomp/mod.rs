// XKB compiler modules from libxkbcommon

pub(crate) mod parser;
pub(crate) mod symbols;

use self::parser::compile_keymap;
use self::parser::xkb_file_from_components;
use self::parser::xkb_parse_string;
use super::shared_types::{
    format_max_groups, XkbComponentNames, XkbFile, XkbKeymap, XkbRuleNames, FILE_TYPE_KEYMAP,
};

fn compile_keymap_file(keymap: &mut XkbKeymap, file: &mut XkbFile) -> bool {
    if file.file_type != FILE_TYPE_KEYMAP {
        return false;
    }
    if !compile_keymap(file, keymap) {
        return false;
    }
    true
}
pub(crate) fn text_v1_keymap_new_from_names(keymap: &mut XkbKeymap, rmlvo: &XkbRuleNames) -> bool {
    let mut ok: bool;
    let mut kccgst: XkbComponentNames = XkbComponentNames::default();

    ok = self::parser::xkb_components_from_rules_names(
        &mut keymap.ctx,
        rmlvo,
        &mut kccgst,
        &mut keymap.num_groups,
    );
    if !ok {
        return false;
    }
    let max_groups: u32 = format_max_groups(keymap.format);
    if keymap.num_groups > max_groups {
        keymap.num_groups = max_groups;
    }
    let file_opt: Option<Box<XkbFile>> = xkb_file_from_components(&mut keymap.ctx, &kccgst);
    drop(kccgst);
    let Some(mut file) = file_opt else {
        return false;
    };
    ok = compile_keymap_file(keymap, &mut file);
    ok
}
pub(crate) fn text_v1_keymap_new_from_string(keymap: &mut XkbKeymap, input: &[u8]) -> bool {
    let Some(mut xkb_file) = xkb_parse_string(&mut keymap.ctx, input, "(input string)", "") else {
        return false;
    };
    let ok: bool = compile_keymap_file(keymap, &mut xkb_file);
    ok
}
