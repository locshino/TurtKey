//
// TurtKey - Application GUI (eframe / egui)
// Based on OpenKey UI design
// Clean, professional multi-language UI (Zero emojis)
//

pub mod header;
pub mod tab_about;
pub mod tab_blacklist;
pub mod tab_convert;
pub mod tab_general;
pub mod tab_macro;
pub mod tab_playground;
pub mod tab_plugins;

pub use tab_playground::egui_key_to_char;

use crate::engine::convert::CaseTransform;
use crate::engine::types::{CodeTable, HotkeyConfig};
use crate::engine::VietnameseEngine;
use crate::i18n::UiLanguage;
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    General,
    Macro,
    Convert,
    Playground,
    Blacklist,
    About,
    Plugins,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyTarget {
    Language,
    Mode,
    Convert,
}

pub struct TurtKeyApp {
    pub engine: VietnameseEngine,
    pub active_tab: ActiveTab,
    pub ui_lang: UiLanguage,
    pub language_hotkey: HotkeyConfig,
    pub mode_hotkey: HotkeyConfig,
    pub convert_hotkey: HotkeyConfig,
    pub recording_target: Option<HotkeyTarget>,
    pub prev_modifiers: egui::Modifiers,

    // Macro tab states
    pub new_macro_key: String,
    pub new_macro_val: String,
    pub macro_status_msg: Option<String>,

    // Convert tool states
    pub convert_source_text: String,
    pub convert_result_text: String,
    pub convert_from_code: CodeTable,
    pub convert_to_code: CodeTable,
    pub convert_case_transform: CaseTransform,
    pub convert_remove_marks: bool,
    pub convert_status_msg: Option<String>,

    // Playground interactive typing states
    pub playground_text: String,
    pub playground_last_event: String,

    // Extensible Plugin Systems
    pub i18n_mgr: crate::plugins::I18nManager,
    pub engine_plugins: crate::plugins::EnginePluginManager,
    pub plugin_custom_json: String,
    pub plugin_status_msg: Option<String>,

    // Excluded Apps (Blacklist) states
    pub blacklist_mgr: crate::engine::blacklist::BlacklistManager,
    pub process_monitor: crate::engine::process_monitor::ProcessMonitor,
    pub running_processes: Vec<crate::engine::process_monitor::ProcessInfo>,
    pub blacklist_search_query: String,
    pub blacklist_manual_input: String,
    pub blacklist_status_msg: Option<String>,
    pub simulated_active_process: String,
}

impl Default for TurtKeyApp {
    fn default() -> Self {
        Self {
            engine: VietnameseEngine::default(),
            active_tab: ActiveTab::General,
            ui_lang: UiLanguage::Vietnamese,
            language_hotkey: HotkeyConfig::ctrl_shift(),
            mode_hotkey: HotkeyConfig::alt_m(),
            convert_hotkey: HotkeyConfig::alt_c(),
            recording_target: None,
            prev_modifiers: egui::Modifiers::default(),

            new_macro_key: String::new(),
            new_macro_val: String::new(),
            macro_status_msg: None,

            convert_source_text: "Tiếng Việt là ngôn ngữ có thanh điệu vô cùng phong phú.".to_string(),
            convert_result_text: String::new(),
            convert_from_code: CodeTable::Unicode,
            convert_to_code: CodeTable::Unicode,
            convert_case_transform: CaseTransform::None,
            convert_remove_marks: false,
            convert_status_msg: None,

            playground_text: String::new(),
            playground_last_event: "Sẵn sàng gõ thử nghiệm...".to_string(),

            i18n_mgr: crate::plugins::I18nManager::new(),
            engine_plugins: crate::plugins::EnginePluginManager::new(),
            plugin_custom_json: String::new(),
            plugin_status_msg: None,

            blacklist_mgr: crate::engine::blacklist::BlacklistManager::default(),
            process_monitor: crate::engine::process_monitor::ProcessMonitor::default(),
            running_processes: Vec::new(),
            blacklist_search_query: String::new(),
            blacklist_manual_input: String::new(),
            blacklist_status_msg: None,
            simulated_active_process: String::new(),
        }
    }
}

impl TurtKeyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TurtKeyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Global hotkey check and recording
        if let Some(target) = self.recording_target {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.recording_target = None;
            } else {
                let recorded = ctx.input(|i| {
                    for event in &i.events {
                        if let egui::Event::Key {
                            key,
                            pressed: true,
                            repeat: false,
                            modifiers,
                            ..
                        } = event
                        {
                            if *key != egui::Key::Escape {
                                let ch = egui_key_to_char(*key);
                                let ctrl = modifiers.ctrl;
                                let shift = modifiers.shift;
                                let alt = modifiers.alt;
                                let win = modifiers.mac_cmd || modifiers.command;
                                if ctrl || shift || alt || win || ch.is_some() {
                                    return Some(HotkeyConfig::new(ctrl, shift, alt, win, ch));
                                }
                            }
                        }
                    }
                    None
                });

                if let Some(config) = recorded {
                    match target {
                        HotkeyTarget::Language => self.language_hotkey = config,
                        HotkeyTarget::Mode => self.mode_hotkey = config,
                        HotkeyTarget::Convert => self.convert_hotkey = config,
                    }
                    self.recording_target = None;
                }
            }
        } else {
            let (ctrl, shift, alt, win) = ctx.input(|i| {
                (
                    i.modifiers.ctrl,
                    i.modifiers.shift,
                    i.modifiers.alt,
                    i.modifiers.mac_cmd || i.modifiers.command,
                )
            });

            let mut triggered_lang = false;
            let mut triggered_mode = false;
            let mut triggered_convert = false;

            // Check key-based hotkeys (e.g., Alt+M, Alt+Z, Alt+C)
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key {
                        key,
                        pressed: true,
                        repeat: false,
                        ..
                    } = event
                    {
                        let ch = egui_key_to_char(*key);
                        if self.language_hotkey.matches(ctrl, shift, alt, win, ch) {
                            triggered_lang = true;
                        }
                        if self.mode_hotkey.matches(ctrl, shift, alt, win, ch) {
                            triggered_mode = true;
                        }
                        if self.convert_hotkey.matches(ctrl, shift, alt, win, ch) {
                            triggered_convert = true;
                        }
                    }
                }
            });

            // Check modifier-only hotkeys (e.g., Ctrl+Shift)
            let curr_mods = ctx.input(|i| i.modifiers);
            if curr_mods != self.prev_modifiers {
                let win_mod = curr_mods.mac_cmd || curr_mods.command;
                let prev_win = self.prev_modifiers.mac_cmd || self.prev_modifiers.command;

                if self.language_hotkey.key_char.is_none()
                    && self.language_hotkey.matches(curr_mods.ctrl, curr_mods.shift, curr_mods.alt, win_mod, None)
                    && !self.language_hotkey.matches(self.prev_modifiers.ctrl, self.prev_modifiers.shift, self.prev_modifiers.alt, prev_win, None)
                {
                    triggered_lang = true;
                }
                if self.mode_hotkey.key_char.is_none()
                    && self.mode_hotkey.matches(curr_mods.ctrl, curr_mods.shift, curr_mods.alt, win_mod, None)
                    && !self.mode_hotkey.matches(self.prev_modifiers.ctrl, self.prev_modifiers.shift, self.prev_modifiers.alt, prev_win, None)
                {
                    triggered_mode = true;
                }
                if self.convert_hotkey.key_char.is_none()
                    && self.convert_hotkey.matches(curr_mods.ctrl, curr_mods.shift, curr_mods.alt, win_mod, None)
                    && !self.convert_hotkey.matches(self.prev_modifiers.ctrl, self.prev_modifiers.shift, self.prev_modifiers.alt, prev_win, None)
                {
                    triggered_convert = true;
                }
            }
            self.prev_modifiers = curr_mods;

            if triggered_lang {
                self.engine.vietnamese_enabled = !self.engine.vietnamese_enabled;
            }
            if triggered_mode {
                self.engine.toggle_target_mode();
            }
            if triggered_convert {
                self.active_tab = ActiveTab::Convert;
            }
        }

        // Auto blacklist check: automatically toggle Vietnamese mode if active window is blacklisted
        let active_proc = crate::engine::process_monitor::ProcessMonitor::get_active_process_name();
        let target_proc = if !self.simulated_active_process.is_empty() {
            Some(self.simulated_active_process.as_str())
        } else {
            active_proc.as_deref()
        };
        self.blacklist_mgr.update_active_app(target_proc, &mut self.engine.vietnamese_enabled);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ui.add_space(8.0);
            self.render_tab_bar(ui);
            ui.separator();
            ui.add_space(8.0);

            egui::ScrollArea::vertical().show(ui, |ui| match self.active_tab {
                ActiveTab::General => self.render_general_tab(ui),
                ActiveTab::Macro => self.render_macro_tab(ui),
                ActiveTab::Convert => self.render_convert_tab(ui),
                ActiveTab::Playground => self.render_playground_tab(ui, ctx),
                ActiveTab::Blacklist => self.render_blacklist_tab(ui),
                ActiveTab::About => self.render_about_tab(ui),
                ActiveTab::Plugins => self.render_plugins_tab(ui),
            });
        });
    }
}
