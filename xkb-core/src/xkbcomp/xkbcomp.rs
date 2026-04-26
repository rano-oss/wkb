use crate::shared_types::xkb_context;
use crate::shared_types::{xkb_component_names, xkb_rule_names};
pub fn xkb_components_from_rules_names(
    ctx: &mut xkb_context,
    rmlvo: &xkb_rule_names,
    out: &mut xkb_component_names,
    explicit_layouts: &mut u32,
) -> bool {
    crate::xkbcomp::rules::xkb_components_from_rules_names(ctx, rmlvo, out, explicit_layouts)
}

pub use crate::messages::XKB_ERROR_KEYMAP_COMPILATION_FAILED;
pub use crate::shared_ast_types::{xkb_file_type_to_string, XkbFile, FILE_TYPE_KEYMAP};
use crate::xkbcomp::ast_build::XkbFileFromComponents;
use crate::xkbcomp::keymap::CompileKeymap;
use crate::xkbcomp::scanner::XkbParseString;

pub use crate::shared_types::{format_max_groups, xkb_keymap};

fn compile_keymap_file(keymap: &mut xkb_keymap, file: &mut XkbFile) -> bool {
    if file.file_type != FILE_TYPE_KEYMAP {
        log::error!(
            "[XKB-{:03}] Cannot compile a {} file alone into a keymap\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32,
            xkb_file_type_to_string(file.file_type)
        );
        return false;
    }
    if !CompileKeymap(file, keymap) {
        log::error!(
            "[XKB-{:03}] Failed to compile keymap\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
        );
        return false;
    }
    true
}
pub fn text_v1_keymap_new_from_names(keymap: &mut xkb_keymap, rmlvo: &xkb_rule_names) -> bool {
    let mut ok: bool;
    let mut kccgst: xkb_component_names = xkb_component_names::default();

    log::debug!(
        "Compiling from RMLVO: rules '{}', model '{}', layout '{}', variant '{}', options '{}'\n",
        rmlvo.rules.to_str().unwrap_or(""),
        rmlvo.model.to_str().unwrap_or(""),
        rmlvo.layout.to_str().unwrap_or(""),
        rmlvo.variant.to_str().unwrap_or(""),
        rmlvo.options.to_str().unwrap_or("")
    );
    ok = xkb_components_from_rules_names(
        &mut keymap.ctx,
        rmlvo,
        &mut kccgst,
        &mut keymap.num_groups,
    );
    if !ok {
        log::error!("[XKB-{:03}] Couldn't look up rules '{}', model '{}', layout '{}', variant '{}', options '{}'\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32,
        &rmlvo.rules.to_str().unwrap_or(""),
        &rmlvo.model.to_str().unwrap_or(""),
        &rmlvo.layout.to_str().unwrap_or(""),
        &rmlvo.variant.to_str().unwrap_or(""),
        &rmlvo.options.to_str().unwrap_or(""));
        return false;
    }
    let max_groups: u32 = format_max_groups(keymap.format);
    if keymap.num_groups > max_groups {
        keymap.num_groups = max_groups;
    }
    // Safe conversion of Vec<i8> fields to string slices for logging
    fn vec_i8_to_str(v: &[i8]) -> String {
        let end = v.iter().position(|&b| b == 0).unwrap_or(v.len());
        v[..end].iter().map(|&b| b as u8 as char).collect()
    }
    log::debug!(
        "Compiling from KcCGST: keycodes '{}', types '{}', compat '{}', symbols '{}'\n",
        vec_i8_to_str(&kccgst.keycodes),
        vec_i8_to_str(&kccgst.types),
        vec_i8_to_str(&kccgst.compatibility),
        vec_i8_to_str(&kccgst.symbols),
    );
    let file_opt: Option<Box<XkbFile>> = XkbFileFromComponents(&mut keymap.ctx, &kccgst);
    drop(kccgst);
    let Some(mut file) = file_opt else {
        log::error!(
            "[XKB-{:03}] Failed to generate parsed XKB file from components\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
        );
        return false;
    };
    ok = compile_keymap_file(keymap, &mut file);
    ok
}
pub fn text_v1_keymap_new_from_string(keymap: &mut xkb_keymap, input: &[u8]) -> bool {
    let Some(mut xkb_file) = XkbParseString(&mut keymap.ctx, input, "(input string)", "") else {
        log::error!(
            "[XKB-{:03}] Failed to parse input xkb string\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
        );
        return false;
    };
    let ok: bool = compile_keymap_file(keymap, &mut xkb_file);
    ok
}
