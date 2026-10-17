//
// TurtKey - Macro Processor (Gõ tắt)
// Ported and enhanced from OpenKey Macro.cpp
// Copyright (c) 2019 Mai Vu Tuyen (OpenKey)
// Rust rewrite (c) 2026 TurtKey
//

use crate::engine::types::TargetMode;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroEntry {
    pub key: String,
    pub replacement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroManager {
    pub enabled: bool,
    pub allow_in_english_mode: bool,
    pub auto_caps: bool,
    #[serde(default)]
    pub selected_mode: TargetMode,
    #[serde(default)]
    pub entries_win: BTreeMap<String, String>,
    #[serde(default)]
    pub entries_linux: BTreeMap<String, String>,
    #[serde(default)]
    pub entries_game: BTreeMap<String, String>,
    // Store in sorted map for deterministic display & backward compatibility
    pub entries: BTreeMap<String, String>,
}

impl Default for MacroManager {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_in_english_mode: false,
            auto_caps: true,
            selected_mode: TargetMode::Win,
            entries_win: BTreeMap::new(),
            entries_linux: BTreeMap::new(),
            entries_game: BTreeMap::new(),
            entries: BTreeMap::new(),
        }
    }
}

impl MacroManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries_for_mode(&self, mode: TargetMode) -> &BTreeMap<String, String> {
        match mode {
            TargetMode::Win => &self.entries_win,
            TargetMode::Linux => &self.entries_linux,
            TargetMode::Game => &self.entries_game,
        }
    }

    pub fn entries_for_mode_mut(&mut self, mode: TargetMode) -> &mut BTreeMap<String, String> {
        match mode {
            TargetMode::Win => &mut self.entries_win,
            TargetMode::Linux => &mut self.entries_linux,
            TargetMode::Game => &mut self.entries_game,
        }
    }

    pub fn add_for_mode(&mut self, key: &str, replacement: &str, mode: TargetMode) -> bool {
        let key = key.trim();
        let replacement = replacement.trim();
        if key.is_empty() || replacement.is_empty() {
            return false;
        }
        self.entries_for_mode_mut(mode).insert(key.to_string(), replacement.to_string());
        if mode == TargetMode::Win {
            self.entries.insert(key.to_string(), replacement.to_string());
        }
        true
    }

    pub fn delete_for_mode(&mut self, key: &str, mode: TargetMode) -> bool {
        let res = self.entries_for_mode_mut(mode).remove(key.trim()).is_some();
        if mode == TargetMode::Win {
            self.entries.remove(key.trim());
        }
        res
    }

    pub fn clear_for_mode(&mut self, mode: TargetMode) {
        self.entries_for_mode_mut(mode).clear();
        if mode == TargetMode::Win {
            self.entries.clear();
        }
    }

    pub fn contains_for_mode(&self, key: &str, mode: TargetMode) -> bool {
        self.entries_for_mode(mode).contains_key(key.trim())
    }

    pub fn get_for_mode(&self, key: &str, mode: TargetMode) -> Option<&String> {
        self.entries_for_mode(mode).get(key.trim())
    }

    pub fn add(&mut self, key: &str, replacement: &str) -> bool {
        self.add_for_mode(key, replacement, self.selected_mode)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.delete_for_mode(key, self.selected_mode)
    }

    pub fn clear(&mut self) {
        self.clear_for_mode(self.selected_mode);
        self.entries.clear();
    }

    pub fn contains(&self, key: &str) -> bool {
        self.contains_for_mode(key, self.selected_mode)
            || self.entries.contains_key(key.trim())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.get_for_mode(key, self.selected_mode)
            .or_else(|| self.entries.get(key.trim()))
    }

    /// Lookup macro for specific mode with auto-capitalization:
    pub fn lookup_for_mode(&self, typed_word: &str, mode: TargetMode) -> Option<String> {
        if !self.enabled || typed_word.is_empty() {
            return None;
        }

        let map = self.entries_for_mode(mode);
        if let Some(val) = Self::lookup_in_map(map, typed_word, self.auto_caps) {
            return Some(val);
        }

        // For Win / Linux mode, if not found in mode table, check legacy/shared entries
        if mode != TargetMode::Game {
            Self::lookup_in_map(&self.entries, typed_word, self.auto_caps)
        } else {
            None
        }
    }

    /// Lookup macro with auto-capitalization (for selected_mode or shared entries)
    pub fn lookup(&self, typed_word: &str) -> Option<String> {
        self.lookup_for_mode(typed_word, self.selected_mode)
    }

    fn lookup_in_map(
        map: &BTreeMap<String, String>,
        typed_word: &str,
        auto_caps: bool,
    ) -> Option<String> {
        // Try exact match first
        if let Some(val) = map.get(typed_word) {
            return Some(val.clone());
        }

        // If auto caps enabled, try case-insensitive lookup
        if auto_caps {
            let lower = typed_word.to_lowercase();
            if let Some(val) = map.get(&lower) {
                let is_all_caps = typed_word
                    .chars()
                    .all(|c| !c.is_alphabetic() || c.is_uppercase())
                    && typed_word.chars().any(|c| c.is_alphabetic());

                if is_all_caps {
                    return Some(val.to_uppercase());
                }

                let mut chars = typed_word.chars();
                let first_is_caps = chars.next().map(|c| c.is_uppercase()).unwrap_or(false);
                let rest_is_lower = chars.all(|c| !c.is_alphabetic() || c.is_lowercase());

                if first_is_caps && rest_is_lower {
                    let mut val_chars = val.chars();
                    if let Some(first) = val_chars.next() {
                        let capitalized =
                            first.to_uppercase().collect::<String>() + val_chars.as_str();
                        return Some(capitalized);
                    }
                }

                return Some(val.clone());
            }
        }

        None
    }

    /// Export macros for specific mode to OpenKey text format (`key:content` or `key\tcontent`)
    pub fn export_mode_to_file<P: AsRef<Path>>(&self, path: P, mode: TargetMode) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for (k, v) in self.entries_for_mode(mode) {
            writeln!(file, "{}:{}", k, v)?;
        }
        Ok(())
    }

    /// Import macros from text file for specific mode
    pub fn import_mode_to_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        mode: TargetMode,
        append: bool,
    ) -> std::io::Result<usize> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        if !append {
            self.entries_for_mode_mut(mode).clear();
            if mode == TargetMode::Win {
                self.entries.clear();
            }
        }

        let mut count = 0;
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Split by ':' or '\t' or '='
            let parts: Vec<&str> = if let Some(idx) = line.find(':') {
                vec![&line[..idx], &line[idx + 1..]]
            } else if let Some(idx) = line.find('\t') {
                vec![&line[..idx], &line[idx + 1..]]
            } else if let Some(idx) = line.find('=') {
                vec![&line[..idx], &line[idx + 1..]]
            } else {
                continue;
            };

            if parts.len() == 2 {
                let k = parts[0].trim();
                let v = parts[1].trim();
                if !k.is_empty() && !v.is_empty() {
                    self.add_for_mode(k, v, mode);
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// Export macros to OpenKey text format (`key:content` or `key\tcontent`)
    pub fn export_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        self.export_mode_to_file(path, self.selected_mode)
    }

    /// Import macros from text file
    pub fn import_from_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        append: bool,
    ) -> std::io::Result<usize> {
        self.import_mode_to_file(path, self.selected_mode, append)
    }
}
