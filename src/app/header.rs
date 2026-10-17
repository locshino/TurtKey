//
// TurtKey - Application GUI: Header & Tab Bar
// Clean, professional multi-language UI (Zero emojis)
//

use super::{ActiveTab, TurtKeyApp};
use crate::engine::types::TargetMode;
use crate::i18n::UiLanguage;
use eframe::egui::{self, Color32, RichText, Stroke, Ui};

impl TurtKeyApp {
    pub(crate) fn render_header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // App Title (No emoji)
            ui.heading(
                RichText::new(self.ui_lang.title())
                    .size(22.0)
                    .color(Color32::from_rgb(0, 180, 136))
                    .strong(),
            );
            ui.label(RichText::new("v0.1.0").weak().size(12.0));

            // UI Language quick toggle button (cycles through loaded language plugins)
            let lang_btn_text = format!("[{}]", self.i18n_mgr.current_language_id().to_uppercase());
            let lang_toggle = egui::Button::new(
                RichText::new(lang_btn_text)
                    .size(11.0)
                    .strong()
                    .color(Color32::WHITE),
            )
            .fill(Color32::from_rgb(100, 116, 139))
            .stroke(Stroke::NONE)
            .rounding(4.0);

            let hover_tooltip = format!(
                "Ngôn ngữ: {} (Bấm để đổi sang ngôn ngữ tiếp theo)",
                self.i18n_mgr.current_language_name()
            );
            if ui.add(lang_toggle).on_hover_text(hover_tooltip).clicked() {
                let langs = self.i18n_mgr.available_languages();
                if let Some(pos) = langs.iter().position(|(id, _)| id == self.i18n_mgr.current_language_id()) {
                    let next_pos = (pos + 1) % langs.len();
                    let next_id = langs[next_pos].0.clone();
                    self.i18n_mgr.set_language(&next_id);
                    self.ui_lang = match next_id.as_str() {
                        "en" => UiLanguage::English,
                        _ => UiLanguage::Vietnamese,
                    };
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Language toggle switch [V]/[E]
                let btn_text = self.ui_lang.lang_mode_btn(self.engine.vietnamese_enabled);
                let btn_color = if self.engine.vietnamese_enabled {
                    Color32::from_rgb(34, 197, 94)
                } else {
                    Color32::from_rgb(148, 163, 184)
                };

                let btn = egui::Button::new(RichText::new(btn_text).color(Color32::WHITE).strong())
                    .fill(btn_color)
                    .stroke(Stroke::NONE)
                    .rounding(6.0);

                let lang_tooltip = self.ui_lang.lang_mode_tooltip(&self.language_hotkey.display_name_with_lang(self.ui_lang));
                if ui.add(btn).on_hover_text(lang_tooltip).clicked() {
                    self.engine.vietnamese_enabled = !self.engine.vietnamese_enabled;
                }

                // Win mode / Linux mode toggle switch (No emoji)
                let mode_text = self.engine.target_mode.display_name();
                let mode_color = match self.engine.target_mode {
                    TargetMode::Win => Color32::from_rgb(37, 99, 235),
                    TargetMode::Linux => Color32::from_rgb(234, 88, 12),
                    TargetMode::Game => Color32::from_rgb(147, 51, 234),
                };

                let mode_btn = egui::Button::new(RichText::new(mode_text).color(Color32::WHITE).strong())
                    .fill(mode_color)
                    .stroke(Stroke::NONE)
                    .rounding(6.0);

                let mode_tooltip = self.ui_lang.target_mode_tooltip(&self.mode_hotkey.display_name_with_lang(self.ui_lang));
                if ui.add(mode_btn).on_hover_text(mode_tooltip).clicked() {
                    self.engine.toggle_target_mode();
                }

                ui.label(
                    RichText::new(format!(
                        "{} | {}",
                        self.engine.input_type.display_name(),
                        self.engine.code_table.display_name()
                    ))
                    .size(12.0)
                    .weak(),
                );
            });
        });
    }

    pub(crate) fn render_tab_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let tabs = [
                (ActiveTab::General, self.ui_lang.tab_general()),
                (ActiveTab::Macro, self.ui_lang.tab_macro()),
                (ActiveTab::Convert, self.ui_lang.tab_convert()),
                (ActiveTab::Playground, self.ui_lang.tab_playground()),
                (ActiveTab::Blacklist, self.ui_lang.tab_blacklist()),
                (ActiveTab::About, self.ui_lang.tab_about()),
                (ActiveTab::Plugins, self.i18n_mgr.t("tab_plugins")),
            ];

            for (tab, label) in tabs {
                let is_selected = self.active_tab == tab;
                let text = if is_selected {
                    RichText::new(label).strong().color(Color32::from_rgb(0, 180, 136))
                } else {
                    RichText::new(label)
                };

                if ui.selectable_label(is_selected, text).clicked() {
                    self.active_tab = tab;
                }
            }
        });
    }
}
