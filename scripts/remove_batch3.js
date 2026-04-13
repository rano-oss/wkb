#!/usr/bin/env node
// Batch 3: Remove enums_h, getopt_core_h, getopt_ext_h, rmlvo_h
// Also handle: rules_h, xkbcommon_compose_h (these have function bodies — flatten, don't just remove)
//
// Strategy:
// - enums_h: 21 constants → shared_types.rs; statics (features.rs only) → features.rs file-level
// - getopt_core_h: extern statics (optarg, optind) → utils.rs
// - getopt_ext_h: option struct + consts + getopt_long extern fn → shared_types.rs + utils.rs
// - rmlvo_h: struct defs canonical in rmlvo.rs (flatten to file-level); RMLVO enum → shared_types.rs

const fs = require('fs');
const path = require('path');

const REMOVE_MODS = [
  'enums_h', 'getopt_core_h', 'getopt_ext_h', 'rmlvo_h',
];

// For these we only add canonical defs, not remove blocks (they have fn bodies to flatten)
// rules_h and xkbcommon_compose_h will be handled in a later batch

const TYPE_CANONICAL = {
  // enums_h constants → shared_types
  'XKB_COMPOSE_FEED_RESULT_VALUES': 'crate::xkb::shared_types',
  'XKB_COMPOSE_STATUS_VALUES': 'crate::xkb::shared_types',
  'XKB_COMPOSE_STATE_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_COMPOSE_FORMAT_VALUES': 'crate::xkb::shared_types',
  'XKB_COMPOSE_COMPILE_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_CONSUMED_MODE_VALUES': 'crate::xkb::shared_types',
  'XKB_STATE_MATCH_VALUES': 'crate::xkb::shared_types',
  'XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES': 'crate::xkb::shared_types',
  'XKB_KEY_DIRECTION_VALUES': 'crate::xkb::shared_types',
  'XKB_A11Y_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_EVENTS_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_KEYBOARD_CONTROL_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_STATE_COMPONENT_VALUES': 'crate::xkb::shared_types',
  'XKB_EVENT_TYPE_VALUES': 'crate::xkb::shared_types',
  'XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_KEYMAP_SERIALIZE_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_KEYMAP_FORMAT_VALUES': 'crate::xkb::shared_types',
  'XKB_KEYMAP_COMPILE_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_CONTEXT_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_KEYSYM_FLAGS_VALUES': 'crate::xkb::shared_types',
  'XKB_RMLVO_BUILDER_FLAGS_VALUES': 'crate::xkb::shared_types',
  // statics from features.rs enums_h — stay in features.rs
  'xkb_log_level_values': 'crate::xkb::features',
  'xkb_error_code_values': 'crate::xkb::features',
  // getopt_core_h statics → utils
  'optarg': 'crate::xkb::utils',
  'optind': 'crate::xkb::utils',
  // getopt_ext_h → shared_types (struct/consts) + utils (fn)
  'option': 'crate::xkb::shared_types',
  'no_argument': 'crate::xkb::shared_types',
  'required_argument': 'crate::xkb::shared_types',
  'getopt_long': 'crate::xkb::utils',
  // rmlvo_h
  'RMLVO': 'crate::xkb::shared_types',
  'RMLVO_OPTIONS': 'crate::xkb::shared_types',
  'RMLVO_VARIANT': 'crate::xkb::shared_types',
  'RMLVO_LAYOUT': 'crate::xkb::shared_types',
  'RMLVO_MODEL': 'crate::xkb::shared_types',
  'RMLVO_RULES': 'crate::xkb::shared_types',
  'xkb_rmlvo_builder': 'crate::xkb::rmlvo',
  'xkb_rmlvo_builder_options': 'crate::xkb::rmlvo',
  'xkb_rmlvo_builder_option': 'crate::xkb::rmlvo',
  'xkb_rmlvo_builder_layouts': 'crate::xkb::rmlvo',
  'xkb_rmlvo_builder_layout': 'crate::xkb::rmlvo',
  'OPTIONS_GROUP_SPECIFIER_PREFIX': 'crate::xkb::shared_types',
};

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

function removeModBlocks(content) {
  const lines = content.split('\n');
  const result = [];
  let inBlock = false;
  let depth = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (!inBlock) {
      const m = line.match(/^pub mod ([a-z_]+_h)\s*\{/);
      if (m && REMOVE_MODS.includes(m[1])) {
        inBlock = true;
        depth = 1;
        depth += countBracesStringAware(line.slice(line.indexOf('{') + 1));
        if (depth <= 0) inBlock = false;
        continue;
      }
      result.push(line);
    } else {
      depth += countBracesStringAware(line);
      if (depth <= 0) inBlock = false;
    }
  }
  return result.join('\n');
}

function rewriteUseStatements(content, filePath) {
  const lines = content.split('\n');
  const result = [];
  let i = 0;
  const isUtils = filePath.endsWith('/utils.rs');
  const isSharedTypes = filePath.endsWith('/shared_types.rs');
  const isRmlvo = filePath.endsWith('/rmlvo.rs');
  const isFeatures = filePath.endsWith('/features.rs');

  while (i < lines.length) {
    let line = lines[i];
    const useMatch = line.match(/^(\s*)(pub\s+)?use\s+(self|super)::([a-z_]+_h)::/);
    if (useMatch && REMOVE_MODS.includes(useMatch[4])) {
      const indent = useMatch[1];
      const pubPrefix = useMatch[2] ? 'pub use' : 'use';
      const modName = useMatch[4];

      let fullLine = line;
      while (!fullLine.includes(';') && i + 1 < lines.length) {
        i++;
        fullLine += '\n' + lines[i];
      }

      const symbols = extractSymbols(fullLine, modName);
      if (symbols.length === 0) { i++; continue; }

      const grouped = {};
      for (const sym of symbols) {
        let canon = TYPE_CANONICAL[sym];
        if (!canon) {
          // Default fallback
          if (modName === 'enums_h') canon = 'crate::xkb::shared_types';
          else if (modName === 'getopt_core_h') canon = 'crate::xkb::utils';
          else if (modName === 'getopt_ext_h') canon = 'crate::xkb::shared_types';
          else if (modName === 'rmlvo_h') canon = 'crate::xkb::rmlvo';
          else canon = 'crate::xkb::shared_types';
        }
        if (!grouped[canon]) grouped[canon] = [];
        grouped[canon].push(sym);
      }

      for (const [canon, syms] of Object.entries(grouped)) {
        // Skip self-imports
        if (isUtils && canon === 'crate::xkb::utils') continue;
        if (isSharedTypes && canon === 'crate::xkb::shared_types') continue;
        if (isRmlvo && canon === 'crate::xkb::rmlvo') continue;
        if (isFeatures && canon === 'crate::xkb::features') continue;

        if (syms.length === 1) {
          result.push(`${indent}${pubPrefix} ${canon}::${syms[0]};`);
        } else {
          result.push(`${indent}${pubPrefix} ${canon}::{${syms.join(', ')}};`);
        }
      }
      i++;
      continue;
    }
    result.push(line);
    i++;
  }
  return result.join('\n');
}

function extractSymbols(fullLine, modName) {
  const singleMatch = fullLine.match(new RegExp(`(?:self|super)::${modName}::(\\w+)\\s*;`));
  if (singleMatch) return [singleMatch[1]];

  const braceMatch = fullLine.match(new RegExp(`(?:self|super)::${modName}::\\{([^}]+)\\}`));
  if (braceMatch) return braceMatch[1].split(',').map(s => s.trim()).filter(s => s.length > 0);

  // Glob import
  const globMatch = fullLine.match(new RegExp(`(?:self|super)::${modName}::\\*`));
  if (globMatch) return ['*'];

  const fallback = fullLine.match(new RegExp(`(?:self|super)::${modName}::(\\w+)`));
  if (fallback) return [fallback[1]];

  return [];
}

function processFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf-8');
  const original = content;

  content = removeModBlocks(content);
  content = rewriteUseStatements(content, filePath);
  content = content.replace(/\n{3,}/g, '\n\n');

  if (content !== original) {
    fs.writeFileSync(filePath, content);
    const removedLines = original.split('\n').length - content.split('\n').length;
    console.log(`${filePath}: ${removedLines > 0 ? '-' : '+'}${Math.abs(removedLines)} lines`);
    return true;
  }
  return false;
}

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
  const hasBlock = REMOVE_MODS.some(m => content.includes(`pub mod ${m} {`));
  const hasUse = REMOVE_MODS.some(m =>
    content.includes(`self::${m}::`) || content.includes(`super::${m}::`)
  );
  if (hasBlock || hasUse) {
    if (processFile(f)) changed++;
  }
}
console.log(`\nTotal files changed: ${changed}`);
