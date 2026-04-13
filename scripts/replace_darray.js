#!/usr/bin/env node
// Phase 1: Replace all darray_next_alloc + realloc blocks with darray_growalloc.
// 
// Target pattern (the if-block containing darray_next_alloc + realloc):
//   if NEED > EXPR.alloc {
//       EXPR.alloc = darray_next_alloc(EXPR.alloc, NEED, size_of::<T>());
//       EXPR.item = realloc(EXPR.item as *mut c_void, (EXPR.alloc as usize) * size_of::<T>()) as *mut T;
//   }
//
// Replacement:
//   darray_growalloc(&mut EXPR.item, &mut EXPR.alloc, NEED);
//
// We also look backwards for `let mut __need = EXPR.size;` and remove it, using EXPR.size directly.

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const DRY_RUN = process.argv.includes('--dry-run');

const grepResult = execSync(
  'grep -rn "darray_next_alloc" /home/rano/Public/wkb/src/ --include="*.rs" -l',
  { encoding: 'utf8' }
).trim();
const files = grepResult.split('\n');

let totalReplaced = 0;

for (const filePath of files) {
  const relPath = path.relative('/home/rano/Public/wkb', filePath);
  if (relPath === 'src/xkb/utils.rs' || relPath === 'src/xkb/xkbcomp/rules.rs') {
    continue; // already done or helper definitions
  }

  let content = fs.readFileSync(filePath, 'utf8');
  let lines = content.split('\n');
  let replaced = 0;
  let needsImport = false;

  // Process from bottom to top
  for (let i = lines.length - 1; i >= 0; i--) {
    if (!lines[i].includes('darray_next_alloc')) continue;
    if (lines[i].includes('pub fn darray_next_alloc') ||
        lines[i].includes('pub unsafe fn darray_next_alloc') ||
        lines[i].includes('pub use')) continue;

    // Found a darray_next_alloc call. Extract the expression base and need variable.
    // Find the if-block boundaries by scanning up and down.
    
    // 1. Find the 'if' line (scan up from darray_next_alloc)
    let ifLine = -1;
    let needVar = '';
    let allocExpr = '';
    
    for (let j = i - 1; j >= Math.max(0, i - 5); j--) {
      const m = lines[j].match(/if\s+(\w+)\s*>\s*(.+\.alloc)\s*\{?\s*$/);
      if (m) {
        ifLine = j;
        needVar = m[1];
        allocExpr = m[2];
        break;
      }
      // Also handle: if __need > EXPR.alloc {  (on one line)
      const m2 = lines[j].match(/if\s+(\w+)\s*>\s*(.+)\.alloc/);
      if (m2) {
        ifLine = j;
        needVar = m2[1];
        allocExpr = m2[2].trim() + '.alloc';
        break;
      }
    }
    
    if (ifLine === -1) {
      console.log(`  WARN L${i+1}: can't find if-line for darray_next_alloc`);
      continue;
    }
    
    // 2. Find the closing '}' of the if-block
    let closingBrace = -1;
    let braceDepth = 0;
    for (let j = ifLine; j < Math.min(lines.length, i + 20); j++) {
      for (const ch of lines[j]) {
        if (ch === '{') braceDepth++;
        if (ch === '}') {
          braceDepth--;
          if (braceDepth === 0) {
            closingBrace = j;
            break;
          }
        }
      }
      if (closingBrace !== -1) break;
    }
    
    if (closingBrace === -1) {
      console.log(`  WARN L${i+1}: can't find closing brace`);
      continue;
    }
    
    // 3. Extract item expression from realloc line
    // Looking for: EXPR.item = realloc(EXPR.item as *mut ...
    let itemExpr = '';
    for (let j = i; j <= closingBrace; j++) {
      const m = lines[j].match(/(\S+\.item)\s*=\s*realloc\(/);
      if (m) {
        itemExpr = m[1];
        break;
      }
    }
    
    if (!itemExpr) {
      console.log(`  WARN L${i+1}: can't find item expression in realloc`);
      continue;
    }
    
    // Extract the base expression (everything before .alloc / .item)
    // allocExpr is like "(*to).levels.alloc"
    const exprBase = allocExpr.replace(/\.alloc$/, '');
    
    // 4. Find the `let mut __need = ...;` line above the if
    let needDeclLine = -1;
    for (let j = ifLine - 1; j >= Math.max(0, ifLine - 3); j--) {
      if (lines[j].match(new RegExp('let\\s+mut\\s+' + needVar + '\\s*[=:]'))) {
        needDeclLine = j;
        break;
      }
    }
    
    // Get the need expression value
    let needValue = needVar; // default: just use the variable
    if (needDeclLine !== -1) {
      // Extract: let mut __need: type = EXPR;
      const declMatch = lines[needDeclLine].match(/=\s*(.+?)\s*;/);
      if (declMatch) {
        needValue = declMatch[1].trim();
      }
    }
    
    // 5. Build replacement
    const indent = lines[ifLine].match(/^(\s*)/)[1];
    const replacement = `${indent}darray_growalloc(&mut ${itemExpr}, &mut ${exprBase}.alloc, ${needValue});`;
    
    // 6. Remove the if-block and optionally the __need declaration
    let removeStart = ifLine;
    let removeEnd = closingBrace;
    
    // If we have a __need declaration that's only used here, remove it too
    if (needDeclLine !== -1 && needDeclLine === ifLine - 1) {
      removeStart = needDeclLine;
    }
    
    if (!DRY_RUN) {
      lines.splice(removeStart, removeEnd - removeStart + 1, replacement);
    }
    
    replaced++;
    needsImport = true;
    console.log(`  L${i+1}: ${exprBase} → darray_growalloc (lines ${removeStart+1}-${removeEnd+1})`);
  }
  
  if (replaced > 0 && !DRY_RUN) {
    // Add import if not already present
    const importLine = 'use crate::xkb::utils::darray_growalloc;';
    const hasImport = lines.some(l => l.includes('darray_growalloc'));
    
    if (!hasImport) {
      // Find a good place to add the import - after existing utils imports or after the first use block
      let insertIdx = -1;
      for (let j = 0; j < lines.length; j++) {
        if (lines[j].includes('use crate::xkb::utils::')) {
          // Add to existing import
          const m = lines[j].match(/use crate::xkb::utils::\{(.+)\};/);
          if (m) {
            lines[j] = lines[j].replace(m[1], m[1] + ', darray_growalloc');
            insertIdx = -2; // marker: already handled
            break;
          }
        }
      }
      
      if (insertIdx !== -2) {
        // Find a good insertion point
        for (let j = 0; j < Math.min(lines.length, 800); j++) {
          if (lines[j].match(/^pub use self::|^use self::/)) {
            insertIdx = j;
          }
        }
        if (insertIdx !== -1) {
          lines.splice(insertIdx + 1, 0, importLine);
        }
      }
    }
    
    fs.writeFileSync(filePath, lines.join('\n'));
  }
  
  if (replaced > 0) {
    console.log(`${relPath}: ${replaced} replacements`);
    totalReplaced += replaced;
  }
}

console.log(`\nTotal: ${totalReplaced} replacements`);
