#!/usr/bin/env node
// Remove orphaned continuation lines left by flattening script
const fs = require('fs');
const root = '/home/rano/Public/wkb';

// Each entry: [file, startLine (1-indexed), endLine (1-indexed inclusive)]
// These are orphaned continuation lines from removed `pub use self::xxx_h::` statements
const orphans = [
  ['src/xkb/keymap_priv.rs', 89, 91],
  ['src/xkb/registry.rs', 866, 994],  // big range, handle carefully
  ['src/xkb/state.rs', 198, 200],
];

for (const [relFile, startHint] of orphans) {
  const filePath = `${root}/${relFile}`;
  const content = fs.readFileSync(filePath, 'utf8');
  const lines = content.split('\n');
  const result = [];
  let i = 0;
  let removed = 0;
  
  while (i < lines.length) {
    const line = lines[i].trim();
    
    // Check if this is an orphaned continuation line
    if (i > 0 && 
        (line.match(/^[A-Z_a-z][A-Za-z0-9_:, ]*[,}]$/) || line.match(/^};$/)) &&
        !line.startsWith('pub ') && !line.startsWith('use ') && !line.startsWith('fn ') &&
        !line.startsWith('let ') && !line.startsWith('//') && !line.startsWith('#') &&
        !line.startsWith('extern') && !line.startsWith('static') && !line.startsWith('const') &&
        !line.startsWith('type ') && !line.startsWith('unsafe') && !line.startsWith('if ') &&
        !line.startsWith('return') && !line.startsWith('impl ') && !line.startsWith('match ') &&
        !line.startsWith('while ') && !line.startsWith('for ')
    ) {
      // Check if previous non-empty line ends with `;` or `};`
      let prevIdx = result.length - 1;
      while (prevIdx >= 0 && result[prevIdx].trim() === '') prevIdx--;
      if (prevIdx >= 0) {
        const prevLine = result[prevIdx].trim();
        if (prevLine.endsWith(';') || prevLine.endsWith('};')) {
          // This is an orphan - skip until we find `;`
          let start = i;
          while (i < lines.length && !lines[i].includes(';')) {
            i++;
          }
          if (i < lines.length) i++; // skip the `;` line too
          removed += (i - start);
          continue;
        }
      }
    }
    
    result.push(lines[i]);
    i++;
  }
  
  if (removed > 0) {
    fs.writeFileSync(filePath, result.join('\n'));
    console.log(`${relFile}: removed ${removed} orphaned lines`);
  }
}
