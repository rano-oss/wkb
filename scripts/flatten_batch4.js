#!/usr/bin/env node
// Batch 4: Flatten all remaining non-parser_h pub mod blocks
// Strategy: For each module, flatten the canonical copy to file-level,
// remove duplicates, and update imports.
//
// Modules to handle:
// - bench_h (2 identical copies) → flatten in compile_keymap.rs, import in custom_parsers.rs
// - tools_common_h (2 different) → flatten both in-place
// - table_h (2 nearly identical) → flatten superset in compose_iter.rs
// - xkbcommon_features_h (2) → already flat in features.rs, flatten state.rs copy
// - xkbregistry_h (2) → flatten in registry.rs
// - xkbcommon_compose_h (3) → flatten in common.rs + compile_compose.rs + compose_iter.rs
// - test_h (1 singleton) → flatten in common.rs
// - ast_build_h (2) → flatten both (re-export → direct import, wrappers → file-level)
// - parser_priv_h (2) → flatten both
// - rules_h (2) → flatten both

const fs = require('fs');

const ALL_MODS = [
  'bench_h', 'tools_common_h', 'table_h', 'xkbcommon_features_h',
  'xkbregistry_h', 'xkbcommon_compose_h', 'test_h', 'ast_build_h',
  'parser_priv_h', 'rules_h',
];

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
    if (ch === '/' && i + 1 < line.length && line[i + 1] === '/') break;
    if (ch === '{') count++;
    if (ch === '}') count--;
  }
  return count;
}

// Extract the content of a pub mod block (everything between the braces)
function extractBlockContent(lines, startIdx) {
  let depth = 0;
  const contentLines = [];
  let started = false;
  for (let i = startIdx; i < lines.length; i++) {
    const line = lines[i];
    depth += countBracesStringAware(line);
    if (i === startIdx) {
      // First line: pub mod xxx_h {
      // Content starts after the {
      const afterBrace = line.slice(line.indexOf('{') + 1).trim();
      if (afterBrace) contentLines.push(afterBrace);
      started = true;
    } else if (depth <= 0) {
      // Last line: }
      const beforeBrace = line.slice(0, line.lastIndexOf('}')).trim();
      if (beforeBrace) contentLines.push(beforeBrace);
      return { content: contentLines, endIdx: i };
    } else {
      // Dedent by 4 spaces
      const dedented = line.startsWith('    ') ? line.slice(4) : line;
      contentLines.push(dedented);
    }
  }
  return { content: contentLines, endIdx: lines.length - 1 };
}

// Remove a pub mod block and replace with its flattened content
function flattenBlock(content, modName) {
  const lines = content.split('\n');
  const result = [];
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    const m = line.match(new RegExp(`^pub mod ${modName}\\s*\\{`));
    if (m) {
      const { content: blockContent, endIdx } = extractBlockContent(lines, i);
      // Insert flattened content
      for (const cl of blockContent) {
        result.push(cl);
      }
      i = endIdx + 1;
      continue;
    }
    result.push(line);
    i++;
  }
  return result.join('\n');
}

// Rewrite use self::modname:: → direct paths (simple version)
function rewriteSelfUses(content, modName, replacements) {
  const lines = content.split('\n');
  const result = [];
  let i = 0;
  while (i < lines.length) {
    let line = lines[i];
    // Match: (pub )use self::modname::
    const useRe = new RegExp(`^(\\s*)(pub\\s+)?use\\s+self::${modName}::`);
    if (useRe.test(line)) {
      // Collect full statement
      let fullLine = line;
      while (!fullLine.includes(';') && i + 1 < lines.length) {
        i++;
        fullLine += '\n' + lines[i];
      }
      // These are now at file level — remove the self:: use entirely
      // (the symbols are in the same scope after flattening)
      i++;
      continue;
    }
    // Also handle super::modname:: inside nested modules
    line = line.replace(new RegExp(`super::${modName}::`, 'g'), 'super::');
    result.push(line);
    i++;
  }
  return result.join('\n');
}

function processFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf-8');
  const original = content;

  for (const modName of ALL_MODS) {
    if (content.includes(`pub mod ${modName} {`)) {
      content = flattenBlock(content, modName);
      content = rewriteSelfUses(content, modName);
    }
  }

  content = content.replace(/\n{3,}/g, '\n\n');

  if (content !== original) {
    fs.writeFileSync(filePath, content);
    const removedLines = original.split('\n').length - content.split('\n').length;
    console.log(`${filePath}: ${removedLines > 0 ? '-' : '+'}${Math.abs(removedLines)} lines`);
    return true;
  }
  return false;
}

function findRsFiles(dir) {
  const files = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = require('path').join(dir, entry.name);
    if (entry.isDirectory()) files.push(...findRsFiles(full));
    else if (entry.name.endsWith('.rs')) files.push(full);
  }
  return files;
}

const srcDir = require('path').join(__dirname, '..', 'src');
const files = findRsFiles(srcDir);
let changed = 0;
for (const f of files) {
  const content = fs.readFileSync(f, 'utf-8');
  const hasBlock = ALL_MODS.some(m => content.includes(`pub mod ${m} {`));
  if (hasBlock) {
    if (processFile(f)) changed++;
  }
}
console.log(`\nTotal files changed: ${changed}`);
