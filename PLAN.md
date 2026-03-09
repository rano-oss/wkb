# Refactoring Plan: Move XKB Utilities from `wkb` to `xkb-parser`

Several pieces of logic in `wkb`'s `src/xkb/` module are purely about the XKB
format and have no dependency on `wkb`'s runtime keymap representation. They
belong in `xkb-parser`, which is the designated home for everything that
understands the XKB file format.

---

## Step 1 — Move `parse_include` into `xkb-parser` ✓ DONE

**Current location:** `wkb/src/xkb/mod.rs` (`parse_include`)

**What it does:** Splits an XKB include string such as `"us(basic)"` into a
locale/file name and an optional layout variant: `("us", Some("basic"))`.

**Why it belongs in `xkb-parser`:** The include string is already represented
in the AST by the `Include` node. Decomposing it is pure syntactic knowledge
about the XKB format. Moving it there also lets `wkb` drop its `regex`
dependency, which is currently only used by this one function.

**Plan:**
- Add a `parse_include(s: &str) -> (String, Option<String>)` free function (or
  an `impl Include` method) to `xkb-parser`, without using `regex` — a simple
  manual parse of the `locale(variant)` pattern is sufficient.
- Re-export it from `xkb-parser`'s public API.
- In `wkb`, remove the local `parse_include` function and replace its call
  sites with the one from `xkb-parser`.
- Remove `regex` from `wkb`'s `Cargo.toml` if it is no longer used elsewhere.

---

## Step 2 — Move keysym-name-to-char helpers into `xkb-parser` ✓ DONE

**Current locations:**
- `wkb/src/xkb/mod.rs` — `unicode_string_to_unicode_char` (`"U0041"` → `'A'`)
  and `hex_string_to_unicode_char` (`"0x0041"` → `'A'`)
- `wkb/src/xkb/compose_parse.rs` — `resolve_keysym_char` (duplicates the
  `U<hex>` branch and also consults `XKBCODES_DEF_TO_UTF8`)

**Why it belongs in `xkb-parser`:** These functions encode XKB's own
conventions for how keysym names represent Unicode code points. The `U<hex>`
and `0x<hex>` formats are part of the XKB/X11 spec, not of `wkb`'s runtime.
The duplication between `mod.rs` and `compose_parse.rs` is a direct symptom of
the logic living in the wrong crate.

**Plan:**
- Add a `keysym_name_to_char(name: &str) -> Option<char>` function to
  `xkb-parser` that handles:
  - Single-character names
  - `U<hex>` Unicode code point notation
  - `0x<hex>` hexadecimal notation
  - Lookup in `x11-keysymdef` for all named keysyms
- Re-export it from `xkb-parser`'s public API.
- In `wkb`, remove `unicode_string_to_unicode_char`, `hex_string_to_unicode_char`,
  and the duplicated logic in `resolve_keysym_char`, replacing all call sites
  with the new unified function from `xkb-parser`.

---

## Step 3 — Shrink `XKBCODES_DEF_TO_UTF8` against `x11-keysymdef`

**Current location:** `wkb/src/xkb/xkb_utf8.rs`

**What it is:** A manually maintained ~3600-entry `BTreeMap<&str, char>` mapping
XKB keysym names to their Unicode characters.

**Why it cannot simply be replaced:** `x11-keysymdef` covers only ~1500 of the
~3600 named entries in `XKBCODES_DEF_TO_UTF8`. The missing ~2000 entries
include entire scripts (Braille, Armenian punctuation, Arabic variants like
`Arabic_heh` and `Arabic_farsi_yeh`) as well as the large block of
`0x1000000+codepoint` hex-literal keysym names that XKB uses for arbitrary
Unicode characters. Dropping them causes real test regressions.

**What was done in Step 2:** `keysym_name_to_char` in `xkb-parser` was added as
a fallback after `XKBCODES_DEF_TO_UTF8` at all call sites, so any name not in
the manual table is tried against `x11-keysymdef` and the `U<hex>` / `0x<hex>`
notations. `XKBCODES_DEF_TO_UTF8` remains the primary lookup.

**Remaining plan:**
- Audit `XKBCODES_DEF_TO_UTF8` against `x11-keysymdef`: remove every entry
  whose name is present in `x11-keysymdef` with the same Unicode mapping,
  since `keysym_name_to_char` will resolve those via the fallback path.
- What cannot be removed are entries that `x11-keysymdef` lacks entirely —
  these must stay in `XKBCODES_DEF_TO_UTF8` (or be contributed upstream to
  the `x11-keysymdef` dataset).
- Once the table is pruned to only the genuinely missing entries, evaluate
  whether it is small enough to move into `xkb-parser` as a supplementary
  table alongside `keysym_name_to_char`.

---

## Step 4 — Move `read_layouts` into `xkb-parser` (without filtering)

**Current location:** `wkb/src/xkb/mod.rs` (`read_layouts`)

**What it does:** Parses an XKB file and returns the names of all
`xkb_symbols` blocks defined in it, currently filtering out a hardcoded list
of `sun_type*` / `suncompat` layout names.

**Why it belongs in `xkb-parser`:** The function is a straightforward query
over the parsed AST — "give me all `xkb_symbols` names in this file". That is
generic parser functionality with no dependency on `wkb`'s keymap runtime. The
sun-compat filter is `wkb`-specific policy and will be removed; callers that
need filtering can do so themselves.

**Plan:**
- Add a method `File::symbol_layout_names(&self) -> Vec<&str>` (or equivalent)
  to `xkb-parser` that returns the name of every `XkbSymbols` block without
  any filtering.
- In `wkb`, replace the `read_layouts` call sites with calls to the new method,
  applying any desired filtering in `wkb` itself if still needed.
- Delete the `read_layouts` function from `wkb/src/xkb/mod.rs`.