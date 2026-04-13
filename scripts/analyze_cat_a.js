#!/usr/bin/env node
const fs = require('fs');
const { execSync } = require('child_process');
const path = require('path');

const ROOT = '/home/rano/Public/wkb';
const files = execSync(`find ${ROOT}/src -name "*.rs"`, {encoding: 'utf8'}).trim().split('\n');

const catA = ['stdbool_h', '__stddef_null_h', 'types_h', 'stdint_uintn_h', 'stdint_intn_h', 'stdint_h', 'config_h', 'string_h'];

function extractModBlocks(lines, modName) {
  const results = [];
  const escaped = modName.replace(/([^a-zA-Z0-9_])/g, '\\$1');
  const re = new RegExp('^\\s*pub\\s+mod\\s+' + escaped + '\\s*\\{');
  let i = 0;
  while (i < lines.length) {
    if (re.test(lines[i])) {
      let depth = 0;
      const start = i;
      const bodyLines = [];
      for (; i < lines.length; i++) {
        bodyLines.push(lines[i]);
        for (const ch of lines[i]) {
          if (ch === '{') depth++;
          if (ch === '}') depth--;
        }
        if (depth <= 0) { i++; break; }
      }
      results.push({ start, end: i, body: bodyLines });
    } else {
      i++;
    }
  }
  return results;
}

for (const mod of catA) {
  const allBlocks = [];
  for (const f of files) {
    try {
      const content = fs.readFileSync(f, 'utf8');
      const lines = content.split('\n');
      const blocks = extractModBlocks(lines, mod);
      for (const b of blocks) {
        allBlocks.push({ file: f.replace(ROOT + '/', ''), ...b });
      }
    } catch(e) {}
  }
  
  console.log(`\n=== ${mod} (${allBlocks.length} instances) ===`);
  
  // Group by normalized body
  const groups = new Map();
  for (const b of allBlocks) {
    const key = b.body.map(l => l.trim()).filter(l => l).join(' ');
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key).push(b);
  }
  
  let v = 0;
  for (const [_, blocks] of groups) {
    v++;
    const repr = blocks[0];
    console.log(`  Variant ${v} (${blocks.length} files, ${repr.body.length} lines):`);
    for (const l of repr.body) {
      console.log(`    ${l}`);
    }
    console.log(`  Files: ${blocks.map(b => path.basename(b.file)).join(', ')}`);
  }
}
