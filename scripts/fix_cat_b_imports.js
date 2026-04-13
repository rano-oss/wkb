#!/usr/bin/env node
// Fix Category B: parse cargo check errors, add libc imports

const fs = require('fs');
const { execSync } = require('child_process');

const ROOT = '/home/rano/Public/wkb';

// Get all errors (ignore exit code)
let errors;
try {
  errors = execSync('cd ' + ROOT + ' && cargo check 2>&1', { encoding: 'utf8', maxBuffer: 4*1024*1024 });
} catch(e) {
  errors = e.stdout || '';
}

// Parse: file -> set of missing symbols
const fileMissing = {};
const lines = errors.split('\n');
for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/error\[E0425\]: cannot find (?:function|value|type) `(\w+)` in this scope/);
  if (m) {
    const sym = m[1];
    // Next line has location
    const loc = (lines[i+1] || '').match(/--> (src\/[^:]+):/);
    if (loc) {
      const file = loc[1];
      if (!fileMissing[file]) fileMissing[file] = new Set();
      fileMissing[file].add(sym);
    }
  }
}

const LIBC_SYMS = new Set([
  'free', 'calloc', 'malloc', 'realloc', 'getenv', 'exit', 'abort',
  'qsort', 'strtol', 'atoi', 'atof', 'unsetenv', 'mkdtemp',
  'fclose', 'fopen', 'fflush', 'fread', 'fwrite', 'feof', 'ferror',
  'fileno', 'perror', 'setvbuf', 'vsnprintf', 'fdopen',
  'FILE', 'EXIT_SUCCESS', 'EXIT_FAILURE', '_IONBF', 'BUFSIZ',
]);

for (const [file, syms] of Object.entries(fileMissing)) {
  const filepath = ROOT + '/' + file;
  const content = fs.readFileSync(filepath, 'utf8');
  const fileLines = content.split('\n');

  const libcImports = [];
  const externFns = [];
  const externStatics = [];
  let needComparFnT = false;

  for (const sym of syms) {
    if (LIBC_SYMS.has(sym)) {
      libcImports.push(sym);
    } else if (sym === 'stderr' || sym === 'stdout' || sym === 'stdin') {
      externStatics.push(sym);
    } else if (sym === 'secure_getenv') {
      externFns.push('fn secure_getenv(name: *const i8) -> *mut i8;');
    } else if (sym === '__compar_fn_t') {
      needComparFnT = true;
    } else {
      console.error(`WARNING: Unknown symbol ${sym} in ${file}`);
    }
  }

  const newLines = [];
  if (libcImports.length > 0) {
    libcImports.sort();
    newLines.push(`use libc::{${libcImports.join(', ')}};`);
  }
  if (externStatics.length > 0 || externFns.length > 0) {
    newLines.push('extern "C" {');
    for (const s of externStatics) {
      newLines.push(`    pub static ${s}: *mut libc::FILE;`);
    }
    for (const f of externFns) {
      newLines.push(`    pub ${f}`);
    }
    newLines.push('}');
  }
  if (needComparFnT) {
    newLines.push('type __compar_fn_t = Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> i32>;');
  }

  if (newLines.length === 0) continue;

  // Find insertion point: after last top-level use/pub use line
  let insertIdx = 0;
  let depth = 0;
  for (let j = 0; j < fileLines.length; j++) {
    const t = fileLines[j].trim();
    for (const ch of fileLines[j]) {
      if (ch === '{') depth++;
      if (ch === '}') depth--;
    }
    if (depth === 0 && (t.startsWith('use ') || t.startsWith('pub use '))) {
      insertIdx = j + 1;
    }
    if (depth === 0 && (t.startsWith('pub unsafe fn') || t.startsWith('pub fn') || t.startsWith('unsafe fn') || t.startsWith('fn ') || t.startsWith('#[test]'))) {
      break;
    }
  }

  fileLines.splice(insertIdx, 0, ...newLines);
  fs.writeFileSync(filepath, fileLines.join('\n'));
  console.log(`${file}: +libc[${libcImports.join(',')}] +extern[${externStatics.concat(externFns.map(f => 'fn')).join(',')}]`);
}
