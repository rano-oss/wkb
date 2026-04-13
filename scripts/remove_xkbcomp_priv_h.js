#!/usr/bin/env node
// Remove all pub mod xkbcomp_priv_h blocks
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const ROOT = '/home/rano/Public/wkb/src/xkb';

// First, add FreeXkbFile re-export to shared_ast_types.rs (alongside existing ast_build re-exports)
const sharedFile = path.join(ROOT, 'shared_ast_types.rs');
let sharedContent = fs.readFileSync(sharedFile, 'utf8');
if (!sharedContent.includes('FreeXkbFile')) {
  sharedContent = sharedContent.replace(
    'pub use crate::xkb::xkbcomp::ast_build::{stmt_type_to_string, xkb_file_type_to_string, stmt_type_to_operator_char};',
    'pub use crate::xkb::xkbcomp::ast_build::{stmt_type_to_string, xkb_file_type_to_string, stmt_type_to_operator_char, FreeXkbFile};'
  );
  fs.writeFileSync(sharedFile, sharedContent);
  console.log('Added FreeXkbFile to shared_ast_types.rs re-exports');
}

// Map of file-specific C2Rust_Unnamed aliases from xkbcomp_priv_h blocks
// These map C2Rust_Unnamed_XX -> canonical name, per-file
const aliasReplacements = {
  'src/xkb/xkbcomp/action.rs': { 'C2Rust_Unnamed_14': 'XkbcompLookup', 'C2Rust_Unnamed_15': 'XkbcompFeatures' },
  'src/xkb/xkbcomp/keycodes.rs': { 'C2Rust_Unnamed_14': 'XkbcompLookup', 'C2Rust_Unnamed_15': 'XkbcompFeatures' },
  'src/xkb/xkbcomp/types.rs': { 'C2Rust_Unnamed_14': 'XkbcompLookup', 'C2Rust_Unnamed_15': 'XkbcompFeatures' },
  'src/xkb/xkbcomp/expr.rs': { 'C2Rust_Unnamed_14': 'XkbcompLookup', 'C2Rust_Unnamed_15': 'XkbcompFeatures' },
  'src/xkb/xkbcomp/compat.rs': { 'C2Rust_Unnamed_16': 'XkbcompLookup', 'C2Rust_Unnamed_17': 'XkbcompFeatures' },
  'src/xkb/xkbcomp/symbols.rs': { 'C2Rust_Unnamed_16': 'XkbcompLookup', 'C2Rust_Unnamed_17': 'XkbcompFeatures' },
  'src/xkb/xkbcomp/keymap.rs': { 'C2Rust_Unnamed_17': 'XkbcompLookup', 'C2Rust_Unnamed_18': 'XkbcompFeatures' },
};

const files = execSync(`grep -rln 'pub mod xkbcomp_priv_h' ${ROOT}/`, { encoding: 'utf8' }).trim().split('\n');

let totalRemoved = 0;
for (const file of files) {
  let content = fs.readFileSync(file, 'utf8');
  const origLen = content.split('\n').length;
  const shortName = file.replace('/home/rano/Public/wkb/', '');

  // Step 1: Remove pub mod xkbcomp_priv_h { ... } block
  const lines = content.split('\n');
  const newLines = [];
  let inBlock = false;
  let depth = 0;
  for (const line of lines) {
    if (!inBlock && line.match(/^pub mod xkbcomp_priv_h\s*\{/)) {
      inBlock = true;
      depth = 0;
    }
    if (inBlock) {
      for (const ch of line) {
        if (ch === '{') depth++;
        if (ch === '}') depth--;
      }
      if (depth === 0) { inBlock = false; continue; }
      continue;
    }
    newLines.push(line);
  }
  content = newLines.join('\n');

  // Step 2: Replace C2Rust_Unnamed aliases from xkbcomp_priv_h
  // ONLY replace in pub use self::xkbcomp_priv_h re-export lines (where they reference the alias)
  // These aliases are imported via `pub use self::xkbcomp_priv_h::{..., C2Rust_Unnamed_14, ...}`
  // After block removal, we convert to `pub use crate::xkb::shared_ast_types::{..., XkbcompLookup, ...}`
  const aliases = aliasReplacements[shortName];
  if (aliases) {
    for (const [alias, canonical] of Object.entries(aliases)) {
      // But we must be careful: some files have file-local structs with the same name (C2Rust_Unnamed_16, etc.)
      // The xkbcomp_priv_h aliases are ONLY referenced in:
      //   1. pub use self::xkbcomp_priv_h::{..., C2Rust_Unnamed_XX, ...} lines (being converted)
      //   2. Actual code usage
      // We should NOT replace file-local struct definitions with the same name
      // Check if file has its own struct definition for this alias
      const hasLocalStruct = content.includes(`pub struct ${alias}`);
      if (!hasLocalStruct) {
        // Safe to replace all occurrences
        content = content.replace(new RegExp(`\\b${alias}\\b`, 'g'), canonical);
      }
      // If it has a local struct, we need to only replace in the import lines, not the struct/usage
    }
  }

  // Step 3: Convert pub use self::xkbcomp_priv_h:: → pub use crate::xkb::shared_ast_types::
  content = content.replace(/pub use self::xkbcomp_priv_h::/g, 'pub use crate::xkb::shared_ast_types::');

  // Step 4: Convert use super::xkbcomp_priv_h:: → use crate::xkb::shared_ast_types::
  content = content.replace(/use super::xkbcomp_priv_h::/g, 'use crate::xkb::shared_ast_types::');

  // Step 5: Convert qualified super::xkbcomp_priv_h:: → crate::xkb::shared_ast_types::
  content = content.replace(/super::xkbcomp_priv_h::/g, 'crate::xkb::shared_ast_types::');

  fs.writeFileSync(file, content);
  const removed = origLen - content.split('\n').length;
  totalRemoved += removed;
  console.log(`  ${shortName}: removed ${removed} lines${aliases ? ', replaced aliases: ' + Object.entries(aliases).map(([a,c]) => `${a}->${c}`).join(', ') : ''}`);
}

console.log(`\nTotal: ${files.length} files, ${totalRemoved} lines removed`);

// Now handle wrapper functions: include.rs, keymap_file_iterator.rs, xkbcomp.rs
// These had wrapper functions in xkbcomp_priv_h that are now removed.
// Callers need to use direct calls instead.
// Check: include.rs uses `use self::xkbcomp_priv_h::{FreeXkbFile, XkbParseFile}`
// After conversion, it will be `use crate::xkb::shared_ast_types::{FreeXkbFile, XkbParseFile}`
// But XkbParseFile is NOT in shared_ast_types! It was a wrapper function in the module.
// Need to replace XkbParseFile calls with direct scanner::XkbParseFile calls.
console.log('\nNow need to fix wrapper function calls (XkbParseFile, etc.)');
