use crate::xkb::messages::{XKB_ERROR_INVALID_PATH, XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH};
use crate::xkb::shared_types::*;

pub type rxkb_log_level = u32;
pub const RXKB_LOG_LEVEL_DEBUG: rxkb_log_level = 50;
pub const RXKB_LOG_LEVEL_INFO: rxkb_log_level = 40;
pub const RXKB_LOG_LEVEL_WARNING: rxkb_log_level = 30;
pub const RXKB_LOG_LEVEL_ERROR: rxkb_log_level = 20;
pub const RXKB_LOG_LEVEL_CRITICAL: rxkb_log_level = 10;
pub type rxkb_popularity = u32;
pub const RXKB_POPULARITY_EXOTIC: rxkb_popularity = 2;
pub const RXKB_POPULARITY_STANDARD: rxkb_popularity = 1;
pub type rxkb_context_flags = u32;
pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;
pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;

pub type context_state = u32;
pub const CONTEXT_FAILED: context_state = 2;
pub const CONTEXT_PARSED: context_state = 1;
pub const CONTEXT_NEW: context_state = 0;

// ---------------------------------------------------------------------------
// Types — fully owned, no raw pointers, no #[repr(C)], no rxkb_object
// ---------------------------------------------------------------------------

pub struct rxkb_context {
    pub context_state: context_state,
    pub load_extra_rules_files: bool,
    pub models: Vec<rxkb_model>,
    pub layouts: Vec<rxkb_layout>,
    pub option_groups: Vec<rxkb_option_group>,
    pub includes: Vec<String>,
    pub log_fn: Option<fn(&rxkb_context, rxkb_log_level, &str)>,
    pub log_level: rxkb_log_level,
}

#[derive(Clone)]
pub struct rxkb_model {
    pub name: String,
    pub vendor: String,
    pub description: String,
    pub popularity: rxkb_popularity,
}

pub struct rxkb_layout {
    pub name: String,
    pub brief: String,
    pub description: String,
    pub variant: String,
    pub popularity: rxkb_popularity,
    pub iso639s: Vec<String>,
    pub iso3166s: Vec<String>,
}

pub struct rxkb_option_group {
    pub allow_multiple: bool,
    pub options: Vec<rxkb_option>,
    pub name: String,
    pub description: String,
    pub popularity: rxkb_popularity,
}

#[derive(Clone)]
pub struct rxkb_option {
    pub name: String,
    pub brief: String,
    pub description: String,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}

#[derive(Clone)]
pub struct config_item {
    pub name: String,
    pub description: String,
    pub brief: String,
    pub vendor: String,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}

// ---------------------------------------------------------------------------
// Accessors
// ---------------------------------------------------------------------------

impl rxkb_layout {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn brief(&self) -> &str {
        &self.brief
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn variant(&self) -> &str {
        &self.variant
    }
    pub fn iso639s(&self) -> &[String] {
        &self.iso639s
    }
    pub fn iso3166s(&self) -> &[String] {
        &self.iso3166s
    }
}

impl rxkb_model {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn vendor(&self) -> &str {
        &self.vendor
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl rxkb_option_group {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn allows_multiple(&self) -> bool {
        self.allow_multiple
    }
    pub fn options(&self) -> &[rxkb_option] {
        &self.options
    }
}

impl rxkb_option {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn brief(&self) -> &str {
        &self.brief
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn is_layout_specific(&self) -> bool {
        self.layout_specific
    }
}

// ---------------------------------------------------------------------------
// Logging
// ---------------------------------------------------------------------------

macro_rules! rxkb_logf {
    ($ctx:expr, $level:expr, $($arg:tt)*) => {{
        rxkb_log($ctx, $level, &format!($($arg)*));
    }};
}

fn rxkb_log(ctx: &rxkb_context, level: rxkb_log_level, msg: &str) {
    if ctx.log_level < level {
        return;
    }
    if let Some(f) = ctx.log_fn {
        f(ctx, level, msg);
    }
}

fn log_level_to_prefix(level: rxkb_log_level) -> &'static str {
    match level {
        50 => "xkbregistry: DEBUG: ",
        40 => "xkbregistry: INFO: ",
        30 => "xkbregistry: WARNING: ",
        20 => "xkbregistry: ERROR: ",
        10 => "xkbregistry: CRITICAL: ",
        _ => "",
    }
}

fn default_log_fn(_ctx: &rxkb_context, level: rxkb_log_level, msg: &str) {
    let prefix = log_level_to_prefix(level);
    if !prefix.is_empty() {
        eprint!("{}", prefix);
    }
    eprint!("{}", msg);
}

fn log_level_from_str(level: &str) -> rxkb_log_level {
    // Try numeric first
    if let Ok(val) = level.trim().parse::<u32>() {
        return val;
    }
    let l = level.to_ascii_lowercase();
    if l.starts_with("crit") {
        RXKB_LOG_LEVEL_CRITICAL
    } else if l.starts_with("err") {
        RXKB_LOG_LEVEL_ERROR
    } else if l.starts_with("warn") {
        RXKB_LOG_LEVEL_WARNING
    } else if l.starts_with("info") {
        RXKB_LOG_LEVEL_INFO
    } else if l.starts_with("debug") || l.starts_with("dbg") {
        RXKB_LOG_LEVEL_DEBUG
    } else {
        RXKB_LOG_LEVEL_ERROR
    }
}

// ---------------------------------------------------------------------------
// Context construction & public API
// ---------------------------------------------------------------------------

impl rxkb_context {
    pub fn new(flags: rxkb_context_flags) -> Option<Box<rxkb_context>> {
        let valid_flags: rxkb_context_flags = RXKB_CONTEXT_NO_DEFAULT_INCLUDES
            | RXKB_CONTEXT_LOAD_EXOTIC_RULES
            | RXKB_CONTEXT_NO_SECURE_GETENV;

        let mut ctx = Box::new(rxkb_context {
            context_state: CONTEXT_NEW,
            load_extra_rules_files: flags & RXKB_CONTEXT_LOAD_EXOTIC_RULES != 0,
            models: Vec::new(),
            layouts: Vec::new(),
            option_groups: Vec::new(),
            includes: Vec::new(),
            log_fn: Some(default_log_fn as fn(&rxkb_context, rxkb_log_level, &str)),
            log_level: RXKB_LOG_LEVEL_ERROR,
        });

        if let Ok(env) = std::env::var("RXKB_LOG_LEVEL") {
            ctx.log_level = log_level_from_str(&env);
        }

        if flags & !valid_flags != 0 {
            rxkb_logf!(
                &*ctx,
                RXKB_LOG_LEVEL_ERROR,
                "{}: Invalid context flags: 0x{:x}\n",
                "rxkb_context_new",
                flags & !valid_flags,
            );
            return None;
        }

        if flags & RXKB_CONTEXT_NO_DEFAULT_INCLUDES == 0 && !ctx.include_path_append_default() {
            rxkb_logf!(
                &*ctx,
                RXKB_LOG_LEVEL_ERROR,
                "[XKB-{:03}] Failed to add any default include path (default system path: {})\n",
                XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as i32,
                "/usr/share/xkeyboard-config-2",
            );
            return None;
        }

        Some(ctx)
    }

    pub fn include_path_append(&mut self, path: &str) {
        if self.context_state != CONTEXT_NEW {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
            return;
        }

        match std::fs::metadata(path) {
            Err(e) => {
                rxkb_logf!(
                    self,
                    RXKB_LOG_LEVEL_INFO,
                    "Include path failed: \"{}\" ({})\n",
                    path,
                    e,
                );
            }
            Ok(m) if !m.is_dir() => {
                rxkb_logf!(
                    self,
                    RXKB_LOG_LEVEL_INFO,
                    "Include path failed: \"{}\" (Not a directory)\n",
                    path,
                );
            }
            Ok(_) => {
                // Validate that rules/evdev.xml exists beneath this path
                let rules_path = format!("{}/rules/evdev.xml", path);
                // We don't actually check the file here — the original just checked the dir.
                let _ = rules_path;
                self.includes.push(path.to_string());
                rxkb_logf!(self, RXKB_LOG_LEVEL_INFO, "Include path added: {}\n", path,);
            }
        }
    }

    pub fn include_path_append_default(&mut self) -> bool {
        if self.context_state != CONTEXT_NEW {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
            return false;
        }

        let mut ret = false;

        let home = std::env::var("HOME").ok();
        let xdg = std::env::var("XDG_CONFIG_HOME").ok();

        if let Some(ref xdg) = xdg {
            let p = format!("{}/xkb", xdg);
            self.include_path_append(&p);
            if self.includes.last().map(|s| s.as_str()) == Some(&p) {
                ret = true;
            }
        } else if let Some(ref home) = home {
            let p = format!("{}/.config/xkb", home);
            self.include_path_append(&p);
            if self.includes.last().map(|s| s.as_str()) == Some(&p) {
                ret = true;
            }
        }

        if let Some(ref home) = home {
            let p = format!("{}/.xkb", home);
            self.include_path_append(&p);
            if self.includes.last().map(|s| s.as_str()) == Some(&p) {
                ret = true;
            }
        }

        // Extra path
        let extra = std::env::var("XKB_CONFIG_EXTRA_PATH").ok();
        let extra_path = extra.as_deref().unwrap_or(DFLT_XKB_CONFIG_EXTRA_PATH_STR);
        self.include_path_append(extra_path);
        if self.includes.last().map(|s| s.as_str()) == Some(extra_path) {
            ret = true;
        }

        // Versioned extensions
        let versioned_ext = std::env::var("XKB_CONFIG_VERSIONED_EXTENSIONS_PATH").ok();
        let versioned_path = versioned_ext
            .as_deref()
            .unwrap_or(DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH_STR);
        self.add_direct_subdirectories(versioned_path);

        // Unversioned extensions
        let unversioned_ext = std::env::var("XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH").ok();
        let unversioned_path = unversioned_ext
            .as_deref()
            .unwrap_or(DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH_STR);
        self.add_direct_subdirectories(unversioned_path);

        // Root path
        let root_env = std::env::var("XKB_CONFIG_ROOT").ok();
        let root_path = root_env.as_deref().unwrap_or(DFLT_XKB_CONFIG_ROOT_STR);
        let prev_len = self.includes.len();
        self.include_path_append(root_path);
        let has_root = self.includes.len() > prev_len;
        if has_root {
            ret = true;
        }

        if !has_root && !root_path.is_empty() {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_WARNING,
                "Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                "/usr/share/X11/xkb",
                root_path,
            );
            let prev_len2 = self.includes.len();
            self.include_path_append(DFLT_XKB_LEGACY_ROOT_STR);
            if self.includes.len() > prev_len2 {
                ret = true;
            }
        }

        ret
    }

    fn add_direct_subdirectories(&mut self, path: &str) {
        let meta = match std::fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return,
        };
        if !meta.is_dir() {
            return;
        }

        let dir_entries = match std::fs::read_dir(path) {
            Ok(d) => d,
            Err(_) => return,
        };

        let mut subdirs: Vec<String> = Vec::new();
        for entry in dir_entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let name = entry.file_name();
            let name_str = match name.to_str() {
                Some(s) => s,
                None => continue,
            };
            if name_str == "." || name_str == ".." {
                continue;
            }
            let entry_path = entry.path();
            let entry_meta = match std::fs::metadata(&entry_path) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if !entry_meta.is_dir() {
                continue;
            }
            if let Some(s) = entry_path.to_str() {
                subdirs.push(s.to_string());
            }
        }

        subdirs.sort();
        for p in &subdirs {
            self.include_path_append(p);
        }
    }

    pub fn parse(&mut self, ruleset: &str) -> bool {
        let mut success = false;
        if self.context_state != CONTEXT_NEW {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_ERROR,
                "parse must only be called on a new context\n",
            );
            return false;
        }

        // Iterate includes in reverse order (like the original)
        let includes: Vec<String> = self.includes.clone();
        let mut idx = includes.len();
        while idx > 0 {
            idx -= 1;
            let path_str = &includes[idx];

            let rules_path = format!("{}/rules/{}.xml", path_str, ruleset);
            rxkb_logf!(self, RXKB_LOG_LEVEL_DEBUG, "Parsing {}\n", rules_path);
            if parse_xml_file(self, &rules_path, RXKB_POPULARITY_STANDARD) {
                success = true;
            }

            if self.load_extra_rules_files {
                let extras_path = format!("{}/rules/{}.extras.xml", path_str, ruleset);
                rxkb_logf!(self, RXKB_LOG_LEVEL_DEBUG, "Parsing {}\n", extras_path);
                if parse_xml_file(self, &extras_path, RXKB_POPULARITY_EXOTIC) {
                    success = true;
                }
            }
        }

        self.context_state = if success {
            CONTEXT_PARSED
        } else {
            CONTEXT_FAILED
        };
        success
    }

    pub fn layouts(&self) -> &[rxkb_layout] {
        &self.layouts
    }

    pub fn models(&self) -> &[rxkb_model] {
        &self.models
    }

    pub fn option_groups(&self) -> &[rxkb_option_group] {
        &self.option_groups
    }
}

// ---------------------------------------------------------------------------
// Default paths (as &str)
// ---------------------------------------------------------------------------

const DFLT_XKB_CONFIG_EXTRA_PATH_STR: &str = "/etc/xkb";
const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH_STR: &str =
    "/usr/share/xkeyboard-config-2/extensions";
const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH_STR: &str = "/usr/share/X11/xkb/extensions";
const DFLT_XKB_CONFIG_ROOT_STR: &str = "/usr/share/xkeyboard-config-2";
const DFLT_XKB_LEGACY_ROOT_STR: &str = "/usr/share/X11/xkb";

// ---------------------------------------------------------------------------
// XML parsing helpers (safe)
// ---------------------------------------------------------------------------

fn get_attr<'a>(
    doc: &'a xmloxide::Document,
    node: xmloxide::tree::NodeId,
    name: &str,
) -> Option<&'a str> {
    doc.attributes(node)
        .iter()
        .find(|a| a.name == name)
        .map(|a| a.value.as_str())
}

#[inline]
fn is_node(doc: &xmloxide::Document, node: xmloxide::tree::NodeId, name: &str) -> bool {
    doc.is_element(node) && doc.node_name(node) == Some(name)
}

fn extract_text(doc: &xmloxide::Document, node: xmloxide::tree::NodeId) -> String {
    for child in doc.children(node) {
        if let Some(text) = doc.node_text(child) {
            if !text.is_empty() {
                return text.to_string();
            }
        }
    }
    String::new()
}

fn parse_config_item(
    ctx: &rxkb_context,
    doc: &xmloxide::Document,
    parent: xmloxide::tree::NodeId,
    config: &mut config_item,
) -> bool {
    for ci in doc.children(parent) {
        if is_node(doc, ci, "configItem") {
            if let Some(raw_popularity) = get_attr(doc, ci, "popularity") {
                if raw_popularity == "standard" {
                    config.popularity = RXKB_POPULARITY_STANDARD;
                } else if raw_popularity == "exotic" {
                    config.popularity = RXKB_POPULARITY_EXOTIC;
                } else {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        "xml: invalid popularity attribute: expected 'standard' or 'exotic', got: '{}'\n",
                        raw_popularity,
                    );
                }
            }
            if let Some(raw_layout_specific) = get_attr(doc, ci, "layout-specific") {
                if raw_layout_specific == "true" {
                    config.layout_specific = true;
                }
            }
            for node in doc.children(ci) {
                if is_node(doc, node, "name") {
                    config.name = extract_text(doc, node);
                } else if is_node(doc, node, "description") {
                    config.description = extract_text(doc, node);
                } else if is_node(doc, node, "shortDescription") {
                    config.brief = extract_text(doc, node);
                } else if is_node(doc, node, "vendor") {
                    config.vendor = extract_text(doc, node);
                }
            }
            if config.name.is_empty() {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_ERROR,
                    "xml: missing required element 'name'\n",
                );
                return false;
            }
            return true;
        }
    }
    false
}

fn parse_model(
    ctx: &mut rxkb_context,
    doc: &xmloxide::Document,
    model: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    let mut config = config_item {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if parse_config_item(ctx, doc, model, &mut config) {
        // Check for duplicate
        if ctx.models.iter().any(|m| m.name == config.name) {
            return;
        }
        ctx.models.push(rxkb_model {
            name: std::mem::take(&mut config.name),
            description: std::mem::take(&mut config.description),
            vendor: std::mem::take(&mut config.vendor),
            popularity: config.popularity,
        });
    }
}

fn parse_language_list(
    doc: &xmloxide::Document,
    language_list: xmloxide::tree::NodeId,
    iso639s: &mut Vec<String>,
) {
    for node in doc.children(language_list) {
        if is_node(doc, node, "iso639Id") {
            let s = extract_text(doc, node);
            if !s.is_empty() && s.len() == 3 {
                iso639s.push(s);
            }
        }
    }
}

fn parse_country_list(
    doc: &xmloxide::Document,
    country_list: xmloxide::tree::NodeId,
    iso3166s: &mut Vec<String>,
) {
    for node in doc.children(country_list) {
        if is_node(doc, node, "iso3166Id") {
            let s = extract_text(doc, node);
            if !s.is_empty() && s.len() == 2 {
                iso3166s.push(s);
            }
        }
    }
}

fn parse_variant(
    ctx: &mut rxkb_context,
    parent_layout_idx: usize,
    doc: &xmloxide::Document,
    variant: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    let mut config = config_item {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if !parse_config_item(ctx, doc, variant, &mut config) {
        return;
    }

    let parent_name = ctx.layouts[parent_layout_idx].name.clone();

    // Check for duplicate
    let exists = ctx
        .layouts
        .iter()
        .any(|v| v.variant == config.name && v.name == parent_name);
    if exists {
        return;
    }

    let parent_brief = ctx.layouts[parent_layout_idx].brief.clone();
    let brief = if config.brief.is_empty() {
        parent_brief
    } else {
        std::mem::take(&mut config.brief)
    };

    let mut new_layout = rxkb_layout {
        name: parent_name,
        variant: std::mem::take(&mut config.name),
        description: std::mem::take(&mut config.description),
        brief,
        popularity: config.popularity,
        iso639s: Vec::new(),
        iso3166s: Vec::new(),
    };

    // Parse language/country lists from variant's configItem
    for ci in doc.children(variant) {
        if is_node(doc, ci, "configItem") {
            let mut found_language_list = false;
            let mut found_country_list = false;
            for node in doc.children(ci) {
                if is_node(doc, node, "languageList") {
                    parse_language_list(doc, node, &mut new_layout.iso639s);
                    found_language_list = true;
                }
                if is_node(doc, node, "countryList") {
                    parse_country_list(doc, node, &mut new_layout.iso3166s);
                    found_country_list = true;
                }
            }
            // Inherit from parent if not found
            if !found_language_list {
                new_layout.iso639s = ctx.layouts[parent_layout_idx].iso639s.clone();
            }
            if !found_country_list {
                new_layout.iso3166s = ctx.layouts[parent_layout_idx].iso3166s.clone();
            }
        }
    }

    ctx.layouts.push(new_layout);
}

fn parse_layout(
    ctx: &mut rxkb_context,
    doc: &xmloxide::Document,
    layout: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    let mut config = config_item {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if !parse_config_item(ctx, doc, layout, &mut config) {
        return;
    }

    // Find existing layout with same name and empty variant
    let existing_idx = ctx
        .layouts
        .iter()
        .position(|el| el.name == config.name && el.variant.is_empty());
    let layout_idx;

    if let Some(idx) = existing_idx {
        layout_idx = idx;
        // Layout already exists, don't overwrite
    } else {
        ctx.layouts.push(rxkb_layout {
            name: std::mem::take(&mut config.name),
            variant: String::new(),
            description: std::mem::take(&mut config.description),
            brief: std::mem::take(&mut config.brief),
            popularity: config.popularity,
            iso639s: Vec::new(),
            iso3166s: Vec::new(),
        });
        layout_idx = ctx.layouts.len() - 1;
    }

    // Parse variants and language/country lists
    for node in doc.children(layout) {
        if is_node(doc, node, "variantList") {
            for vnode in doc.children(node) {
                if is_node(doc, vnode, "variant") {
                    parse_variant(ctx, layout_idx, doc, vnode, popularity);
                }
            }
        }
        if existing_idx.is_none() && is_node(doc, node, "configItem") {
            for ll in doc.children(node) {
                if is_node(doc, ll, "languageList") {
                    parse_language_list(doc, ll, &mut ctx.layouts[layout_idx].iso639s);
                }
                if is_node(doc, ll, "countryList") {
                    parse_country_list(doc, ll, &mut ctx.layouts[layout_idx].iso3166s);
                }
            }
        }
    }
}

fn parse_option(
    ctx: &mut rxkb_context,
    group_idx: usize,
    doc: &xmloxide::Document,
    option: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    let mut config = config_item {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if parse_config_item(ctx, doc, option, &mut config) {
        // Check for duplicate
        if ctx.option_groups[group_idx]
            .options
            .iter()
            .any(|o| o.name == config.name)
        {
            return;
        }
        ctx.option_groups[group_idx].options.push(rxkb_option {
            name: std::mem::take(&mut config.name),
            brief: String::new(),
            description: std::mem::take(&mut config.description),
            popularity: config.popularity,
            layout_specific: config.layout_specific,
        });
    }
}

fn parse_group(
    ctx: &mut rxkb_context,
    doc: &xmloxide::Document,
    group: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    let mut config = config_item {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if !parse_config_item(ctx, doc, group, &mut config) {
        return;
    }

    let existing_idx = ctx
        .option_groups
        .iter()
        .position(|el| el.name == config.name);
    let group_idx;

    if let Some(idx) = existing_idx {
        group_idx = idx;
    } else {
        let mut og = rxkb_option_group {
            allow_multiple: false,
            options: Vec::new(),
            name: std::mem::take(&mut config.name),
            description: std::mem::take(&mut config.description),
            popularity: config.popularity,
        };
        if let Some(multiple) = get_attr(doc, group, "allowMultipleSelection") {
            if multiple == "true" {
                og.allow_multiple = true;
            }
        }
        ctx.option_groups.push(og);
        group_idx = ctx.option_groups.len() - 1;
    }

    for node in doc.children(group) {
        if is_node(doc, node, "option") {
            parse_option(ctx, group_idx, doc, node, popularity);
        }
    }
}

// ---------------------------------------------------------------------------
// DTD
// ---------------------------------------------------------------------------

const XKBCONFIG_DTD: &str = "\
<!ELEMENT xkbConfigRegistry (modelList?, layoutList?, optionList?)>\n\
<!ATTLIST xkbConfigRegistry version CDATA \"1.1\">\n\
<!ELEMENT modelList (model*)>\n\
<!ELEMENT model (configItem)>\n\
<!ELEMENT layoutList (layout*)>\n\
<!ELEMENT layout (configItem,  variantList?)>\n\
<!ELEMENT optionList (group*)>\n\
<!ELEMENT variantList (variant*)>\n\
<!ELEMENT variant (configItem)>\n\
<!ELEMENT group (configItem, option*)>\n\
<!ATTLIST group allowMultipleSelection (true|false) \"false\">\n\
<!ELEMENT option (configItem)>\n\
<!ELEMENT configItem (name, shortDescription?, description?, vendor?, countryList?, languageList?, hwList?)>\n\
<!ATTLIST configItem layout-specific (true|false) \"false\">\n\
<!ATTLIST configItem popularity (standard|exotic) #IMPLIED>\n\
<!ELEMENT name (#PCDATA)>\n\
<!ELEMENT shortDescription (#PCDATA)>\n\
<!ELEMENT description (#PCDATA)>\n\
<!ELEMENT vendor (#PCDATA)>\n\
<!ELEMENT countryList (iso3166Id+)>\n\
<!ELEMENT iso3166Id (#PCDATA)>\n\
<!ELEMENT languageList (iso639Id+)>\n\
<!ELEMENT iso639Id (#PCDATA)>\n\
<!ELEMENT hwList (hwId+)>\n\
<!ELEMENT hwId (#PCDATA)>";

// ---------------------------------------------------------------------------
// XML file parsing
// ---------------------------------------------------------------------------

fn parse_xml_file(ctx: &mut rxkb_context, path: &str, popularity: rxkb_popularity) -> bool {
    let mut doc = match xmloxide::Document::parse_file(path) {
        Ok(d) => d,
        Err(_) => return false,
    };
    // Validate
    {
        let dtd = match xmloxide::validation::dtd::parse_dtd(XKBCONFIG_DTD) {
            Ok(dtd) => dtd,
            Err(_) => {
                rxkb_logf!(ctx, RXKB_LOG_LEVEL_ERROR, "Failed to load DTD\n");
                return false;
            }
        };
        let result = xmloxide::validation::dtd::validate(&mut doc, &dtd);
        if !result.is_valid {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "XML error: failed to validate document at {}\n",
                path,
            );
            return false;
        }
    }
    let root = match doc.root_element() {
        Some(r) => r,
        None => return false,
    };
    for node in doc.children(root) {
        if is_node(&doc, node, "modelList") {
            for mnode in doc.children(node) {
                if is_node(&doc, mnode, "model") {
                    parse_model(ctx, &doc, mnode, popularity);
                }
            }
        } else if is_node(&doc, node, "layoutList") {
            for lnode in doc.children(node) {
                if is_node(&doc, lnode, "layout") {
                    parse_layout(ctx, &doc, lnode, popularity);
                }
            }
        } else if is_node(&doc, node, "optionList") {
            for onode in doc.children(node) {
                if is_node(&doc, onode, "group") {
                    parse_group(ctx, &doc, onode, popularity);
                }
            }
        }
    }
    true
}
