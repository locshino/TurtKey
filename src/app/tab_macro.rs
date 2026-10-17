//
// TurtKey - Application GUI: Macro / Shorthand Tab
// Clean, professional multi-language UI (Zero emojis)
//

use super::TurtKeyApp;
use eframe::egui::{self, Color32, RichText, Ui};

impl TurtKeyApp {
    pub(crate) fn render_macro_tab(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.macro_settings_title()).strong().size(15.0));
            ui.add_space(4.0);

            ui.checkbox(&mut self.engine.macro_mgr.enabled, self.ui_lang.macro_enabled());
            ui.checkbox(
                &mut self.engine.macro_mgr.allow_in_english_mode,
                self.ui_lang.macro_allow_in_english(),
            );
            ui.checkbox(
                &mut self.engine.macro_mgr.auto_caps,
                self.ui_lang.macro_auto_caps(),
            );
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.quick_consonants_title()).strong().size(15.0));
            ui.add_space(4.0);

            ui.checkbox(
                &mut self.engine.quick_start_consonant,
                self.ui_lang.quick_start_consonant(),
            );
            ui.checkbox(
                &mut self.engine.quick_end_consonant,
                self.ui_lang.quick_end_consonant(),
            );
            ui.checkbox(
                &mut self.engine.quick_telex,
                self.ui_lang.quick_telex(),
            );
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.macro_table_title()).strong().size(15.0));
            ui.add_space(4.0);

            // Mode selector tabs
            ui.horizontal(|ui| {
                ui.label(RichText::new(self.ui_lang.macro_mode_select_label()).strong());
                for mode in crate::engine::types::TargetMode::all() {
                    let is_selected = self.engine.macro_mgr.selected_mode == *mode;
                    let is_active = self.engine.target_mode == *mode;
                    let label = if is_active {
                        format!("{} (Active)", mode.display_name())
                    } else {
                        mode.display_name().to_string()
                    };

                    let text = if is_selected {
                        RichText::new(label).strong().color(Color32::from_rgb(0, 180, 136))
                    } else {
                        RichText::new(label)
                    };

                    if ui.selectable_label(is_selected, text).clicked() {
                        self.engine.macro_mgr.selected_mode = *mode;
                    }
                }
            });

            if self.engine.macro_mgr.selected_mode == crate::engine::types::TargetMode::Game {
                ui.add_space(2.0);
                ui.label(
                    RichText::new(self.ui_lang.macro_mode_game_hint())
                        .size(11.0)
                        .color(Color32::from_rgb(147, 51, 234)),
                );
            }

            ui.add_space(4.0);

            // Add new macro row
            ui.horizontal(|ui| {
                ui.label(self.ui_lang.macro_short_label());
                ui.add(egui::TextEdit::singleline(&mut self.new_macro_key).desired_width(80.0));
                ui.label(self.ui_lang.macro_replace_label());
                ui.add(egui::TextEdit::singleline(&mut self.new_macro_val).desired_width(180.0));

                if ui.button(self.ui_lang.macro_add_btn()).clicked() {
                    let cur_mode = self.engine.macro_mgr.selected_mode;
                    if self.engine.macro_mgr.add_for_mode(&self.new_macro_key, &self.new_macro_val, cur_mode) {
                        self.macro_status_msg = Some(self.ui_lang.macro_added_success(&self.new_macro_key, &self.new_macro_val));
                        self.new_macro_key.clear();
                        self.new_macro_val.clear();
                    } else {
                        self.macro_status_msg = Some(self.ui_lang.macro_empty_error().to_string());
                    }
                }
            });

            if let Some(msg) = &self.macro_status_msg {
                ui.label(RichText::new(msg).color(Color32::from_rgb(0, 180, 136)).size(12.0));
            }

            ui.add_space(6.0);

            // Table of macros
            let mut to_delete: Option<String> = None;
            let cur_mode = self.engine.macro_mgr.selected_mode;
            egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                egui::Grid::new("macro_table_grid")
                    .striped(true)
                    .num_columns(3)
                    .spacing([24.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new(self.ui_lang.col_shortcut()).strong());
                        ui.label(RichText::new(self.ui_lang.col_replacement()).strong());
                        ui.label(RichText::new(self.ui_lang.col_action()).strong());
                        ui.end_row();

                        for (k, v) in self.engine.macro_mgr.entries_for_mode(cur_mode) {
                            ui.label(k);
                            ui.label(v);
                            if ui.button(self.ui_lang.delete_btn()).clicked() {
                                to_delete = Some(k.clone());
                            }
                            ui.end_row();
                        }
                    });
            });

            if let Some(k) = to_delete {
                self.engine.macro_mgr.delete_for_mode(&k, cur_mode);
            }
        });
    }
}
