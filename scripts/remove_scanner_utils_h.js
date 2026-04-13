#!/usr/bin/env node
// Remove scanner_utils_h blocks from 6 files (scanner_utils.rs already rewritten)
// and update imports to point to crate::xkb::scanner_utils::
const fs = require('fs');
const path = require('path');

const root = '/home/rano/Public/wkb';
const MOD_NAME = 'scanner_utils_h';
const CANONICAL = 'crate::xkb::scanner_utils';

const files = [
  'src/xkb/xkbcomp/ast_build.rs',
  'src/xkb/xkbcomp/include.rs',
  'src/xkb/xkbcomp/keymap_file_iterator.rs',
  'src/xkb/xkbcomp/parser.rs',
  'src/xkb/xkbcomp/rules.rs',
  'src/xkb/xkbcomp/scanner.rs',
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
  let inPubUse = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Remove pub mod scanner_utils_h { ... } block
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

    // Handle pub use self::scanner_utils_h:: (possibly multi-line)
    if (line.match(new RegExp(`pub use self::${MOD_NAME}::`))) {
      let converted = line.replace(`self::${MOD_NAME}`, CANONICAL);
      result.push(converted);
      if (!line.includes(';')) {
        inPubUse = true;
      }
      continue;
    }

    // Handle use super::scanner_utils_h:: (possibly multi-line)
    if (line.match(new RegExp(`use super::${MOD_NAME}::`))) {
      let converted = line.replace(`super::${MOD_NAME}`, CANONICAL);
      result.push(converted);
      if (!line.includes(';')) {
        inPubUse = true;
      }
      continue;
    }

    // Continue multi-line use statement
    if (inPubUse) {
      result.push(line);
      if (line.includes(';')) {
        inPubUse = false;
      }
      continue;
    }

    result.push(line);
  }

  let newContent = result.join('\n');
  
  // Replace crate::xkb::scanner_utils::scanner_utils_h:: with crate::xkb::scanner_utils::
  newContent = newContent.replace(/crate::xkb::scanner_utils::scanner_utils_h::/g, 'crate::xkb::scanner_utils::');
  
  const newLen = newContent.split('\n').length;
  const removed = origLen - newLen;
  totalRemoved += removed;
  
  if (removed > 0 || content !== newContent) {
    fs.writeFileSync(filePath, newContent);
    console.log(`${relFile}: ${removed} lines removed, imports updated`);
  } else {
    console.log(`${relFile}: no changes`);
  }
}

console.log(`\nTotal: ${totalRemoved} lines removed from ${files.length} files`);
