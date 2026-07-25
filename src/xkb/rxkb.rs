//! Registry (rxkb) subsystem — pure-Rust XML parser for XKB registry files.
//!
//! Only compiled when `feature = "testing"` is enabled (dev-dependency).
//! Provides layout/model/option enumeration from `rules/*.xml` registry files.

// ── Rxkb types ──────────────────────────────────────────────────────

pub(crate) type RxkbPopularity = u32;
pub(crate) const RXKB_POPULARITY_EXOTIC: RxkbPopularity = 2;
pub(crate) const RXKB_POPULARITY_STANDARD: RxkbPopularity = 1;

pub(crate) type RxkbContextFlags = u32;
pub(crate) const RXKB_CONTEXT_NO_SECURE_GETENV: RxkbContextFlags = 4;
pub(crate) const RXKB_CONTEXT_LOAD_EXOTIC_RULES: RxkbContextFlags = 2;
pub(crate) const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: RxkbContextFlags = 1;
pub(crate) const RXKB_CONTEXT_NO_FLAGS: RxkbContextFlags = 0;

pub(crate) type ContextState = u32;
pub(crate) const CONTEXT_FAILED: ContextState = 2;
pub(crate) const CONTEXT_PARSED: ContextState = 1;
pub(crate) const CONTEXT_NEW: ContextState = 0;

pub(crate) struct RxkbContext {
    pub(crate) context_state: ContextState,
    pub(crate) load_extra_rules_files: bool,
    pub(crate) models: Vec<RxkbModel>,
    pub(crate) layouts: Vec<RxkbLayout>,
    pub(crate) option_groups: Vec<RxkbOptionGroup>,
    pub(crate) includes: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct RxkbModel {
    pub(crate) name: String,
}

pub(crate) struct RxkbLayout {
    pub(crate) name: String,
    pub(crate) brief: String,
    pub(crate) variant: String,
}

pub(crate) struct RxkbOptionGroup {
    pub(crate) options: Vec<RxkbOption>,
    pub(crate) name: String,
}

#[derive(Clone)]
pub(crate) struct RxkbOption {
    pub(crate) name: String,
}

#[derive(Clone, Default)]
pub(crate) struct ConfigItem {
    pub(crate) name: String,
    pub(crate) brief: String,
    pub(crate) popularity: RxkbPopularity,
}

// ── Safe wrapper ────────────────────────────────────────────────────

/// Safe wrapper around RxkbContext for keyboard layout registry
pub(crate) struct RegistryContext {
    inner: Box<RxkbContext>,
}

impl RegistryContext {
    pub(crate) fn new() -> Option<Self> {
        let inner = RxkbContext::new(RXKB_CONTEXT_NO_FLAGS)?;
        Some(RegistryContext { inner })
    }

    pub(crate) fn include_path_append_default(&mut self) {
        self.inner.include_path_append_default();
    }

    pub(crate) fn parse(&mut self, ruleset: &str) -> bool {
        self.inner.parse(ruleset)
    }

    pub(crate) fn layouts(&self) -> impl Iterator<Item = &RxkbLayout> {
        self.inner.layouts().iter()
    }
}

// ── Accessors ───────────────────────────────────────────────────────

impl RxkbLayout {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn variant(&self) -> &str {
        &self.variant
    }
}

// ── Context construction & public API ───────────────────────────────

impl RxkbContext {
    pub(crate) fn new(flags: RxkbContextFlags) -> Option<Box<RxkbContext>> {
        let valid_flags: RxkbContextFlags = RXKB_CONTEXT_NO_DEFAULT_INCLUDES
            | RXKB_CONTEXT_LOAD_EXOTIC_RULES
            | RXKB_CONTEXT_NO_SECURE_GETENV;

        let mut ctx = Box::new(RxkbContext {
            context_state: CONTEXT_NEW,
            load_extra_rules_files: flags & RXKB_CONTEXT_LOAD_EXOTIC_RULES != 0,
            models: Vec::new(),
            layouts: Vec::new(),
            option_groups: Vec::new(),
            includes: Vec::new(),
        });

        if flags & !valid_flags != 0 {
            return None;
        }

        if flags & RXKB_CONTEXT_NO_DEFAULT_INCLUDES == 0 && !ctx.include_path_append_default() {
            return None;
        }

        Some(ctx)
    }

    pub(crate) fn include_path_append(&mut self, path: &str) {
        if self.context_state != CONTEXT_NEW {
            return;
        }

        if std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false) {
            self.includes.push(path.to_string());
        }
    }

    pub(crate) fn include_path_append_default(&mut self) -> bool {
        if self.context_state != CONTEXT_NEW {
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

    pub(crate) fn parse(&mut self, ruleset: &str) -> bool {
        let mut success = false;
        if self.context_state != CONTEXT_NEW {
            return false;
        }

        // Iterate includes in reverse order (like the original)
        let includes: Vec<String> = self.includes.clone();
        for idx in (0..includes.len()).rev() {
            let path_str = &includes[idx];

            let rules_path = format!("{}/rules/{}.xml", path_str, ruleset);
            if parse_xml_file(self, &rules_path, RXKB_POPULARITY_STANDARD) {
                success = true;
            }

            if self.load_extra_rules_files {
                let extras_path = format!("{}/rules/{}.extras.xml", path_str, ruleset);
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

    pub(crate) fn layouts(&self) -> &[RxkbLayout] {
        &self.layouts
    }
}

// ── Default paths (as &str) ─────────────────────────────────────────

const DFLT_XKB_CONFIG_EXTRA_PATH_STR: &str = "/etc/xkb";
const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH_STR: &str =
    "/usr/share/xkeyboard-config-2/extensions";
const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH_STR: &str = "/usr/share/X11/xkb/extensions";
const DFLT_XKB_CONFIG_ROOT_STR: &str = "/usr/share/xkeyboard-config-2";
const DFLT_XKB_LEGACY_ROOT_STR: &str = "/usr/share/X11/xkb";

// ── XML parsing helpers ─────────────────────────────────────────────

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
    doc: &xmloxide::Document,
    parent: xmloxide::tree::NodeId,
    config: &mut ConfigItem,
) -> bool {
    for ci in doc.children(parent) {
        if is_node(doc, ci, "configItem") {
            if let Some(raw_popularity) = get_attr(doc, ci, "popularity") {
                if raw_popularity == "standard" {
                    config.popularity = RXKB_POPULARITY_STANDARD;
                } else if raw_popularity == "exotic" {
                    config.popularity = RXKB_POPULARITY_EXOTIC;
                }
            }
            for node in doc.children(ci) {
                if is_node(doc, node, "name") {
                    config.name = extract_text(doc, node);
                } else if is_node(doc, node, "shortDescription") {
                    config.brief = extract_text(doc, node);
                }
            }
            if config.name.is_empty() {
                return false;
            }
            return true;
        }
    }
    false
}

fn parse_model(
    ctx: &mut RxkbContext,
    doc: &xmloxide::Document,
    model: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        popularity,
        ..ConfigItem::default()
    };
    if parse_config_item(doc, model, &mut config) {
        // Check for duplicate
        if ctx.models.iter().any(|m| m.name == config.name) {
            return;
        }
        ctx.models.push(RxkbModel {
            name: std::mem::take(&mut config.name),
        });
    }
}

fn parse_variant(
    ctx: &mut RxkbContext,
    parent_layout_idx: usize,
    doc: &xmloxide::Document,
    variant: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        popularity,
        ..ConfigItem::default()
    };
    if !parse_config_item(doc, variant, &mut config) {
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

    let new_layout = RxkbLayout {
        name: parent_name,
        variant: std::mem::take(&mut config.name),
        brief,
    };

    ctx.layouts.push(new_layout);
}

fn parse_layout(
    ctx: &mut RxkbContext,
    doc: &xmloxide::Document,
    layout: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        popularity,
        ..ConfigItem::default()
    };
    if !parse_config_item(doc, layout, &mut config) {
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
        ctx.layouts.push(RxkbLayout {
            name: std::mem::take(&mut config.name),
            variant: String::new(),
            brief: std::mem::take(&mut config.brief),
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
    }
}

fn parse_option(
    ctx: &mut RxkbContext,
    group_idx: usize,
    doc: &xmloxide::Document,
    option: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        popularity,
        ..ConfigItem::default()
    };
    if parse_config_item(doc, option, &mut config) {
        // Check for duplicate
        if ctx.option_groups[group_idx]
            .options
            .iter()
            .any(|o| o.name == config.name)
        {
            return;
        }
        ctx.option_groups[group_idx].options.push(RxkbOption {
            name: std::mem::take(&mut config.name),
        });
    }
}

fn parse_group(
    ctx: &mut RxkbContext,
    doc: &xmloxide::Document,
    group: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        popularity,
        ..ConfigItem::default()
    };
    if !parse_config_item(doc, group, &mut config) {
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
        let og = RxkbOptionGroup {
            options: Vec::new(),
            name: std::mem::take(&mut config.name),
        };
        ctx.option_groups.push(og);
        group_idx = ctx.option_groups.len() - 1;
    }

    for node in doc.children(group) {
        if is_node(doc, node, "option") {
            parse_option(ctx, group_idx, doc, node, popularity);
        }
    }
}

// ── DTD ─────────────────────────────────────────────────────────────

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

// ── XML file parsing ────────────────────────────────────────────────

fn parse_xml_file(ctx: &mut RxkbContext, path: &str, popularity: RxkbPopularity) -> bool {
    let mut doc = match xmloxide::Document::parse_file(path) {
        Ok(d) => d,
        Err(_) => return false,
    };
    // Validate
    {
        let dtd = match xmloxide::validation::dtd::parse_dtd(XKBCONFIG_DTD) {
            Ok(dtd) => dtd,
            Err(_) => {
                return false;
            }
        };
        let result = xmloxide::validation::dtd::validate(&mut doc, &dtd);
        if !result.is_valid {
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
