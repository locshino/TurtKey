//
// TurtKey - Excluded Applications (Blacklist) Manager
// Automatically disables Vietnamese IME for games/apps (e.g. WASD movement)
// and automatically restores Vietnamese mode when switching back to chat/browser.
// Clean, professional architecture (Zero emojis)
//

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExcludedApp {
    pub name: String,
    pub path: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlacklistManager {
    pub enabled: bool,
    pub apps: Vec<ExcludedApp>,
    #[serde(skip)]
    pub active_app_name: Option<String>,
    #[serde(skip)]
    pub was_previously_blacklisted: bool,
    #[serde(skip)]
    pub last_user_vietnamese_state: bool,
}

impl Default for BlacklistManager {
    fn default() -> Self {
        Self {
            enabled: true,
            apps: vec![
                ExcludedApp {
                    name: "cs2.exe".to_string(),
                    path: None,
                    enabled: true,
                },
                ExcludedApp {
                    name: "valorant.exe".to_string(),
                    path: None,
                    enabled: true,
                },
                ExcludedApp {
                    name: "dota2.exe".to_string(),
                    path: None,
                    enabled: true,
                },
                ExcludedApp {
                    name: "league of legends.exe".to_string(),
                    path: None,
                    enabled: true,
                },
                ExcludedApp {
                    name: "genshinimpact.exe".to_string(),
                    path: None,
                    enabled: true,
                },
            ],
            active_app_name: None,
            was_previously_blacklisted: false,
            last_user_vietnamese_state: true,
        }
    }
}

impl BlacklistManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clean and extract canonical process name from raw input or file path
    pub fn clean_process_name(raw: &str) -> Result<String, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err("Tên ứng dụng không được để trống".to_string());
        }
        if trimmed.contains('\0') {
            return Err("Tên ứng dụng chứa ký tự không hợp lệ".to_string());
        }

        // Normalize Windows backslashes so path extraction works cross-platform
        let normalized = trimmed.replace('\\', "/");
        let p = Path::new(&normalized);
        let filename = p
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(trimmed)
            .trim();

        if filename.is_empty() || filename == "." || filename == ".." {
            return Err("Tên file thực thi không hợp lệ".to_string());
        }

        Ok(filename.to_string())
    }

    /// Add a new application to blacklist
    pub fn add_app(&mut self, name_or_path: &str, path: Option<String>) -> Result<(), String> {
        let clean_name = Self::clean_process_name(name_or_path)?;
        let clean_lower = clean_name.to_lowercase();

        // Check if already present (case-insensitive)
        if self.apps.iter().any(|a| a.name.to_lowercase() == clean_lower) {
            return Err(format!("Ứng dụng '{}' đã tồn tại trong danh sách", clean_name));
        }

        self.apps.push(ExcludedApp {
            name: clean_name,
            path,
            enabled: true,
        });

        Ok(())
    }

    /// Remove application from blacklist by index
    pub fn remove_app(&mut self, index: usize) -> bool {
        if index < self.apps.len() {
            self.apps.remove(index);
            true
        } else {
            false
        }
    }

    /// Toggle enabled status of a specific rule
    pub fn toggle_app(&mut self, index: usize) -> bool {
        if let Some(app) = self.apps.get_mut(index) {
            app.enabled = !app.enabled;
            true
        } else {
            false
        }
    }

    /// Check if a process name or path matches any active blacklist rule
    pub fn is_blacklisted(&self, process_name_or_path: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let clean_current = match Self::clean_process_name(process_name_or_path) {
            Ok(c) => c.to_lowercase(),
            Err(_) => return false,
        };

        let current_stem = clean_current
            .strip_suffix(".exe")
            .unwrap_or(&clean_current);

        self.apps.iter().filter(|a| a.enabled).any(|rule| {
            let rule_lower = rule.name.to_lowercase();
            let rule_stem = rule_lower
                .strip_suffix(".exe")
                .unwrap_or(&rule_lower);

            clean_current == rule_lower || current_stem == rule_stem
        })
    }

    /// Check and automatically toggle IME mode based on focused application
    pub fn update_active_app(
        &mut self,
        current_process: Option<&str>,
        vietnamese_enabled: &mut bool,
    ) -> bool {
        let is_target_blacklisted = match current_process {
            Some(proc) if self.enabled => self.is_blacklisted(proc),
            _ => false,
        };

        let mut mode_changed = false;

        if is_target_blacklisted {
            if !self.was_previously_blacklisted {
                // Switching from normal app to game: remember current state and turn OFF
                self.last_user_vietnamese_state = *vietnamese_enabled;
                if *vietnamese_enabled {
                    *vietnamese_enabled = false;
                    mode_changed = true;
                }
                self.was_previously_blacklisted = true;
            }
        } else if self.was_previously_blacklisted {
            // Switching from game back to normal app: restore previous user state
            if self.last_user_vietnamese_state && !*vietnamese_enabled {
                *vietnamese_enabled = true;
                mode_changed = true;
            }
            self.was_previously_blacklisted = false;
        }

        self.active_app_name = current_process.map(|s| s.to_string());
        mode_changed
    }
}
