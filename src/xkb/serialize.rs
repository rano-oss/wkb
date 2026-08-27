//! Lossy WKB to XKB v1 serialization.
use super::keynames::{evdev_to_keyname, named_key_to_keysym};
use super::keysym::{keysym_for_state_modifier, keysym_get_name};
use crate::modifiers::Modifier;
use crate::named_keys::NamedKey;
use crate::{KBLayout, WKB};
use std::fmt::Write;
fn max_level(layout: &KBLayout, key: u32) -> usize {
    let levels = layout.state_keymap.num_levels;
    (0..levels)
        .rev()
        .find(|&level| {
            layout.named_key_map.get(level, key) != NamedKey::Unnamed
                || layout.level_exceptions_keymap.get(level, key).is_some()
        })
        .map_or_else(
            || usize::from(layout.modifiers.get(key).is_some()),
            |level| level + 1,
        )
}
fn type_name(layout: &KBLayout, key: u32, levels: usize) -> &'static str {
    if levels == 2
        && matches!(
            (
                layout.state_keymap.get(0, key),
                layout.state_keymap.get(1, key)
            ),
            (Some(lower), Some(upper))
                if lower.is_ascii_lowercase()
                    && upper == lower.to_ascii_uppercase()
        )
    {
        "ALPHABETIC"
    } else {
        match levels {
            0 | 1 => "ONE_LEVEL",
            2 => "TWO_LEVEL",
            3 | 4 => "FOUR_LEVEL",
            _ => "EIGHT_LEVEL",
        }
    }
}
fn sym_name(sym: u32) -> String {
    if sym == 0 {
        "NoSymbol".into()
    } else if (0x0100_0000..=0x0110_ffff).contains(&sym) {
        format!("{sym:#010x}")
    } else {
        keysym_get_name(sym)
            .map(|name| name.into_owned())
            .unwrap_or_else(|| format!("{sym:#010x}"))
    }
}
impl WKB {
    fn resolve_keysym(&self, layout: usize, level: usize, key: u32) -> u32 {
        let layout = &self.layouts[layout];
        let named = named_key_to_keysym(layout.named_key_map.get(level, key));
        if named != 0 {
            return named;
        }
        if let Some(ch) = layout
            .level_exceptions_keymap
            .get(level, key)
            .or_else(|| layout.state_keymap.get(level, key))
        {
            return 0x0100_0000 | ch as u32;
        }
        if level == 0 {
            return layout
                .modifiers
                .get(key)
                .and_then(|modifier| match modifier {
                    Modifier::Single(value) => keysym_for_state_modifier(value),
                    Modifier::Leveled(values) => values.values().next().and_then(keysym_for_state_modifier),
                })
                .unwrap_or(0);
        }
        0
    }
    pub(crate) fn generate_xkb_string(&self) -> String {
        let layouts = self.layouts.len();
        let keys = self
            .layouts
            .first()
            .map_or(0, |layout| layout.named_key_map.num_keys.min(248)) as u32;
        let used = |key| {
            self.layouts
                .iter()
                .any(|layout| max_level(layout, key) != 0)
        };
        let mut out = String::with_capacity(40 * 1024);
        out.push_str("xkb_keymap {\nxkb_keycodes \"wkb\" {\n\tminimum = 8;\n");
        writeln!(out, "\tmaximum = {};", keys + 7).unwrap();
        for key in (0..keys).filter(|&key| used(key)) {
            writeln!(out, "\t<{}> = {};", evdev_to_keyname(key), key + 8).unwrap();
        }
        out.push_str("\tindicator 1 = \"Caps Lock\";\n\tindicator 2 = \"Num Lock\";\n\tindicator 3 = \"Scroll Lock\";\n};\n\n");
        out.push_str(TYPES_XKB);
        out.push_str(COMPAT_XKB);
        out.push_str("xkb_symbols \"wkb\" {\n");
        for (index, layout) in self.layouts.iter().enumerate() {
            writeln!(out, "\tname[{}]= {:?};", index + 1, layout.name).unwrap();
        }
        for key in (0..keys).filter(|&key| used(key)) {
            let levels = self
                .layouts
                .iter()
                .map(|layout| max_level(layout, key))
                .max()
                .unwrap_or(1);
            writeln!(out, "\tkey <{}> {{", evdev_to_keyname(key)).unwrap();
            for (group, layout) in self.layouts.iter().enumerate() {
                writeln!(
                    out,
                    "\t\ttype[group{}]= {:?},",
                    group + 1,
                    type_name(layout, key, max_level(layout, key))
                )
                .unwrap();
                write!(out, "\t\tsymbols[group{}]= [ ", group + 1).unwrap();
                for level in 0..levels {
                    if level != 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&sym_name(self.resolve_keysym(group, level, key)));
                }
                out.push_str(if group + 1 == layouts { " ]" } else { " ],\n" });
            }
            if self
                .layouts
                .iter()
                .any(|layout| layout.repeat_keys.contains(key))
            {
                out.push_str(",\n\t\trepeat=Yes");
            }
            out.push_str("\n\t};\n");
        }
        out.push_str("\tmodifier_map Shift { <I050>, <I062> };\n\tmodifier_map Lock { <I066> };\n\tmodifier_map Control { <I037>, <I105> };\n\tmodifier_map Mod1 { <I064> };\n\tmodifier_map Mod2 { <I077> };\n\tmodifier_map Mod4 { <I133> };\n\tmodifier_map Mod5 { <I092> };\n};\n\n};\n");
        out
    }
}
const TYPES_XKB: &str = "xkb_types \"wkb\" {\n\tvirtual_modifiers NumLock,Alt,LevelThree,LevelFive;\n\n\ttype \"ONE_LEVEL\" {\n\t\tmodifiers= none;\n\t\tlevel_name[Level1]= \"Any\";\n\t};\n\ttype \"TWO_LEVEL\" {\n\t\tmodifiers= Shift;\n\t\tmap[Shift]= Level2;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t};\n\ttype \"ALPHABETIC\" {\n\t\tmodifiers= Shift+Lock;\n\t\tmap[Shift]= Level2;\n\t\tmap[Lock]= Level2;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t};\n\ttype \"FOUR_LEVEL\" {\n\t\tmodifiers= Shift+LevelThree;\n\t\tmap[Shift]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t};\n\ttype \"FOUR_LEVEL_SEMIALPHABETIC\" {\n\t\tmodifiers= Shift+Lock+LevelThree;\n\t\tmap[Shift]= Level2;\n\t\tmap[Lock]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tmap[Lock+LevelThree]= Level3;\n\t\tmap[Shift+Lock+LevelThree]= Level4;\n\t\tpreserve[Lock+LevelThree]= Lock;\n\t\tpreserve[Shift+Lock+LevelThree]= Lock;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t};\n\ttype \"EIGHT_LEVEL\" {\n\t\tmodifiers= Shift+LevelThree+LevelFive;\n\t\tmap[Shift]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tmap[LevelFive]= Level5;\n\t\tmap[Shift+LevelFive]= Level6;\n\t\tmap[LevelThree+LevelFive]= Level7;\n\t\tmap[Shift+LevelThree+LevelFive]= Level8;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t\tlevel_name[Level5]= \"X1\";\n\t\tlevel_name[Level6]= \"X2\";\n\t\tlevel_name[Level7]= \"X3\";\n\t\tlevel_name[Level8]= \"X4\";\n\t};\n};\n";
const COMPAT_XKB: &str = "xkb_compat \"wkb\" {\n\tvirtual_modifiers NumLock,Alt,LevelThree,LevelFive;\n\n\tinterpret Any+AnyOf(all) {\n\t\taction= SetMods(modifiers=modMapMods,clearLocks);\n\t};\n\tinterpret Shift_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Shift,clearLocks);\n\t};\n\tinterpret Shift_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Shift,clearLocks);\n\t};\n\tinterpret Caps_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=Lock);\n\t};\n\tinterpret Num_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=NumLock);\n\t};\n\tinterpret Control_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Control,clearLocks);\n\t};\n\tinterpret Control_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Control,clearLocks);\n\t};\n\tinterpret Alt_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Alt,clearLocks);\n\t};\n\tinterpret Super_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Mod4,clearLocks);\n\t};\n\tinterpret Super_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Mod4,clearLocks);\n\t};\n\tinterpret ISO_Level3_Shift+AnyOf(all) {\n\t\taction= SetMods(modifiers=LevelThree,clearLocks);\n\t};\n\tinterpret Scroll_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=Mod3);\n\t};\n\n\tindicator \"Caps Lock\" {\n\t\tmodifiers= Lock;\n\t};\n\tindicator \"Num Lock\" {\n\t\tmodifiers= NumLock;\n\t};\n\tindicator \"Scroll Lock\" {\n\t\tmodifiers= Mod3;\n\t};\n};\n";
