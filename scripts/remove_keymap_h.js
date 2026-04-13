#!/usr/bin/env node
// Remove all pub mod keymap_h blocks from 18 files.
// Strategy:
// 1. Remove the pub mod keymap_h { ... } blocks
// 2. Replace `use super::keymap_h::*` / `use super::keymap_h::{...}` with `use crate::xkb::shared_types::*`
// 3. Replace qualified `super::keymap_h::Symbol` with `Symbol` (assuming shared_types::* is imported)
// 4. Replace `keymap_h::Symbol` refs in the same file

const fs = require('fs');
const path = require('path');
const root = '/home/rano/Public/wkb';

const files = [
  'src/xkb/xkbcomp/action.rs',
  'src/xkb/xkbcomp/ast_build.rs',
  'src/xkb/xkbcomp/compat.rs',
  'src/xkb/xkbcomp/keycodes.rs',
  'src/xkb/xkbcomp/keymap.rs',
  'src/xkb/xkbcomp/rules.rs',
  'src/xkb/xkbcomp/symbols.rs',
  'src/xkb/xkbcomp/types.rs',
  'src/xkb/xkbcomp/vmod.rs',
  'src/xkb/xkbcomp/xkbcomp.rs',
  'src/xkb/xkbcomp/expr.rs',
  'src/xkb/common.rs',
  'src/xkb/keymap.rs',
  'src/xkb/keymap_compare.rs',
  'src/xkb/keymap_priv.rs',
  'src/xkb/rmlvo.rs',
  'src/xkb/state.rs',
  'src/xkb/text.rs',
];

let totalRemoved = 0;

for (const f of files) {
  const fullPath = path.join(root, f);
  const content = fs.readFileSync(fullPath, 'utf8');
  const lines = content.split('\n');
  const newLines = [];
  let inBlock = false;
  let depth = 0;
  let removedCount = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // Detect start of keymap_h block
    if (!inBlock && /^\s*pub\s+mod\s+keymap_h\s*\{/.test(line)) {
      inBlock = true;
      depth = 0;
      // Count braces on this line
      for (const ch of line) {
        if (ch === '{') depth++;
        if (ch === '}') depth--;
      }
      removedCount++;
      if (depth === 0) inBlock = false;
      continue;
    }
    
    if (inBlock) {
      for (const ch of line) {
        if (ch === '{') depth++;
        if (ch === '}') depth--;
      }
      removedCount++;
      if (depth === 0) inBlock = false;
      continue;
    }
    
    newLines.push(line);
  }
  
  if (removedCount > 0) {
    // Now fix references to keymap_h
    let result = newLines.join('\n');
    
    // Replace `use super::keymap_h::*;` with nothing (shared_types::* should already be available)
    // But we need to check if the file already has shared_types imported
    const hasSharedTypes = /use\s+crate::xkb::shared_types::\*/.test(result);
    
    // Remove `use super::keymap_h::*;` lines
    result = result.replace(/^\s*use\s+super::keymap_h::\*;\s*\n/gm, '');
    
    // Replace `use super::keymap_h::{...};` - remove these lines, symbols come from shared_types
    result = result.replace(/^\s*use\s+super::keymap_h::\{[^}]*\};\s*\n/gm, '');
    
    // Replace `use super::keymap_h::Symbol;` single imports
    result = result.replace(/^\s*use\s+super::keymap_h::\w+;\s*\n/gm, '');
    
    // Replace qualified paths: super::keymap_h::Symbol -> Symbol
    result = result.replace(/super::keymap_h::(\w+)/g, '$1');
    
    // Replace self::keymap_h::Symbol -> Symbol  
    result = result.replace(/self::keymap_h::(\w+)/g, '$1');
    
    // Replace bare keymap_h::Symbol -> Symbol
    result = result.replace(/\bkeymap_h::(\w+)/g, '$1');
    
    fs.writeFileSync(fullPath, result);
    console.log(`${f}: removed ${removedCount} lines of keymap_h block`);
    totalRemoved += removedCount;
  }
}

console.log(`\nTotal: removed ${totalRemoved} lines from ${files.length} files`);
