#!/usr/bin/env node
// Flatten singleton pub mod blocks: remove the wrapper, keep content at file level,
// and convert self::modName:: references to direct use.
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const root = '/home/rano/Public/wkb';

// All singleton modules to flatten (excluding keysym_names_h which is 21K lines)
const SINGLETONS = [
  'xmlversion_h', 'xmlstring_h', 'xmlmemory_h', 'xmlIO_h', 'xmlerror_h', 'xmlautomata_h',
  'xkbcommon_names_h', 'valid_h', 'utils_paths_h', 'util_list_h', 'tree_h',
  'sys_types_h', '__stddef_size_t_h', 'state_priv_h', 'mman_linux_h', 'mman_h',
  'messages_h', 'keymap_formats_h', 'keymap_file_iterator_h', 'keymap_compare_h',
  'inttypes_h', 'iconv_h', 'hash_h', 'entities_h', 'encoding_h', 'dump_h',
  'dict_h', 'darray_h', 'compose_iter_h', 'assert_h', 'action_h',
];

let totalFlattened = 0;
let filesChanged = 0;

for (const modName of SINGLETONS) {
  let grepResult;
  try {
    grepResult = execSync(`cd ${root} && grep -rn 'pub mod ${modName} {' src/`, { encoding: 'utf8' }).trim();
  } catch { continue; }
  
  if (!grepResult) continue;
  const match = grepResult.match(/^(.*?):(\d+):/);
  if (!match) continue;
  const [, relFile, lineNumStr] = match;
  const lineNum = parseInt(lineNumStr);
  
  const filePath = path.join(root, relFile);
  const content = fs.readFileSync(filePath, 'utf8');
  const lines = content.split('\n');
  
  // Find the block boundaries
  let depth = 0;
  let blockStart = lineNum - 1; // 0-indexed
  let blockEnd = -1;
  
  for (let i = blockStart; i < lines.length; i++) {
    let inStr = false;
    for (let j = 0; j < lines[i].length; j++) {
      if (lines[i][j] === '"' && (j === 0 || lines[i][j-1] !== '\\')) inStr = !inStr;
      if (!inStr) {
        if (lines[i][j] === '{') depth++;
        if (lines[i][j] === '}') depth--;
      }
    }
    if (depth === 0) {
      blockEnd = i;
      break;
    }
  }
  
  if (blockEnd < 0) {
    console.log(`WARNING: ${modName} in ${relFile}: could not find block end`);
    continue;
  }
  
  // Extract block content (skip first line "pub mod xxx_h {" and last line "}")
  const blockContent = lines.slice(blockStart + 1, blockEnd);
  
  // Dedent by 4 spaces (typical module indentation)
  const dedented = blockContent.map(l => {
    if (l.startsWith('    ')) return l.substring(4);
    if (l.trim() === '') return '';
    return l;
  });
  
  // Build new file: replace the block with dedented content
  const newLines = [
    ...lines.slice(0, blockStart),
    ...dedented,
    ...lines.slice(blockEnd + 1),
  ];
  
  let newContent = newLines.join('\n');
  
  // Replace self::modName:: with direct reference (since content is now at file level)
  // pub use self::xxx_h::Y -> just remove the line (Y is now at file level)
  // use self::xxx_h::Y -> just remove the line
  newContent = newContent.split('\n').filter(l => {
    return !l.match(new RegExp(`^pub use self::${modName}::`)) &&
           !l.match(new RegExp(`^use self::${modName}::`));
  }).join('\n');
  
  // For references inside other blocks: super::xxx_h:: or self::xxx_h::
  // These need to stay as-is since we're flattening — the symbols are now at file level
  // so super::xxx_h::Foo -> just Foo (since it's in scope)
  // Actually, for nested modules using super::xxx_h::, we need super:: (parent module)
  // But since we're flattening into the same file, super::xxx_h::Foo -> super::Foo
  // This is tricky — let's handle it differently: don't change these, just ensure
  // the self:: pub use removal handles the common case
  
  if (newContent !== content) {
    fs.writeFileSync(filePath, newContent);
    const removed = lines.length - newContent.split('\n').length;
    totalFlattened++;
    filesChanged++;
    console.log(`${modName}: ${relFile} — flattened (${removed > 0 ? removed + ' lines saved' : 'reformatted'})`);
  }
}

console.log(`\nTotal: ${totalFlattened} modules flattened in ${filesChanged} files`);
