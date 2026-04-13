#!/usr/bin/env node
// Remove pub mod blocks for batch 2:
// struct_timespec_h, struct_stat_h, stat_h, bits_stat_h, errno_base_h,
// locale_h, include_locale_h, unistd_h, fcntl_linux_h, fcntl_h,
// dirent_h, include_dirent_h
//
// Also rewrites pub use self::xxx_h:: and use super::xxx_h:: to canonical crate:: imports.

const fs = require('fs');
const path = require('path');

const MOD_NAMES = [
  'struct_timespec_h', 'struct_stat_h', 'stat_h', 'bits_stat_h',
  'errno_base_h', 'locale_h', 'include_locale_h', 'unistd_h',
  'fcntl_linux_h', 'fcntl_h', 'dirent_h', 'include_dirent_h',
];

// Map from module name to canonical crate path for types/constants
const TYPE_CANONICAL = {
  // shared_types.rs
  'timespec': 'crate::xkb::shared_types',
  'stat': 'crate::xkb::shared_types',
  '__S_IFMT': 'crate::xkb::shared_types',
  'ENOMEM': 'crate::xkb::shared_types',
  'EACCES': 'crate::xkb::shared_types',
  'ENOTDIR': 'crate::xkb::shared_types',
  '__LC_CTYPE': 'crate::xkb::shared_types',
  '__LC_ALL': 'crate::xkb::shared_types',
  'LC_CTYPE': 'crate::xkb::shared_types',
  'LC_ALL': 'crate::xkb::shared_types',
  'STDIN_FILENO': 'crate::xkb::shared_types',
  'STDOUT_FILENO': 'crate::xkb::shared_types',
  'STDERR_FILENO': 'crate::xkb::shared_types',
  'R_OK': 'crate::xkb::shared_types',
  'X_OK': 'crate::xkb::shared_types',
  'O_RDONLY': 'crate::xkb::shared_types',
  'O_WRONLY': 'crate::xkb::shared_types',
  'dirent': 'crate::xkb::shared_types',
  'DIR': 'crate::xkb::shared_types',
  // utils.rs (extern fns)
  'fstat': 'crate::xkb::utils',
  'mkdir': 'crate::xkb::utils',
  'xkb_stat': 'crate::xkb::utils',
  'setlocale': 'crate::xkb::utils',
  'eaccess': 'crate::xkb::utils',
  'close': 'crate::xkb::utils',
  'dup': 'crate::xkb::utils',
  'dup2': 'crate::xkb::utils',
  'open': 'crate::xkb::utils',
  'closedir': 'crate::xkb::utils',
  'opendir': 'crate::xkb::utils',
  'readdir': 'crate::xkb::utils',
  '__dirstream': 'crate::xkb::utils',
};

// Module-name to canonical module mapping (for rewriting use statements)
const MOD_CANONICAL = {};
for (const mod_name of MOD_NAMES) {
  // Default: all go to shared_types
  MOD_CANONICAL[mod_name] = 'crate::xkb::shared_types';
}
// Overrides for modules whose symbols are in utils
MOD_CANONICAL['stat_h'] = null; // mixed: types in shared_types, fns in utils
MOD_CANONICAL['fcntl_h'] = 'crate::xkb::utils';
MOD_CANONICAL['include_dirent_h'] = null; // mixed
MOD_CANONICAL['include_locale_h'] = null; // mixed
MOD_CANONICAL['unistd_h'] = null; // mixed

function removeModBlocks(content) {
  const lines = content.split('\n');
  const result = [];
  let inBlock = false;
  let depth = 0;
  let blockModName = null;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (!inBlock) {
      // Check for pub mod xxx_h {
      const m = line.match(/^pub mod ([a-z_]+_h)\s*\{/);
      if (m && MOD_NAMES.includes(m[1])) {
        inBlock = true;
        depth = 1;
        blockModName = m[1];
        // Count braces on this line (after the opening {), string-aware
        depth += countBracesStringAware(line.slice(line.indexOf('{') + 1));
        if (depth <= 0) {
          inBlock = false; // single-line block
        }
        continue; // skip this line
      }
      result.push(line);
    } else {
      // Inside block — count braces (string-aware)
      depth += countBracesStringAware(line);
      if (depth <= 0) {
        inBlock = false;
        blockModName = null;
      }
      // skip line (it's inside the removed block)
    }
  }
  return result.join('\n');
}

function countBracesStringAware(line) {
  let count = 0;
  let inString = false;
  let escape = false;
  for (let i = 0; i < line.length; i++) {
    const ch = line[i];
    if (escape) { escape = false; continue; }
    if (ch === '\\' && inString) { escape = true; continue; }
    if (ch === '"') { inString = !inString; continue; }
    if (inString) continue;
    // Skip line comments
    if (ch === '/' && i + 1 < line.length && line[i + 1] === '/') break;
    if (ch === '{') count++;
    if (ch === '}') count--;
  }
  return count;
}

function rewriteUseStatements(content, filePath) {
  const lines = content.split('\n');
  const result = [];
  let i = 0;

  // Determine if this file IS utils.rs or shared_types.rs (skip self-imports)
  const isUtils = filePath.endsWith('/utils.rs');
  const isSharedTypes = filePath.endsWith('/shared_types.rs');

  while (i < lines.length) {
    let line = lines[i];

    // Match: pub use self::xxx_h:: or use self::xxx_h:: or use super::xxx_h::
    // Also handle pub use crate::xkb::some_file::xxx_h:: (unlikely but safe)
    const useMatch = line.match(/^(\s*)(pub\s+)?use\s+(self|super)::([a-z_]+_h)::/);
    if (useMatch && MOD_NAMES.includes(useMatch[4])) {
      const indent = useMatch[1];
      const pubPrefix = useMatch[2] ? 'pub use' : 'use';
      const modName = useMatch[4];

      // Collect the full use statement (may be multi-line)
      let fullLine = line;
      while (!fullLine.includes(';') && i + 1 < lines.length) {
        i++;
        fullLine += '\n' + lines[i];
      }

      // Extract symbols from the use statement
      const symbols = extractSymbols(fullLine, modName);

      if (symbols.length === 0) {
        // No symbols to import — drop the line
        i++;
        continue;
      }

      // Group symbols by their canonical module
      const grouped = {};
      for (const sym of symbols) {
        let canon = TYPE_CANONICAL[sym];
        if (!canon) {
          // Unknown symbol — guess based on module
          if (MOD_CANONICAL[modName]) {
            canon = MOD_CANONICAL[modName];
          } else {
            // Mixed module — try to determine
            canon = guessCanonical(sym, modName);
          }
        }
        if (!grouped[canon]) grouped[canon] = [];
        grouped[canon].push(sym);
      }

      // Generate replacement use statements
      for (const [canon, syms] of Object.entries(grouped)) {
        // Skip self-imports
        if (isUtils && canon === 'crate::xkb::utils') { i++; continue; }
        if (isSharedTypes && canon === 'crate::xkb::shared_types') { i++; continue; }

        if (syms.length === 1) {
          result.push(`${indent}${pubPrefix} ${canon}::${syms[0]};`);
        } else {
          result.push(`${indent}${pubPrefix} ${canon}::{${syms.join(', ')}};`);
        }
      }

      i++;
      continue;
    }

    // Handle bare stat_h::stat( function calls -> xkb_stat(
    // But NOT type references to stat (the struct)
    if (line.match(/stat_h::stat\s*\(/)) {
      line = line.replace(/stat_h::stat\s*\(/g, 'xkb_stat(');
      // Need to add import for xkb_stat if not present
    }

    result.push(line);
    i++;
  }

  return result.join('\n');
}

function guessCanonical(sym, modName) {
  // For mixed modules, guess based on whether symbol looks like a function or constant
  const fns = ['fstat', 'mkdir', 'stat', 'xkb_stat', 'setlocale', 'eaccess',
    'close', 'dup', 'dup2', 'open', 'closedir', 'opendir', 'readdir'];
  const types = ['__dirstream', 'DIR', 'dirent'];

  if (fns.includes(sym)) return 'crate::xkb::utils';
  if (types.includes(sym) && sym !== 'dirent') return 'crate::xkb::utils';
  if (sym === 'dirent' || sym === 'DIR') return 'crate::xkb::shared_types';
  // Constants go to shared_types
  return 'crate::xkb::shared_types';
}

function extractSymbols(fullLine, modName) {
  // Remove the use prefix up to ::
  // Pattern: use (self|super)::modname::{sym1, sym2, ...};
  // or: use (self|super)::modname::sym;
  const singleMatch = fullLine.match(new RegExp(`(?:self|super)::${modName}::(\\w+)\\s*;`));
  if (singleMatch) {
    return [singleMatch[1]];
  }

  const braceMatch = fullLine.match(new RegExp(`(?:self|super)::${modName}::\\{([^}]+)\\}`));
  if (braceMatch) {
    return braceMatch[1].split(',').map(s => s.trim()).filter(s => s.length > 0);
  }

  // Fallback: try to find any symbol
  const fallback = fullLine.match(new RegExp(`(?:self|super)::${modName}::(\\w+)`));
  if (fallback) {
    return [fallback[1]];
  }

  return [];
}

function processFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf-8');
  const original = content;

  // Step 1: Remove pub mod blocks
  content = removeModBlocks(content);

  // Step 2: Rewrite use statements
  content = rewriteUseStatements(content, filePath);

  // Step 3: Clean up multiple blank lines
  content = content.replace(/\n{3,}/g, '\n\n');

  if (content !== original) {
    fs.writeFileSync(filePath, content);
    const removedLines = original.split('\n').length - content.split('\n').length;
    console.log(`${filePath}: ${removedLines > 0 ? '-' : '+'}${Math.abs(removedLines)} lines`);
    return true;
  }
  return false;
}

// Find all files with relevant pub mod blocks
function findFiles() {
  const srcDir = path.join(__dirname, '..', 'src');
  const files = [];
  function walk(dir) {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.name.endsWith('.rs')) files.push(full);
    }
  }
  walk(srcDir);
  return files;
}

const files = findFiles();
let changed = 0;
for (const f of files) {
  const content = fs.readFileSync(f, 'utf-8');
  // Only process files that reference our target modules
  const hasBlock = MOD_NAMES.some(m => content.includes(`pub mod ${m} {`));
  const hasUse = MOD_NAMES.some(m =>
    content.includes(`self::${m}::`) || content.includes(`super::${m}::`)
  );
  if (hasBlock || hasUse) {
    if (processFile(f)) changed++;
  }
}
console.log(`\nTotal files changed: ${changed}`);
