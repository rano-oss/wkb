#!/usr/bin/env node
// Remove all pub mod ast_h blocks from xkbcomp files
// Strategy:
// 1. Add C2Rust_Unnamed type aliases + re-exports to shared_ast_types.rs
// 2. Remove pub mod ast_h { ... } blocks
// 3. Convert pub use self::ast_h:: → pub use crate::xkb::shared_ast_types::
// 4. Convert use super::ast_h:: → use crate::xkb::shared_ast_types::

const fs = require('fs');
const path = require('path');

const ROOT = '/home/rano/Public/wkb/src/xkb';

// Step 1: Add type aliases and re-exports to shared_ast_types.rs
const sharedFile = path.join(ROOT, 'shared_ast_types.rs');
let sharedContent = fs.readFileSync(sharedFile, 'utf8');

// Add after DarrayKeysym struct (after line with `pub item: *mut xkb_keysym_t,\n}`)
const darrayEnd = 'pub struct DarrayKeysym {\n    pub size: darray_size_t,\n    pub alloc: darray_size_t,\n    pub item: *mut xkb_keysym_t,\n}';
if (!sharedContent.includes('pub type C2Rust_Unnamed_1 = DarrayKeysym')) {
  const insertAfter = darrayEnd;
  const aliases = `

// Type aliases for C2Rust_Unnamed variants that map to DarrayKeysym
pub type C2Rust_Unnamed_1 = DarrayKeysym;
pub type C2Rust_Unnamed_13 = DarrayKeysym;
pub type C2Rust_Unnamed_15 = DarrayKeysym;

// Re-export ast_build functions used by consumers via ast_h
pub use crate::xkb::xkbcomp::ast_build::{stmt_type_to_string, xkb_file_type_to_string, stmt_type_to_operator_char};`;

  sharedContent = sharedContent.replace(insertAfter, insertAfter + aliases);
  fs.writeFileSync(sharedFile, sharedContent);
  console.log('Updated shared_ast_types.rs with type aliases + re-exports');
} else {
  console.log('shared_ast_types.rs already has C2Rust_Unnamed aliases');
}

// Step 2-4: Process each file with ast_h block
const { execSync } = require('child_process');
const files = execSync(`grep -rln 'pub mod ast_h' ${ROOT}/`, { encoding: 'utf8' }).trim().split('\n');

let totalRemoved = 0;
let totalFiles = 0;

for (const file of files) {
  let content = fs.readFileSync(file, 'utf8');
  const origLen = content.split('\n').length;
  const shortName = file.replace('/home/rano/Public/wkb/', '');
  
  // Step 2: Remove pub mod ast_h { ... } block
  const lines = content.split('\n');
  const newLines = [];
  let inBlock = false;
  let depth = 0;
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    if (!inBlock && line.match(/^pub mod ast_h\s*\{/)) {
      inBlock = true;
      depth = 0;
    }
    
    if (inBlock) {
      for (const ch of line) {
        if (ch === '{') depth++;
        if (ch === '}') depth--;
      }
      if (depth === 0) {
        inBlock = false;
      }
      continue; // skip this line (inside block)
    }
    
    newLines.push(line);
  }
  
  content = newLines.join('\n');
  
  // Step 3: Convert pub use self::ast_h:: → pub use crate::xkb::shared_ast_types::
  content = content.replace(/pub use self::ast_h::/g, 'pub use crate::xkb::shared_ast_types::');
  
  // Step 4: Convert use super::ast_h:: → use crate::xkb::shared_ast_types::
  content = content.replace(/use super::ast_h::/g, 'use crate::xkb::shared_ast_types::');
  
  // Also handle any bare ast_h:: references
  content = content.replace(/super::ast_h::/g, 'crate::xkb::shared_ast_types::');
  
  fs.writeFileSync(file, content);
  const newLen = content.split('\n').length;
  const removed = origLen - newLen;
  totalRemoved += removed;
  totalFiles++;
  console.log(`  ${shortName}: removed ${removed} lines`);
}

console.log(`\nTotal: ${totalFiles} files, ${totalRemoved} lines removed`);
