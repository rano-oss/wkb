#!/usr/bin/env node
// Remove utils_numbers_h blocks from 9 files and update imports
const fs = require('fs');
const path = require('path');

const root = '/home/rano/Public/wkb';
const MOD_NAME = 'utils_numbers_h';
const CANONICAL = 'crate::xkb::utils';

const files = [
  'src/xkb/common.rs',
  'src/xkb/custom_parsers.rs',
  'src/xkb/keymap_formats.rs',
  'src/xkb/keysym.rs',
  'src/xkb/state.rs',
  'src/xkb/xkbcomp/expr.rs',
  'src/xkb/xkbcomp/rules.rs',
  'src/xkb/xkbcomp/scanner.rs',
  'src/xkb/xkbcomp/symbols.rs',
];

let totalRemoved = 0;

for (const relFile of files) {
  const filePath = path.join(root, relFile);
  let content = fs.readFileSync(filePath, 'utf8');
  const origLen = content.split('\n').length;
  let lines = content.split('\n');
  let result = [];
  let inBlock = false;
  let depth = 0;
  let inPubUse = false; // tracking multi-line pub use self::utils_numbers_h

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Remove pub mod utils_numbers_h { ... } block
    if (!inBlock && line.match(new RegExp(`^pub mod ${MOD_NAME}\\s*\\{`))) {
      inBlock = true;
      depth = 1;
      continue;
    }
    if (inBlock) {
      for (const ch of line) {
        if (ch === '{') depth++;
        if (ch === '}') depth--;
      }
      if (depth <= 0) {
        inBlock = false;
      }
      continue;
    }

    // Handle pub use self::utils_numbers_h:: (possibly multi-line)
    if (line.match(new RegExp(`pub use self::${MOD_NAME}::`))) {
      // Convert to canonical import
      let converted = line.replace(`self::${MOD_NAME}`, CANONICAL);
      result.push(converted);
      // Check if multi-line (no semicolon yet)
      if (!line.includes(';')) {
        inPubUse = true;
      }
      continue;
    }

    // Handle use super::utils_numbers_h:: (possibly multi-line)
    if (line.match(new RegExp(`use super::${MOD_NAME}::`))) {
      let converted = line.replace(`super::${MOD_NAME}`, CANONICAL);
      result.push(converted);
      if (!line.includes(';')) {
        inPubUse = true;
      }
      continue;
    }

    // Continue multi-line use statement (just pass through until semicolon)
    if (inPubUse) {
      result.push(line);
      if (line.includes(';')) {
        inPubUse = false;
      }
      continue;
    }

    result.push(line);
  }

  const newContent = result.join('\n');
  const newLen = newContent.split('\n').length;
  const removed = origLen - newLen;
  totalRemoved += removed;
  
  if (removed > 0 || content !== newContent) {
    fs.writeFileSync(filePath, newContent);
    console.log(`${relFile}: ${removed} lines removed`);
  } else {
    console.log(`${relFile}: no changes`);
  }
}

console.log(`\nTotal: ${totalRemoved} lines removed from ${files.length} files`);
