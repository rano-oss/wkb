//! Generate XKB v1 text format from WKB's flat keysym tables.
//!
//! This produces a minimal but valid keymap string that Wayland clients (GTK, Qt, etc.)
//! can parse to set up their own keyboard state. It synthesises key names from the
//! standard evdev offset (XKB keycode = evdev + 8) and infers key types from the
//! number of distinct keysym levels per key.

use crate::flat_keymap::{FlatKeysymMap, MAX_LEVELS};
use crate::keysyms::keysym_get_name;
use crate::WKB;

// ── Standard evdev → XKB key name table ──
// Indexed by evdev code (0-based). `None` entries use fallback `I{evdev+8:03}`.

const EVDEV_KEYNAMES: &[(u32, &str)] = &[
    (1, "ESC"),
    (2, "AE01"),
    (3, "AE02"),
    (4, "AE03"),
    (5, "AE04"),
    (6, "AE05"),
    (7, "AE06"),
    (8, "AE07"),
    (9, "AE08"),
    (10, "AE09"),
    (11, "AE10"),
    (12, "AE11"),
    (13, "AE12"),
    (14, "BKSP"),
    (15, "TAB"),
    (16, "AD01"),
    (17, "AD02"),
    (18, "AD03"),
    (19, "AD04"),
    (20, "AD05"),
    (21, "AD06"),
    (22, "AD07"),
    (23, "AD08"),
    (24, "AD09"),
    (25, "AD10"),
    (26, "AD11"),
    (27, "AD12"),
    (28, "RTRN"),
    (29, "LCTL"),
    (30, "AC01"),
    (31, "AC02"),
    (32, "AC03"),
    (33, "AC04"),
    (34, "AC05"),
    (35, "AC06"),
    (36, "AC07"),
    (37, "AC08"),
    (38, "AC09"),
    (39, "AC10"),
    (40, "AC11"),
    (41, "TLDE"),
    (42, "LFSH"),
    (43, "BKSL"),
    (44, "AB01"),
    (45, "AB02"),
    (46, "AB03"),
    (47, "AB04"),
    (48, "AB05"),
    (49, "AB06"),
    (50, "AB07"),
    (51, "AB08"),
    (52, "AB09"),
    (53, "AB10"),
    (54, "RTSH"),
    (55, "KPMU"),
    (56, "LALT"),
    (57, "SPCE"),
    (58, "CAPS"),
    (59, "FK01"),
    (60, "FK02"),
    (61, "FK03"),
    (62, "FK04"),
    (63, "FK05"),
    (64, "FK06"),
    (65, "FK07"),
    (66, "FK08"),
    (67, "FK09"),
    (68, "FK10"),
    (69, "NMLK"),
    (70, "SCLK"),
    (71, "KP7"),
    (72, "KP8"),
    (73, "KP9"),
    (74, "KPSU"),
    (75, "KP4"),
    (76, "KP5"),
    (77, "KP6"),
    (78, "KPAD"),
    (79, "KP1"),
    (80, "KP2"),
    (81, "KP3"),
    (82, "KP0"),
    (83, "KPDL"),
    (84, "LVL3"),
    (86, "LSGT"),
    (87, "FK11"),
    (88, "FK12"),
    (89, "AB11"),
    (90, "KATA"),
    (91, "HIRA"),
    (92, "HENK"),
    (93, "HKTG"),
    (94, "MUHE"),
    (95, "JPCM"),
    (96, "KPEN"),
    (97, "RCTL"),
    (98, "KPDV"),
    (99, "PRSC"),
    (100, "RALT"),
    (101, "LNFD"),
    (102, "HOME"),
    (103, "UP"),
    (104, "PGUP"),
    (105, "LEFT"),
    (106, "RGHT"),
    (107, "END"),
    (108, "DOWN"),
    (109, "PGDN"),
    (110, "INS"),
    (111, "DELE"),
    (113, "MUTE"),
    (116, "POWR"),
    (117, "KPEQ"),
    (119, "PAUS"),
    (122, "HNGL"),
    (123, "HJCV"),
    (124, "AE13"),
    (125, "LWIN"),
    (126, "RWIN"),
    (127, "COMP"),
    (128, "STOP"),
    (129, "AGAI"),
    (130, "PROP"),
    (131, "UNDO"),
    (132, "FRNT"),
    (133, "COPY"),
    (134, "OPEN"),
    (135, "PAST"),
    (136, "FIND"),
    (137, "CUT"),
    (138, "HELP"),
    (175, "MDSW"),
    (183, "FK13"),
    (184, "FK14"),
    (185, "FK15"),
    (186, "FK16"),
    (187, "FK17"),
    (188, "FK18"),
    (189, "FK19"),
    (190, "FK20"),
    (191, "FK21"),
    (192, "FK22"),
    (193, "FK23"),
    (194, "FK24"),
    (195, "LVL5"),
    (196, "ALT"),
    (197, "META"),
    (198, "SUPR"),
    (199, "HYPR"),
    // Well-known multimedia / generic keys
    (114, "VOLD"),
    (115, "VOLU"),
    (163, "NEXS"),
    (164, "PLPA"),
    (165, "PRVS"),
    (166, "STPS"),
    (172, "I180"),
    (176, "I184"),
    (177, "I185"),
    (178, "I186"),
    (179, "I187"),
    (180, "I188"),
    (181, "I189"),
    (182, "I190"),
];

/// Get the XKB key name for an evdev code.
fn evdev_to_keyname(evdev: u32) -> String {
    // Binary search or linear — table is small
    for &(code, name) in EVDEV_KEYNAMES {
        if code == evdev {
            return name.to_string();
        }
    }
    // Fallback: generic I{xkb_keycode} format
    format!("I{:03}", evdev + 8)
}

/// Determine how many levels a key actually uses across all groups.
fn key_max_level(keysym_map: &FlatKeysymMap, evdev: u32, num_layouts: usize) -> usize {
    let mut max_level = 0;
    for layout in 0..num_layouts {
        for level in (0..MAX_LEVELS).rev() {
            if keysym_map.get(layout, level, evdev) != 0 {
                if level + 1 > max_level {
                    max_level = level + 1;
                }
                break;
            }
        }
    }
    max_level
}

/// Pick a key type name based on the number of levels.
fn type_for_levels(num_levels: usize) -> &'static str {
    match num_levels {
        0 | 1 => "ONE_LEVEL",
        2 => "TWO_LEVEL",
        3 | 4 => "FOUR_LEVEL",
        _ => "EIGHT_LEVEL",
    }
}

/// Format a keysym as its XKB name, or hex fallback.
fn sym_name(sym: u32) -> String {
    if sym == 0 {
        return "NoSymbol".to_string();
    }
    if let Some(name) = keysym_get_name(sym) {
        name.to_string()
    } else if (0x0100_0000..=0x0110_ffff).contains(&sym) {
        // Unicode keysym: 0x1000000 + codepoint → "U<4-6 hex digits>"
        let cp = sym - 0x0100_0000;
        format!("U{:04X}", cp)
    } else {
        format!("0x{:08x}", sym)
    }
}

impl WKB {
    /// Generate XKB v1 text format string from flat keysym tables.
    ///
    /// This produces a minimal but fully valid keymap that Wayland clients
    /// can parse. Returns `None` if the keysym map is empty (non-xkb build).
    #[cfg(feature = "xkb")]
    pub fn generate_xkb_string(&self) -> String {
        let num_layouts = self.keysym_map.num_layouts;
        let num_keys = self.keysym_map.num_keys;
        // XKB keycodes max at 255; evdev = xkb - 8, so max evdev = 247
        let max_evdev = num_keys.min(248) as u32;
        // Estimate capacity: ~40KB for a typical keymap
        let mut out = String::with_capacity(40 * 1024);

        out.push_str("xkb_keymap {\n");

        // ── xkb_keycodes ──
        self.write_keycodes(&mut out, max_evdev);

        // ── xkb_types ──
        write_types(&mut out);

        // ── xkb_compat ──
        write_compat(&mut out);

        // ── xkb_symbols ──
        self.write_symbols(&mut out, max_evdev, num_layouts);

        out.push_str("};\n");
        out
    }

    fn write_keycodes(&self, out: &mut String, max_evdev: u32) {
        use std::fmt::Write;

        out.push_str("xkb_keycodes \"wkb\" {\n");
        writeln!(out, "\tminimum = 8;").unwrap();
        writeln!(out, "\tmaximum = {};", max_evdev + 8 - 1).unwrap();

        for evdev in 0..max_evdev {
            // Only emit keys that have at least one keysym
            if key_max_level(&self.keysym_map, evdev, self.keysym_map.num_layouts) > 0 {
                let name = evdev_to_keyname(evdev);
                writeln!(out, "\t<{}> = {};", name, evdev + 8).unwrap();
            }
        }

        // LED indicators
        out.push_str("\tindicator 1 = \"Caps Lock\";\n");
        out.push_str("\tindicator 2 = \"Num Lock\";\n");
        out.push_str("\tindicator 3 = \"Scroll Lock\";\n");
        out.push_str("};\n\n");
    }

    fn write_symbols(&self, out: &mut String, max_evdev: u32, num_layouts: usize) {
        use std::fmt::Write;

        out.push_str("xkb_symbols \"wkb\" {\n");

        // Group names
        for (i, name) in self.layout_names.iter().enumerate() {
            writeln!(out, "\tname[{}]= \"{}\";", i + 1, name).unwrap();
        }
        out.push('\n');

        // Per-key symbols
        for evdev in 0..max_evdev {
            let max_level = key_max_level(&self.keysym_map, evdev, num_layouts);
            if max_level == 0 {
                continue;
            }
            let name = evdev_to_keyname(evdev);
            let type_name = type_for_levels(max_level);

            if num_layouts == 1 {
                // Single-group format
                write!(out, "\tkey <{}> {{ type= \"{}\",", name, type_name).unwrap();
                out.push_str(" [ ");
                for level in 0..max_level {
                    if level > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&sym_name(self.keysym_map.get(0, level, evdev)));
                }
                out.push_str(" ]");
                // repeat
                if self.repeat_keys.contains(evdev) {
                    out.push_str(", repeat=Yes");
                }
                out.push_str(" };\n");
            } else {
                // Multi-group format
                out.push_str("\tkey <");
                out.push_str(&name);
                out.push_str("> {\n");
                // Per-group types
                for g in 0..num_layouts {
                    // Compute per-group level count
                    let mut glevel = 0;
                    for level in (0..MAX_LEVELS).rev() {
                        if self.keysym_map.get(g, level, evdev) != 0 {
                            glevel = level + 1;
                            break;
                        }
                    }
                    let gt = type_for_levels(glevel.max(max_level));
                    writeln!(out, "\t\ttype[group{}]= \"{}\",", g + 1, gt).unwrap();
                }
                // Per-group symbols
                for g in 0..num_layouts {
                    write!(out, "\t\tsymbols[{}]= [ ", g + 1).unwrap();
                    for level in 0..max_level {
                        if level > 0 {
                            out.push_str(", ");
                        }
                        out.push_str(&sym_name(self.keysym_map.get(g, level, evdev)));
                    }
                    if g < num_layouts - 1 {
                        out.push_str(" ],\n");
                    } else {
                        out.push_str(" ]");
                    }
                }
                if self.repeat_keys.contains(evdev) {
                    out.push_str(",\n\t\trepeat=Yes");
                }
                out.push('\n');
                out.push_str("\t};\n");
            }
        }

        // modifier_map entries
        out.push_str("\tmodifier_map Shift { <LFSH> };\n");
        out.push_str("\tmodifier_map Shift { <RTSH> };\n");
        out.push_str("\tmodifier_map Lock { <CAPS> };\n");
        out.push_str("\tmodifier_map Control { <LCTL> };\n");
        out.push_str("\tmodifier_map Control { <RCTL> };\n");
        out.push_str("\tmodifier_map Mod1 { <LALT> };\n");
        out.push_str("\tmodifier_map Mod2 { <NMLK> };\n");
        out.push_str("\tmodifier_map Mod4 { <LWIN> };\n");
        out.push_str("\tmodifier_map Mod5 { <LVL3> };\n");
        out.push_str("};\n\n");
    }
}

/// Write standard key types.
fn write_types(out: &mut String) {
    out.push_str(
        r#"xkb_types "wkb" {
	virtual_modifiers NumLock,Alt,LevelThree,LevelFive;

	type "ONE_LEVEL" {
		modifiers= none;
		level_name[Level1]= "Any";
	};
	type "TWO_LEVEL" {
		modifiers= Shift;
		map[Shift]= Level2;
		level_name[Level1]= "Base";
		level_name[Level2]= "Shift";
	};
	type "ALPHABETIC" {
		modifiers= Shift+Lock;
		map[Shift]= Level2;
		map[Lock]= Level2;
		level_name[Level1]= "Base";
		level_name[Level2]= "Shift";
	};
	type "FOUR_LEVEL" {
		modifiers= Shift+LevelThree;
		map[Shift]= Level2;
		map[LevelThree]= Level3;
		map[Shift+LevelThree]= Level4;
		level_name[Level1]= "Base";
		level_name[Level2]= "Shift";
		level_name[Level3]= "Alt Base";
		level_name[Level4]= "Shift Alt";
	};
	type "FOUR_LEVEL_SEMIALPHABETIC" {
		modifiers= Shift+Lock+LevelThree;
		map[Shift]= Level2;
		map[Lock]= Level2;
		map[LevelThree]= Level3;
		map[Shift+LevelThree]= Level4;
		map[Lock+LevelThree]= Level3;
		map[Shift+Lock+LevelThree]= Level4;
		preserve[Lock+LevelThree]= Lock;
		preserve[Shift+Lock+LevelThree]= Lock;
		level_name[Level1]= "Base";
		level_name[Level2]= "Shift";
		level_name[Level3]= "Alt Base";
		level_name[Level4]= "Shift Alt";
	};
	type "EIGHT_LEVEL" {
		modifiers= Shift+LevelThree+LevelFive;
		map[Shift]= Level2;
		map[LevelThree]= Level3;
		map[Shift+LevelThree]= Level4;
		map[LevelFive]= Level5;
		map[Shift+LevelFive]= Level6;
		map[LevelThree+LevelFive]= Level7;
		map[Shift+LevelThree+LevelFive]= Level8;
		level_name[Level1]= "Base";
		level_name[Level2]= "Shift";
		level_name[Level3]= "Alt Base";
		level_name[Level4]= "Shift Alt";
		level_name[Level5]= "X1";
		level_name[Level6]= "X2";
		level_name[Level7]= "X3";
		level_name[Level8]= "X4";
	};
};

"#,
    );
}

/// Write a minimal but valid xkb_compat section.
fn write_compat(out: &mut String) {
    out.push_str(
        r#"xkb_compat "wkb" {
	virtual_modifiers NumLock,Alt,LevelThree,LevelFive;

	interpret Any+AnyOf(all) {
		action= SetMods(modifiers=modMapMods,clearLocks);
	};
	interpret Shift_L+AnyOf(all) {
		action= SetMods(modifiers=Shift,clearLocks);
	};
	interpret Shift_R+AnyOf(all) {
		action= SetMods(modifiers=Shift,clearLocks);
	};
	interpret Caps_Lock+AnyOf(all) {
		action= LockMods(modifiers=Lock);
	};
	interpret Num_Lock+AnyOf(all) {
		action= LockMods(modifiers=NumLock);
	};
	interpret Control_L+AnyOf(all) {
		action= SetMods(modifiers=Control,clearLocks);
	};
	interpret Control_R+AnyOf(all) {
		action= SetMods(modifiers=Control,clearLocks);
	};
	interpret Alt_L+AnyOf(all) {
		action= SetMods(modifiers=Alt,clearLocks);
	};
	interpret Super_L+AnyOf(all) {
		action= SetMods(modifiers=Mod4,clearLocks);
	};
	interpret Super_R+AnyOf(all) {
		action= SetMods(modifiers=Mod4,clearLocks);
	};
	interpret ISO_Level3_Shift+AnyOf(all) {
		action= SetMods(modifiers=LevelThree,clearLocks);
	};
	interpret Scroll_Lock+AnyOf(all) {
		action= LockMods(modifiers=Mod3);
	};

	indicator "Caps Lock" {
		modifiers= Lock;
	};
	indicator "Num Lock" {
		modifiers= NumLock;
	};
	indicator "Scroll Lock" {
		modifiers= Mod3;
	};
};

"#,
    );
}
