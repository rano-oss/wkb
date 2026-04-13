#!/usr/bin/env node
// Category B: Remove stdlib_h, stdio_h, struct_FILE_h, FILE_h modules
// Step 1: Just remove the blocks and their use lines. Replace _IO_FILE references.
// Step 2: Compile errors will tell us what libc imports to add.

const fs = require('fs');
const { execSync } = require('child_process');

const ROOT = '/home/rano/Public/wkb';
const DEAD_MODS = new Set(['stdlib_h', 'stdio_h', 'struct_FILE_h', 'FILE_h']);

function processFile(filepath) {
  const content = fs.readFileSync(filepath, 'utf8');
  const lines = content.split('\n');
  const result = [];
  let i = 0;
  let changed = false;

  while (i < lines.length) {
    // Remove pub mod DEAD_MOD { ... }
    const modMatch = lines[i].match(/^\s*pub\s+mod\s+(\w+)\s*\{/);
    if (modMatch && DEAD_MODS.has(modMatch[1])) {
      let depth = 0;
      for (; i < lines.length; i++) {
        for (const ch of lines[i]) {
          if (ch === '{') depth++;
          if (ch === '}') depth--;
        }
        if (depth <= 0) { i++; break; }
      }
      changed = true;
      continue;
    }

    // Remove use (self|super)::DEAD_MOD::... (multi-line aware)
    let isDeadUse = false;
    for (const mod of DEAD_MODS) {
      if (new RegExp('^\\s*(pub\\s+)?use\\s+(self|super)::' + mod + '::').test(lines[i])) {
        isDeadUse = true;
        break;
      }
    }
    if (isDeadUse) {
      while (i < lines.length) {
        const hasEnd = lines[i].includes(';');
        i++;
        if (hasEnd) break;
      }
      changed = true;
      continue;
    }

    result.push(lines[i]);
    i++;
  }

  if (!changed) return false;

  // Clean up blank lines
  const cleaned = [];
  let blankCount = 0;
  for (const line of result) {
    if (line.trim() === '') {
      blankCount++;
      if (blankCount <= 2) cleaned.push(line);
    } else {
      blankCount = 0;
      cleaned.push(line);
    }
  }

  fs.writeFileSync(filepath, cleaned.join('\n'));
  return true;
}

const files = execSync(`find ${ROOT}/src -name "*.rs"`, {encoding: 'utf8'}).trim().split('\n');
let count = 0;
for (const f of files) {
  if (processFile(f)) count++;
}
console.log(`Total files modified: ${count}`);
