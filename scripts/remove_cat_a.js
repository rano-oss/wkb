#!/usr/bin/env node
// Category A removal — attempt 3
// 
// Algorithm:
// Phase 1: Remove dead module blocks + their use imports
// Phase 2: Replace qualified refs (super::dead_mod::SYM)
// Phase 3: Replace bare symbols in expressions (not in definitions, not in use statements)
// Phase 4: Remove recursive type aliases
// Phase 5: Remove empty modules that resulted from removing inner use statements

const fs = require('fs');
const { execSync } = require('child_process');

const ROOT = '/home/rano/Public/wkb';

const DEAD_MODS = new Set([
  'stdbool_h', '__stddef_null_h', 'types_h', 'stdint_uintn_h',
  'stdint_intn_h', 'stdint_h', 'config_h', 'string_h'
]);

const SYM_MAP = {
  'false_0': '0', 'true_0': '1',
  '__int8_t': 'i8', '__uint8_t': 'u8',
  '__int16_t': 'i16', '__uint16_t': 'u16',
  '__int32_t': 'i32', '__uint32_t': 'u32',
  '__int64_t': 'i64', '__uint64_t': 'u64',
  '__mode_t': 'u32', '__off64_t': 'i64', '__off_t': 'i64',
  '__nlink_t': 'u64', '__syscall_slong_t': 'i64', '__time_t': 'i64',
  '__uid_t': 'u32', '__gid_t': 'u32', '__dev_t': 'u64', '__ino_t': 'u64',
  '__ino64_t': 'u64', '__blkcnt_t': 'i64', '__blksize_t': 'i64',
  'ssize_t': 'isize',
  'uint8_t': 'u8', 'uint16_t': 'u16', 'uint64_t': 'u64',
  'UINT32_MAX': 'u32::MAX', 'UINT16_MAX': 'u16::MAX', 'UINT8_MAX': 'u8::MAX',
  'INT32_MAX': 'i32::MAX', 'INT64_MAX': 'i64::MAX',
  'INT8_MIN': 'i8::MIN', 'INT16_MIN': 'i16::MIN',
  'INT8_MAX': 'i8::MAX', 'INT16_MAX': 'i16::MAX',
  'SIZE_MAX': 'usize::MAX', 'UINT32_WIDTH': '32',
  'intmax_t': 'i64', 'uintptr_t': 'usize',
  'NULL': 'std::ptr::null_mut::<core::ffi::c_void>()',
  'NULL_0': 'std::ptr::null_mut::<core::ffi::c_void>()',
  'EXIT_INVALID_USAGE': '2',
};

const RUST_PRIMITIVES = new Set(['i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64', 'usize', 'isize']);

// Symbols that should only be replaced when qualified (not bare) — these are
// symbols whose names are Rust primitive types when mapped.
// e.g., don't replace bare `uint8_t` since it could be a re-export name in a non-dead module
const QUALIFIED_ONLY = new Set(); // We'll handle this differently

function isDefiningLine(line, sym) {
  // Returns true if this line is defining `sym` (so we should NOT replace `sym` on this line)
  // e.g., `pub const UINT16_MAX: i32 = 65535;`  — sym=UINT16_MAX — don't replace LHS
  // e.g., `pub type __uint32_t = u32;`  — sym=__uint32_t — don't replace LHS
  const trimmed = line.trim();
  if (trimmed.match(new RegExp('^(pub\\s+)?(const|static)\\s+' + sym.replace(/([^a-zA-Z0-9_])/g, '\\$1') + '\\s*[=:]'))) return true;
  if (trimmed.match(new RegExp('^(pub\\s+)?type\\s+' + sym.replace(/([^a-zA-Z0-9_])/g, '\\$1') + '\\s*='))) return true;
  return false;
}

function processFile(filepath) {
  const content = fs.readFileSync(filepath, 'utf8');
  const lines = content.split('\n');
  const result = [];
  let i = 0;
  let changed = false;

  while (i < lines.length) {
    // Phase 1: Remove pub mod DEAD_MOD { ... } blocks
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

    // Phase 1b: Remove use lines importing from dead modules
    // Match various patterns:
    //   use super::types_h::__uint32_t;
    //   use super::types_h::{__int8_t, __int16_t};
    //   pub use self::stdbool_h::true_0;
    const useDeadRe = /^\s*(pub\s+)?use\s+(self|super)::(\w+)::/;
    const useDeadMatch = lines[i].match(useDeadRe);
    if (useDeadMatch && DEAD_MODS.has(useDeadMatch[3])) {
      while (i < lines.length) {
        const hasEnd = lines[i].includes(';');
        i++;
        if (hasEnd) break;
      }
      changed = true;
      continue;
    }

    let line = lines[i];
    const isUseLine = /^\s*(pub\s+)?use\s+/.test(line);

    // Phase 2: Replace qualified refs: super::dead_mod::SYM or self::dead_mod::SYM
    for (const mod of DEAD_MODS) {
      const escaped = mod.replace(/([^a-zA-Z0-9_])/g, '\\$1');
      const qualRe = new RegExp('(?:self|super)::' + escaped + '::(\\w+)', 'g');
      const newLine = line.replace(qualRe, (match, sym) => {
        if (SYM_MAP[sym] !== undefined) return SYM_MAP[sym];
        if (RUST_PRIMITIVES.has(sym)) return sym;
        console.error(`WARNING: Unknown qualified ${mod}::${sym} in ${filepath}:${i+1}`);
        return match;
      });
      if (newLine !== line) { line = newLine; changed = true; }
    }

    // Phase 3: Bare symbol replacement — SKIP use/pub use lines entirely
    if (!isUseLine) {
      for (const [sym, repl] of Object.entries(SYM_MAP)) {
        // Skip primitives as bare replacements (they're identities)
        if (RUST_PRIMITIVES.has(sym)) continue;
        // Skip if this line is DEFINING the symbol (don't replace LHS of const/type)
        if (isDefiningLine(line, sym)) continue;

        const escaped = sym.replace(/([^a-zA-Z0-9_])/g, '\\$1');
        const symRe = new RegExp('\\b' + escaped + '\\b', 'g');
        const newLine = line.replace(symRe, (match, offset) => {
          // Don't replace inside string literals
          const before = line.substring(0, offset);
          const dquotes = (before.match(/"/g) || []).length;
          if (dquotes % 2 !== 0) return match;
          // Don't replace inside b"..." byte strings
          const bquotes = (before.match(/b"/g) || []).length;
          // This is rough but good enough since byte strings use b"..."
          return repl;
        });
        if (newLine !== line) { line = newLine; changed = true; }
      }
    }

    // Phase 4: Remove recursive type aliases: pub type X = X;
    const typeAlias = line.match(/^\s*pub\s+type\s+(\w+)\s*=\s*(\w+)\s*;/);
    if (typeAlias && typeAlias[1] === typeAlias[2]) {
      i++;
      changed = true;
      continue;
    }

    result.push(line);
    i++;
  }

  // Clean up consecutive blank lines
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

  if (changed) {
    fs.writeFileSync(filepath, cleaned.join('\n'));
    return true;
  }
  return false;
}

const files = execSync(`find ${ROOT}/src -name "*.rs"`, {encoding: 'utf8'}).trim().split('\n');
let changedCount = 0;
for (const f of files) {
  if (processFile(f)) {
    changedCount++;
    console.log(`Modified: ${f.replace(ROOT + '/', '')}`);
  }
}
console.log(`\nTotal files modified: ${changedCount}`);
