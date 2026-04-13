#!/usr/bin/env node
// Fix missing imports after text_h, expr_h, limits_h block removal
const fs = require('fs');
const ROOT = '/home/rano/Public/wkb/src/xkb';

// What each file's text_h block was re-exporting:
const textImports = {
  'xkbcomp/action.rs': 'use crate::xkb::text::{actionTypeNames, ctrlMaskNames, ActionTypeText, KeyNameText, LookupEntry, LookupString, LookupValue};',
  'xkbcomp/compat.rs': 'use crate::xkb::text::{ctrlMaskNames, groupComponentMaskNames, modComponentMaskNames, symInterpretMatchMaskNames, useModMapValueNames, KeysymText, LookupEntry, LookupString, ModMaskText, SIMatchText};',
  'xkbcomp/keycodes.rs': 'use crate::xkb::text::{KeyNameText, LookupEntry};',
  'xkbcomp/keymap.rs': null, // special — had function + constants
  'xkbcomp/symbols.rs': 'use crate::xkb::text::{ActionTypeText, KeyNameText, KeysymText, LookupEntry, ModIndexText};',
  'xkbcomp/types.rs': 'use crate::xkb::text::{LookupEntry, ModMaskText};',
  'xkbcomp/vmod.rs': 'use crate::xkb::text::ModMaskText;',
  'xkbcomp/expr.rs': 'use crate::xkb::text::{LookupEntry, buttonNames};',
  'xkbcomp/scanner.rs': 'use crate::xkb::text::LookupEntry;',
};

// What each file's expr_h block was re-exporting:
const exprImports = {
  'xkbcomp/compat.rs': 'use crate::xkb::xkbcomp::expr::{ExprResolveBoolean, ExprResolveEnum, ExprResolveGroupMask, ExprResolveLhs, ExprResolveMask, ExprResolveMod, ExprResolveModMask};',
  'xkbcomp/keycodes.rs': 'use crate::xkb::xkbcomp::expr::{ExprResolveInteger, ExprResolveLhs, ExprResolveString};',
  'xkbcomp/keymap.rs': 'use crate::xkb::xkbcomp::expr::{ExprResolveGroup, ExprResolveGroupMask};',
  'xkbcomp/symbols.rs': 'use crate::xkb::xkbcomp::expr::{ExprResolveBoolean, ExprResolveEnum, ExprResolveGroup, ExprResolveLhs, ExprResolveModMask, ExprResolveString};',
  'xkbcomp/types.rs': 'use crate::xkb::xkbcomp::expr::{ExprResolveLevel, ExprResolveLhs, ExprResolveModMask, ExprResolveString};',
  'xkbcomp/vmod.rs': 'use crate::xkb::xkbcomp::expr::ExprResolveModMask;',
};

// CHAR_BIT imports needed
const charBitImports = {
  'xkbcomp/symbols.rs': true,
};

// Add imports to files
for (const [relPath, importLine] of Object.entries(textImports)) {
  if (!importLine) continue;
  const filePath = `${ROOT}/${relPath}`;
  let content = fs.readFileSync(filePath, 'utf8');
  if (!content.includes(importLine.substring(0, 30))) {
    // Add after last use statement at top level (first non-use, non-pub-use, non-empty, non-comment line)
    const lines = content.split('\n');
    let insertIdx = 0;
    for (let i = 0; i < lines.length; i++) {
      const l = lines[i].trim();
      if (l.startsWith('use ') || l.startsWith('pub use ') || l === '' || l.startsWith('//') || l.startsWith('#')) {
        insertIdx = i + 1;
      } else if (l.startsWith('pub mod ') || l.startsWith('pub ') || l.startsWith('unsafe ') || l.startsWith('fn ') || l.startsWith('static ') || l.startsWith('const ')) {
        break;
      }
    }
    lines.splice(insertIdx, 0, importLine);
    fs.writeFileSync(filePath, lines.join('\n'));
    console.log(`Added text imports to ${relPath}`);
  }
}

for (const [relPath, importLine] of Object.entries(exprImports)) {
  const filePath = `${ROOT}/${relPath}`;
  let content = fs.readFileSync(filePath, 'utf8');
  if (!content.includes(importLine.substring(0, 30))) {
    const lines = content.split('\n');
    let insertIdx = 0;
    for (let i = 0; i < lines.length; i++) {
      const l = lines[i].trim();
      if (l.startsWith('use ') || l.startsWith('pub use ') || l === '' || l.startsWith('//') || l.startsWith('#')) {
        insertIdx = i + 1;
      } else if (l.startsWith('pub mod ') || l.startsWith('pub ') || l.startsWith('unsafe ') || l.startsWith('fn ') || l.startsWith('static ') || l.startsWith('const ')) {
        break;
      }
    }
    lines.splice(insertIdx, 0, importLine);
    fs.writeFileSync(filePath, lines.join('\n'));
    console.log(`Added expr imports to ${relPath}`);
  }
}

// CHAR_BIT
for (const [relPath] of Object.entries(charBitImports)) {
  const filePath = `${ROOT}/${relPath}`;
  let content = fs.readFileSync(filePath, 'utf8');
  const importLine = 'use crate::xkb::shared_types::CHAR_BIT;';
  if (!content.includes(importLine)) {
    const lines = content.split('\n');
    let insertIdx = 0;
    for (let i = 0; i < lines.length; i++) {
      const l = lines[i].trim();
      if (l.startsWith('use ') || l.startsWith('pub use ') || l === '' || l.startsWith('//') || l.startsWith('#')) {
        insertIdx = i + 1;
      } else {
        break;
      }
    }
    lines.splice(insertIdx, 0, importLine);
    fs.writeFileSync(filePath, lines.join('\n'));
    console.log(`Added CHAR_BIT import to ${relPath}`);
  }
}

console.log('Done. Now need to handle keymap.rs text_h unique content manually.');
