//
// TurtKey - Dedicated UI Internationalization (i18n) Plugin System
// Clean, extensible language pack architecture (Zero emojis)
// Supports dynamic loading, JSON file imports, and built-in language packs
//

pub mod builtin_packs;

pub use builtin_packs::{
    builtin_english, builtin_french, builtin_german, builtin_japanese, builtin_spanish,
    builtin_vietnamese,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Metadata for a language pack plugin
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguagePluginMeta {
    pub id: String,          // ISO 639-1 code or custom identifier, e.g. "vi", "en", "fr", "ja", "de", "es"
    pub name: String,        // Native display name, e.g. "Tiếng Việt", "English", "Français", "日本語"
    pub author: String,      // Plugin author / translator
    pub version: String,     // Semantic version, e.g. "1.0.0"
}

/// Language pack containing translated UI strings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguagePack {
    pub meta: LanguagePluginMeta,
    pub strings: HashMap<String, String>,
}

impl LanguagePack {
    pub fn new(id: impl Into<String>, name: impl Into<String>, author: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            meta: LanguagePluginMeta {
                id: id.into(),
                name: name.into(),
                author: author.into(),
                version: version.into(),
            },
            strings: HashMap::new(),
        }
    }

    pub fn with_string(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.strings.insert(key.into(), val.into());
        self
    }

    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.strings.insert(key.into(), val.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.strings.get(key).map(|s| s.as_str())
    }

    /// Parse a language pack from JSON string
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Serialize language pack to pretty JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Registry and runtime manager for UI i18n plugins
#[derive(Debug, Clone)]
pub struct I18nManager {
    packs: HashMap<String, LanguagePack>,
    ordered_ids: Vec<String>,
    current_id: String,
    fallback_id: String,
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}

impl I18nManager {
    /// Initialize manager with default language plugins: Vietnamese and English
    pub fn new() -> Self {
        let mut mgr = Self {
            packs: HashMap::new(),
            ordered_ids: Vec::new(),
            current_id: "vi".to_string(),
            fallback_id: "vi".to_string(),
        };

        // Default language plugins: Vietnamese and English only (for simplicity)
        mgr.register_pack(builtin_vietnamese());
        mgr.register_pack(builtin_english());

        mgr
    }

    /// Initialize manager with all extended built-in language plugins (vi, en, fr, ja, de, es)
    pub fn with_all_builtins() -> Self {
        let mut mgr = Self::new();
        mgr.register_pack(builtin_french());
        mgr.register_pack(builtin_japanese());
        mgr.register_pack(builtin_german());
        mgr.register_pack(builtin_spanish());
        mgr
    }

    /// Register a language pack plugin
    pub fn register_pack(&mut self, pack: LanguagePack) {
        let id = pack.meta.id.clone();
        if !self.ordered_ids.contains(&id) {
            self.ordered_ids.push(id.clone());
        }
        self.packs.insert(id, pack);
    }

    /// Load and register a language pack from a JSON string
    pub fn load_pack_from_json(&mut self, json_str: &str) -> Result<String, String> {
        let pack = LanguagePack::from_json(json_str)
            .map_err(|e| format!("Invalid JSON language pack: {}", e))?;
        let id = pack.meta.id.clone();
        self.register_pack(pack);
        Ok(id)
    }

    /// Scan directory and load all *.json language packs
    pub fn load_packs_from_dir<P: AsRef<Path>>(&mut self, dir_path: P) -> Result<usize, std::io::Error> {
        let path = dir_path.as_ref();
        if !path.exists() || !path.is_dir() {
            return Ok(0);
        }

        let mut count = 0;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if let Ok(pack) = LanguagePack::from_json(&content) {
                        self.register_pack(pack);
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }

    /// Switch active language
    pub fn set_language(&mut self, id: &str) -> bool {
        if self.packs.contains_key(id) {
            self.current_id = id.to_string();
            true
        } else {
            false
        }
    }

    pub fn current_language_id(&self) -> &str {
        &self.current_id
    }

    pub fn current_language_name(&self) -> &str {
        self.packs
            .get(&self.current_id)
            .map(|p| p.meta.name.as_str())
            .unwrap_or("Tiếng Việt")
    }

    /// Get list of all available languages: (id, display_name)
    pub fn available_languages(&self) -> Vec<(String, String)> {
        self.ordered_ids
            .iter()
            .filter_map(|id| self.packs.get(id).map(|p| (p.meta.id.clone(), p.meta.name.clone())))
            .collect()
    }

    /// Get total number of registered language packs
    pub fn pack_count(&self) -> usize {
        self.packs.len()
    }

    /// Get metadata for all registered packs
    pub fn all_metadata(&self) -> Vec<LanguagePluginMeta> {
        self.ordered_ids
            .iter()
            .filter_map(|id| self.packs.get(id).map(|p| p.meta.clone()))
            .collect()
    }

    /// Translate key with 3-tier fallback:
    /// 1. Current active pack
    /// 2. Fallback pack (Vietnamese)
    /// 3. Key string itself
    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(pack) = self.packs.get(&self.current_id) {
            if let Some(val) = pack.get(key) {
                return val;
            }
        }
        if let Some(fallback_pack) = self.packs.get(&self.fallback_id) {
            if let Some(val) = fallback_pack.get(key) {
                return val;
            }
        }
        key
    }
}
