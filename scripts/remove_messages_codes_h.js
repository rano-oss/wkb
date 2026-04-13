#!/usr/bin/env node
// Remove all pub mod messages_codes_h blocks from 24 files.
// Convert `pub use self::messages_codes_h::{...}` to `pub use crate::xkb::messages::{...}`

const fs = require('fs');
const path = require('path');
const root = '/home/rano/Public/wkb';
const { execSync } = require('child_process');

const files = execSync("grep -rl 'pub mod messages_codes_h' src/", { cwd: root, encoding: 'utf8' }).trim().split('\n');

let totalRemoved = 0;

for (const f of files) {
  // Skip messages.rs - already rewritten
  if (f === 'src/xkb/messages.rs') continue;
  
  const fullPath = path.join(root, f);
  const content = fs.readFileSync(fullPath, 'utf8');
  const lines = content.split('\n');
  const newLines = [];
  let inBlock = false;
  let depth = 0;
  let removedCount = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    if (!inBlock && /^\s*pub\s+mod\s+messages_codes_h\s*\{/.test(line)) {
      inBlock = true;
      depth = 0;
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
    let result = newLines.join('\n');
    
    // Fix multi-line pub use self::messages_codes_h::{...} -> pub use crate::xkb::messages::{...}
    result = result.replace(/pub\s+use\s+self::messages_codes_h::/g, 'pub use crate::xkb::messages::');
    
    // Fix use super::messages_codes_h::
    result = result.replace(/use\s+super::messages_codes_h::/g, 'use crate::xkb::messages::');
    
    // Fix bare messages_codes_h:: references
    result = result.replace(/\bmessages_codes_h::(\w+)/g, 'crate::xkb::messages::$1');
    
    fs.writeFileSync(fullPath, result);
    console.log(`${f}: removed ${removedCount} lines`);
    totalRemoved += removedCount;
  }
}

console.log(`\nTotal: removed ${totalRemoved} lines from ${files.length - 1} files`);
