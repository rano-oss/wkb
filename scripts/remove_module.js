#!/usr/bin/env node
// Generic module block removal script
// Usage: node remove_module.js <module_name> <replacement_path>
// Removes pub mod <module_name> { ... } blocks and converts references to replacement_path

const fs = require('fs');
const { execSync } = require('child_process');

const modName = process.argv[2];
if (!modName) { console.error('Usage: node remove_module.js <module_name>'); process.exit(1); }

const ROOT = '/home/rano/Public/wkb/src/xkb';

let files;
try {
  files = execSync(`grep -rln 'pub mod ${modName}' ${ROOT}/`, { encoding: 'utf8' }).trim().split('\n');
} catch { console.log(`No files with pub mod ${modName}`); process.exit(0); }

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

  // Remove any pub use self::<modName>::{...} (may be multi-line)
  // Replace with nothing — the block's pub use already pointed to canonical locations
  const outLines = [];
  let skipping = false;
  for (const line of content.split('\n')) {
    if (line.match(new RegExp(`pub use self::${modName}::`))) {
      skipping = true;
    }
    if (skipping) {
      if (line.includes(';')) { skipping = false; }
      continue; // skip this line
    }
    outLines.push(line);
  }
  content = outLines.join('\n');

  // Replace use self::<modName>:: → (remove, these are private imports from removed module)
  // Actually: use self::<modName>::X needs to become use <canonical>::X
  // But we don't know the canonical path generically. Just remove the line and let compiler guide us.
  // Actually for re-export modules: the block content already had `pub use crate::xkb::foo::bar`
  // so the file-level `pub use self::<modName>::{A, B}` was making A, B available.
  // After block removal, A, B are no longer available. But they ARE available via the canonical path.
  // We've already removed pub use self:: lines above.
  
  // Also handle use super::<modName>::
  content = content.replace(new RegExp(`use super::${modName}::`, 'g'), 'use_FIXME::');

  fs.writeFileSync(file, content);
  const removed = origLen - content.split('\n').length;
  totalRemoved += removed;
  const shortName = file.replace('/home/rano/Public/wkb/', '');
  console.log(`  ${shortName}: removed ${removed} lines`);
}
console.log(`\nTotal: ${files.length} files, ${totalRemoved} lines removed`);
