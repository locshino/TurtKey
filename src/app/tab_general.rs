//
// TurtKey - Application GUI: General Settings & Hotkeys Tab
// Clean, professional multi-language UI (Zero emojis)
//

use super::{HotkeyTarget, TurtKeyApp};
use crate::engine::types::{CodeTable, HotkeyConfig, InputType, TargetMode};
use crate::i18n::UiLanguage;
use eframe::egui::{self, Color32, RichText, Ui};

impl TurtKeyApp {
    pub(crate) fn render_general_tab(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.core_settings()).strong().size(15.0));
            ui.add_space(4.0);

            egui::Grid::new("general_grid")
                .num_columns(2)
                .spacing([24.0, 12.0])
                .show(ui, |ui| {
                    ui.label(self.ui_lang.ui_language_label());
                    let mut switch_lang_to: Option<String> = None;
                    egui::ComboBox::from_id_salt("ui_language_combo")
                        .selected_text(self.i18n_mgr.current_language_name())
                        .show_ui(ui, |ui| {
                            for (id, name) in self.i18n_mgr.available_languages() {
                                let is_active = self.i18n_mgr.current_language_id() == id;
                                if ui.selectable_label(is_active, &name).clicked() {
                                    switch_lang_to = Some(id);
                                }
                            }
                        });
                    if let Some(id) = switch_lang_to {
                        self.i18n_mgr.set_language(&id);
                        self.ui_lang = match id.as_str() {
                            "en" => UiLanguage::English,
                            _ => UiLanguage::Vietnamese,
                        };
                    }
                    ui.end_row();

                    ui.label(self.i18n_mgr.t("active_engine_plugin"));
                    let mut switch_engine_to: Option<String> = None;
                    egui::ComboBox::from_id_salt("engine_plugin_combo")
                        .selected_text(self.engine_plugins.active_name())
                        .show_ui(ui, |ui| {
                            for (id, name, _) in self.engine_plugins.available_engines() {
                                let is_active = self.engine_plugins.active_id() == id;
                                if ui.selectable_label(is_active, &name).clicked() {
                                    switch_engine_to = Some(id);
                                }
                            }
                        });
                    if let Some(id) = switch_engine_to {
                        self.engine_plugins.set_active(&id);
                    }
                    ui.end_row();

                    ui.label(self.ui_lang.input_method_label());
                    egui::ComboBox::from_id_salt("input_type_combo")
                        .selected_text(self.engine.input_type.display_name())
                        .show_ui(ui, |ui| {
                            for it in InputType::all() {
                                ui.selectable_value(&mut self.engine.input_type, *it, it.display_name());
                            }
                        });
                    ui.end_row();

                    ui.label(self.ui_lang.code_table_label());
                    egui::ComboBox::from_id_salt("code_table_combo")
                        .selected_text(self.engine.code_table.display_name())
                        .show_ui(ui, |ui| {
                            for ct in CodeTable::all() {
                                ui.selectable_value(&mut self.engine.code_table, *ct, ct.display_name());
                            }
                        });
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.hotkey_section_title()).strong().size(15.0));
            ui.add_space(4.0);

            if let Some(target) = self.recording_target {
                let target_name = match (target, self.ui_lang) {
                    (HotkeyTarget::Language, UiLanguage::Vietnamese) => "Chuyển Tiếng Việt [V] / [E]",
                    (HotkeyTarget::Language, UiLanguage::English) => "Toggle Vietnamese [V] / [E]",
                    (HotkeyTarget::Mode, UiLanguage::Vietnamese) => "Chuyển Win mode / Linux mode",
                    (HotkeyTarget::Mode, UiLanguage::English) => "Toggle Win mode / Linux mode",
                    (HotkeyTarget::Convert, UiLanguage::Vietnamese) => "Mở công cụ Chuyển mã (Convert)",
                    (HotkeyTarget::Convert, UiLanguage::English) => "Open Convert Tool",
                };
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.ui_lang.recording_banner(target_name))
                            .color(Color32::from_rgb(239, 68, 68))
                            .strong(),
                    );
                    if ui.button(self.ui_lang.cancel_btn()).clicked() {
                        self.recording_target = None;
                    }
                });
                ui.label(
                    RichText::new(self.ui_lang.recording_hint())
                        .size(11.0)
                        .weak(),
                );
                ui.add_space(4.0);
            }

            egui::Grid::new("hotkeys_grid")
                .num_columns(3)
                .spacing([16.0, 10.0])
                .show(ui, |ui| {
                    // Row 1: Language Switch
                    ui.label(self.ui_lang.lang_switch_hotkey_label());
                    let lang_btn_text = if self.recording_target == Some(HotkeyTarget::Language) {
                        self.ui_lang.recording_status().to_string()
                    } else {
                        format!("{} {}", self.ui_lang.record_prefix(), self.language_hotkey.display_name_with_lang(self.ui_lang))
                    };
                    if ui.button(lang_btn_text).clicked() {
                        self.recording_target = if self.recording_target == Some(HotkeyTarget::Language) {
                            None
                        } else {
                            Some(HotkeyTarget::Language)
                        };
                    }
                    ui.horizontal(|ui| {
                        if ui.small_button("Ctrl+Shift").clicked() {
                            self.language_hotkey = HotkeyConfig::ctrl_shift();
                        }
                        if ui.small_button("Alt+Z").clicked() {
                            self.language_hotkey = HotkeyConfig::alt_z();
                        }
                        if ui.small_button("Win+Space").clicked() {
                            self.language_hotkey = HotkeyConfig::win_space();
                        }
                    });
                    ui.end_row();

                    // Row 2: Mode Switch (Win mode / Linux mode)
                    ui.label(self.ui_lang.mode_switch_hotkey_label());
                    let mode_btn_text = if self.recording_target == Some(HotkeyTarget::Mode) {
                        self.ui_lang.recording_status().to_string()
                    } else {
                        format!("{} {}", self.ui_lang.record_prefix(), self.mode_hotkey.display_name_with_lang(self.ui_lang))
                    };
                    if ui.button(mode_btn_text).clicked() {
                        self.recording_target = if self.recording_target == Some(HotkeyTarget::Mode) {
                            None
                        } else {
                            Some(HotkeyTarget::Mode)
                        };
                    }
                    ui.horizontal(|ui| {
                        if ui.small_button("Alt+M").clicked() {
                            self.mode_hotkey = HotkeyConfig::alt_m();
                        }
                        if ui.small_button("Ctrl+Alt+L").clicked() {
                            self.mode_hotkey = HotkeyConfig::ctrl_alt_l();
                        }
                    });
                    ui.end_row();

                    // Row 3: Convert Tool
                    ui.label(self.ui_lang.convert_hotkey_label());
                    let conv_btn_text = if self.recording_target == Some(HotkeyTarget::Convert) {
                        self.ui_lang.recording_status().to_string()
                    } else {
                        format!("{} {}", self.ui_lang.record_prefix(), self.convert_hotkey.display_name_with_lang(self.ui_lang))
                    };
                    if ui.button(conv_btn_text).clicked() {
                        self.recording_target = if self.recording_target == Some(HotkeyTarget::Convert) {
                            None
                        } else {
                            Some(HotkeyTarget::Convert)
                        };
                    }
                    ui.horizontal(|ui| {
                        if ui.small_button("Alt+C").clicked() {
                            self.convert_hotkey = HotkeyConfig::alt_c();
                        }
                        if ui.small_button("Ctrl+Shift+C").clicked() {
                            self.convert_hotkey = HotkeyConfig::ctrl_shift_c();
                        }
                    });
                    ui.end_row();
                });

            ui.add_space(4.0);
            ui.label(
                RichText::new(self.ui_lang.hotkey_hint())
                    .size(11.0)
                    .weak(),
            );
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.target_mode_title()).strong().size(15.0));
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.engine.target_mode, TargetMode::Win, self.ui_lang.win_mode_radio());
                ui.radio_value(&mut self.engine.target_mode, TargetMode::Linux, self.ui_lang.linux_mode_radio());
                ui.radio_value(&mut self.engine.target_mode, TargetMode::Game, self.ui_lang.game_mode_radio());
            });

            ui.add_space(2.0);
            ui.label(
                RichText::new(format!("• {}", self.engine.target_mode.description_with_lang(self.ui_lang)))
                    .size(12.0)
                    .color(Color32::from_rgb(0, 180, 136)),
            );
            ui.label(
                RichText::new(self.ui_lang.target_mode_hint(&self.mode_hotkey.display_name_with_lang(self.ui_lang)))
                    .size(11.0)
                    .weak(),
            );
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.typing_options_title()).strong().size(15.0));
            ui.add_space(4.0);

            ui.checkbox(
                &mut self.engine.use_modern_orthography,
                self.ui_lang.opt_modern_orthography(),
            );
            ui.checkbox(
                &mut self.engine.check_spelling,
                self.ui_lang.opt_spelling_check(),
            );
            ui.checkbox(
                &mut self.engine.restore_if_wrong_spelling,
                self.ui_lang.opt_restore_if_wrong(),
            );
            ui.checkbox(
                &mut self.engine.allow_consonant_zfwj,
                self.ui_lang.opt_allow_zfwj(),
            );
            ui.checkbox(
                &mut self.engine.upper_case_first_char,
                self.ui_lang.opt_auto_caps_sentence(),
            );
        });
    }
}
