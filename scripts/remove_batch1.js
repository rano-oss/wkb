#!/usr/bin/env node
// Batch removal of simple re-export modules: text_h, expr_h, limits_h, keysym_h, errno_h, unistd_h, util_mem_h
// These are all pure re-exports, constants, or simple functions that can be moved to canonical locations.

const fs = require('fs');
const { execSync } = require('child_process');
const ROOT = '/home/rano/Public/wkb/src/xkb';

function removeModuleBlocks(modName) {
  let files;
  try {
    files = execSync(`grep -rln 'pub mod ${modName}' ${ROOT}/`, { encoding: 'utf8' }).trim().split('\n');
  } catch { return 0; }
  
  let totalRemoved = 0;
  for (const file of files) {
    let content = fs.readFileSync(file, 'utf8');
    const origLen = content.split('\n').length;
    
    // Remove pub mod block
    const lines = content.split('\n');
    const newLines = [];
    let inBlock = false, depth = 0;
    for (const line of lines) {
      if (!inBlock && line.match(new RegExp(`^pub mod ${modName}\\s*\\{`))) { inBlock = true; depth = 0; }
      if (inBlock) {
        for (const ch of line) { if (ch === '{') depth++; if (ch === '}') depth--; }
        if (depth === 0) { inBlock = false; continue; }
        continue;
      }
      newLines.push(line);
    }
    content = newLines.join('\n');
    
    // Remove pub use self::<modName>::{...} lines (multi-line aware)
    const outLines = [];
    let skipping = false;
    for (const line of content.split('\n')) {
      if (!skipping && line.match(new RegExp(`pub use self::${modName}::`))) {
        skipping = true;
      }
      if (!skipping && line.match(new RegExp(`use self::${modName}::`))) {
        skipping = true;
      }
      if (skipping) {
        if (line.includes(';')) { skipping = false; }
        continue;
      }
      outLines.push(line);
    }
    content = outLines.join('\n');
    
    // Handle super:: refs — we'll fix manually after
    // Just flag them for now by not touching them
    
    fs.writeFileSync(file, content);
    const removed = origLen - content.split('\n').length;
    totalRemoved += removed;
  }
  console.log(`  ${modName}: ${files.length} files, ${totalRemoved} lines removed`);
  return totalRemoved;
}

// === limits_h: just CHAR_BIT = 8 constant. Replace usages with literal 8 ===
// Actually, CHAR_BIT is used as `limits_h::CHAR_BIT` or imported. Let's just replace usages.
{
  let files;
  try {
    files = execSync(`grep -rln 'CHAR_BIT' ${ROOT}/`, { encoding: 'utf8' }).trim().split('\n');
  } catch { files = []; }
  
  for (const file of files) {
    let content = fs.readFileSync(file, 'utf8');
    // Replace CHAR_BIT usage (not definitions)
    // Pattern: limits_h::CHAR_BIT or standalone CHAR_BIT in expressions
    content = content.replace(/limits_h::CHAR_BIT/g, '8');
    // Remove `use super::limits_h::CHAR_BIT;` or similar
    content = content.replace(/use super::limits_h::\w+;\n/g, '');
    fs.writeFileSync(file, content);
  }
}
let total = 0;
total += removeModuleBlocks('limits_h');

// === text_h: pure pub use re-exports from crate::xkb::text ===
// Also has some unique content in keymap.rs (format_control_names_offset function and constants)
total += removeModuleBlocks('text_h');

// === expr_h: pure pub use re-exports from crate::xkb::xkbcomp::expr ===
total += removeModuleBlocks('expr_h');

console.log(`\nGrand total: ${total} lines removed`);
console.log('\nRun cargo check to find missing imports...');
