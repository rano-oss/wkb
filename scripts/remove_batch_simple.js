#!/usr/bin/env node
// Generic batch removal for pub mod blocks that are purely re-exports or simple constants.
// For each module, removes all `pub mod xxx_h { ... }` blocks and converts
// `self::xxx_h::` / `super::xxx_h::` references to canonical paths.
// Blocks that contain canonical (non-re-export) definitions are kept and flattened.
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const root = '/home/rano/Public/wkb';

// Module name -> canonical crate path for re-export replacement
// "FLATTEN" means the block in this specific file should be flattened to file-level
const MODULES = {
  'utils_paths_h': 'crate::xkb::utils_paths',
  'vmod_h': 'crate::xkb::xkbcomp::vmod',
  'action_h': 'crate::xkb::xkbcomp::action',
  'rules_h': 'crate::xkb::xkbcomp::rules',
  'parser_priv_h': 'crate::xkb::xkbcomp::scanner',
  'ast_build_h': 'crate::xkb::xkbcomp::ast_build',
  'rmlvo_h': 'crate::xkb::rmlvo',
};

function removeBlock(content, modName) {
  const lines = content.split('\n');
  const result = [];
  let inBlock = false;
  let depth = 0;
  let inPubUse = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Remove pub mod xxx_h { ... } block
    if (!inBlock && line.match(new RegExp(`^pub mod ${modName}\\s*\\{`))) {
      inBlock = true;
      depth = 1;
      continue;
    }
    if (inBlock) {
      let inStr = false;
      for (let j = 0; j < line.length; j++) {
        if (line[j] === '"' && (j === 0 || line[j-1] !== '\\')) inStr = !inStr;
        if (!inStr) {
          if (line[j] === '{') depth++;
          if (line[j] === '}') depth--;
        }
      }
      if (depth <= 0) inBlock = false;
      continue;
    }

    result.push(line);
  }
  return result.join('\n');
}

function replaceImports(content, modName, canonical) {
  // Replace pub use self::xxx_h:: -> pub use <canonical>::
  // Replace use super::xxx_h:: -> use <canonical>::
  // Replace use self::xxx_h:: -> use <canonical>::
  // Handle the rmlvo_h special case where it might reference rmlvo_h::xxx (inside blocks)
  let result = content;
  result = result.replace(new RegExp(`self::${modName}::`, 'g'), `${canonical}::`);
  result = result.replace(new RegExp(`super::${modName}::`, 'g'), `${canonical}::`);
  return result;
}

let totalRemoved = 0;
let filesChanged = 0;

for (const [modName, canonical] of Object.entries(MODULES)) {
  // Find all files with this module
  let grepResult;
  try {
    grepResult = execSync(`cd ${root} && grep -rln 'pub mod ${modName}\\b' src/`, { encoding: 'utf8' }).trim();
  } catch { continue; }
  
  if (!grepResult) continue;
  const files = grepResult.split('\n');
  
  for (const file of files) {
    const filePath = path.join(root, file);
    const origContent = fs.readFileSync(filePath, 'utf8');
    const origLen = origContent.split('\n').length;
    
    let content = removeBlock(origContent, modName);
    content = replaceImports(content, modName, canonical);
    
    const newLen = content.split('\n').length;
    const removed = origLen - newLen;
    
    if (content !== origContent) {
      fs.writeFileSync(filePath, content);
      totalRemoved += removed;
      filesChanged++;
      if (removed > 0) {
        console.log(`${modName}: ${file} — ${removed} lines removed`);
      } else {
        console.log(`${modName}: ${file} — imports updated`);
      }
    }
  }
  
  // Also fix references in files that don't have the module defined
  // (they might have `use self::xxx_h::` in nested blocks)
  try {
    const refFiles = execSync(`cd ${root} && grep -rln '${modName}::' src/ || true`, { encoding: 'utf8' }).trim();
    if (refFiles) {
      for (const file of refFiles.split('\n')) {
        const filePath = path.join(root, file);
        const origContent = fs.readFileSync(filePath, 'utf8');
        const content = replaceImports(origContent, modName, canonical);
        if (content !== origContent) {
          fs.writeFileSync(filePath, content);
          filesChanged++;
          console.log(`${modName}: ${file} — references updated`);
        }
      }
    }
  } catch {}
}

console.log(`\nTotal: ${totalRemoved} lines removed, ${filesChanged} files changed`);
