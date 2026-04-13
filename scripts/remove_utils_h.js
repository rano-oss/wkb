#!/usr/bin/env node
// Remove all pub mod utils_h blocks from 24 files.
// Then convert `pub use self::utils_h::{...}` to `pub use crate::xkb::utils::{...}`

const fs = require('fs');
const path = require('path');
const root = '/home/rano/Public/wkb';
const { execSync } = require('child_process');

const files = execSync("grep -rl 'pub mod utils_h' src/", { cwd: root, encoding: 'utf8' }).trim().split('\n');

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
    
    if (!inBlock && /^\s*pub\s+mod\s+utils_h\s*\{/.test(line)) {
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
    
    // Fix multi-line pub use self::utils_h::{...} -> pub use crate::xkb::utils::{...}
    // Handle both single-line and multi-line forms
    result = result.replace(/pub\s+use\s+self::utils_h::/g, 'pub use crate::xkb::utils::');
    
    // Also fix any remaining `use super::utils_h::` references
    result = result.replace(/use\s+super::utils_h::/g, 'use crate::xkb::utils::');
    
    // Fix bare `utils_h::` references  
    result = result.replace(/\butils_h::(\w+)/g, 'crate::xkb::utils::$1');
    
    fs.writeFileSync(fullPath, result);
    console.log(`${f}: removed ${removedCount} lines of utils_h block`);
    totalRemoved += removedCount;
  }
}

console.log(`\nTotal: removed ${totalRemoved} lines from ${files.length} files`);
