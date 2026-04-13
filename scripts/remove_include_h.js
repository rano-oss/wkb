#!/usr/bin/env node
// Remove include_h blocks from 7 files and update imports
const fs = require('fs');
const path = require('path');

const root = '/home/rano/Public/wkb';
const MOD_NAME = 'include_h';
const CANONICAL = 'crate::xkb::xkbcomp::include';

const files = [
  'src/xkb/xkbcomp/ast_build.rs',
  'src/xkb/xkbcomp/compat.rs',
  'src/xkb/xkbcomp/keycodes.rs',
  'src/xkb/xkbcomp/keymap_file_iterator.rs',
  'src/xkb/xkbcomp/rules.rs',
  'src/xkb/xkbcomp/symbols.rs',
  'src/xkb/xkbcomp/types.rs',
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

    // Remove pub mod include_h { ... } block (improved brace counting to skip strings)
    if (!inBlock && line.match(new RegExp(`^pub mod ${MOD_NAME}\\s*\\{`))) {
      inBlock = true;
      depth = 1;
      continue;
    }
    if (inBlock) {
      let inStr = false;
      for (let j = 0; j < line.length; j++) {
        if (line[j] === '"' && (j === 0 || line[j-1] !== '\\')) inStr = !inStr;
        if (!inStr) {
          if (line[j] === '{') depth++;
          if (line[j] === '}') depth--;
        }
      }
      if (depth <= 0) {
        inBlock = false;
      }
      continue;
    }

    // Handle pub use self::include_h:: (possibly multi-line)
    if (line.match(new RegExp(`pub use self::${MOD_NAME}::`))) {
      let converted = line.replace(`self::${MOD_NAME}`, CANONICAL);
      result.push(converted);
      if (!line.includes(';')) {
        inPubUse = true;
      }
      continue;
    }

    // Handle use super::include_h:: (possibly multi-line) 
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
