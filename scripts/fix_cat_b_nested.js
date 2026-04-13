#!/usr/bin/env node
// Fix remaining compilation errors after Cat B removal:
// 1. Add `use libc::FILE;` inside pub mod blocks that reference FILE
// 2. Fix __compar_fn_t -> add `extern "C"` to fn pointer types for qsort
// 3. Fix vsnprintf -> add extern decl
// 4. Fix keysym type mismatches (XKB_KEYSYM constants)

const fs = require('fs');
const { execSync } = require('child_process');
const ROOT = '/home/rano/Public/wkb';

let errors;
try {
  errors = execSync('cd ' + ROOT + ' && cargo check 2>&1', { encoding: 'utf8', maxBuffer: 4*1024*1024 });
} catch(e) { errors = e.stdout || ''; }

const lines = errors.split('\n');

// Collect FILE errors by file:line
const fileErrors = new Map(); // file -> set of line numbers where FILE is missing
const callocErrors = new Map();
for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes('cannot find type `FILE`') || lines[i].includes('cannot find function `calloc`')) {
    const loc = (lines[i+1] || '').match(/--> (src\/[^:]+):(\d+)/);
    if (loc) {
      const sym = lines[i].includes('FILE') ? 'FILE' : 'calloc';
      const map = sym === 'FILE' ? fileErrors : callocErrors;
      if (!map.has(loc[1])) map.set(loc[1], new Set());
      map.get(loc[1]).add(parseInt(loc[2]));
    }
  }
}

// For each file with FILE errors, find the pub mod block containing the error line
// and add `use libc::FILE;` at the start of that block
for (const [file, errorLines] of [...fileErrors, ...callocErrors]) {
  const sym = fileErrors.has(file) && fileErrors.get(file) === errorLines ? 'FILE' : 'calloc';
  const filepath = ROOT + '/' + file;
  const content = fs.readFileSync(filepath, 'utf8');
  const fLines = content.split('\n');
  
  // Find all pub mod blocks and their ranges
  const mods = [];
  let j = 0;
  while (j < fLines.length) {
    const m = fLines[j].match(/^(\s*)pub\s+mod\s+(\w+)\s*\{/);
    if (m) {
      const start = j;
      const indent = m[1];
      let depth = 0;
      for (; j < fLines.length; j++) {
        for (const ch of fLines[j]) {
          if (ch === '{') depth++;
          if (ch === '}') depth--;
        }
        if (depth <= 0) { j++; break; }
      }
      mods.push({ name: m[2], start, end: j, indent });
    } else {
      j++;
    }
  }
  
  // For each error line, find which mod block it's in
  const modsNeedingImport = new Set();
  for (const errLine of errorLines) {
    const lineIdx = errLine - 1; // 0-indexed
    for (const mod of mods) {
      if (lineIdx >= mod.start && lineIdx < mod.end) {
        modsNeedingImport.add(mod);
        break;
      }
    }
  }
  
  // Insert use libc::FILE/calloc inside each mod block (after the opening brace)
  let offset = 0;
  const allSyms = new Set();
  if (fileErrors.has(file)) fileErrors.get(file).forEach(() => allSyms.add('FILE'));
  if (callocErrors.has(file)) callocErrors.get(file).forEach(() => allSyms.add('calloc'));
  
  for (const mod of [...modsNeedingImport].sort((a, b) => a.start - b.start)) {
    // Check what's needed in this mod
    const modContent = fLines.slice(mod.start + offset, mod.end + offset).join('\n');
    const needed = [];
    if (modContent.includes('FILE') && fileErrors.has(file)) needed.push('FILE');
    if (modContent.includes('calloc') && callocErrors.has(file)) needed.push('calloc');
    
    if (needed.length > 0) {
      const importLine = `${mod.indent}    use libc::{${needed.join(', ')}};`;
      fLines.splice(mod.start + offset + 1, 0, importLine);
      offset++;
    }
  }
  
  fs.writeFileSync(filepath, fLines.join('\n'));
  console.log(`Fixed ${file}: added libc imports in ${modsNeedingImport.size} mod blocks`);
}

// Fix vsnprintf: replace with extern decl in registry.rs
const regPath = ROOT + '/src/xkb/registry.rs';
const regContent = fs.readFileSync(regPath, 'utf8');
const newRegContent = regContent.replace(
  /use libc::\{([^}]*)\bvsnprintf\b([^}]*)\};/,
  (match, before, after) => {
    const cleaned = (before + after).replace(/,\s*,/g, ',').replace(/^\s*,/, '').replace(/,\s*$/, '');
    return `use libc::{${cleaned}};\nextern "C" { fn vsnprintf(s: *mut i8, maxlen: usize, format: *const i8, ...) -> i32; }`;
  }
);
if (newRegContent !== regContent) {
  fs.writeFileSync(regPath, newRegContent);
  console.log('Fixed registry.rs: replaced vsnprintf import with extern decl');
}
