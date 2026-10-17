//
// TurtKey - Application GUI: Plugins Tab
// Clean, professional multi-language UI (Zero emojis)
//

use super::TurtKeyApp;
use crate::i18n::UiLanguage;
use eframe::egui::{self, Color32, RichText, Ui};

impl TurtKeyApp {
    pub(crate) fn render_plugins_tab(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new(self.i18n_mgr.t("plugin_ui_header")).strong().size(15.0));
            ui.label(RichText::new("Các gói ngôn ngữ giao diện (Language Packs) đang nạp:").weak().size(12.0));
            ui.add_space(6.0);

            egui::Grid::new("plugins_ui_grid")
                .num_columns(5)
                .spacing([18.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(RichText::new("Mã ID").strong());
                    ui.label(RichText::new("Tên ngôn ngữ").strong());
                    ui.label(RichText::new("Tác giả").strong());
                    ui.label(RichText::new("Phiên bản").strong());
                    ui.label(RichText::new("Trạng thái").strong());
                    ui.end_row();

                    let mut plugin_tab_switch_lang: Option<String> = None;
                    for meta in self.i18n_mgr.all_metadata() {
                        let is_active = self.i18n_mgr.current_language_id() == meta.id;
                        ui.label(&meta.id);
                        ui.label(&meta.name);
                        ui.label(&meta.author);
                        ui.label(&meta.version);
                        if is_active {
                            ui.label(RichText::new("Đang dùng").color(Color32::from_rgb(34, 197, 94)).strong());
                        } else if ui.button("Kích hoạt").clicked() {
                            plugin_tab_switch_lang = Some(meta.id.clone());
                        }
                        ui.end_row();
                    }
                    if let Some(id) = plugin_tab_switch_lang {
                        self.i18n_mgr.set_language(&id);
                        self.ui_lang = match id.as_str() {
                            "en" => UiLanguage::English,
                            _ => UiLanguage::Vietnamese,
                        };
                    }
                });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new(self.i18n_mgr.t("plugin_engine_header")).strong().size(15.0));
            ui.label(RichText::new("Các bộ gõ nhập liệu độc lập (Input Engine Plugins):").weak().size(12.0));
            ui.add_space(6.0);

            egui::Grid::new("plugins_engine_grid")
                .num_columns(4)
                .spacing([18.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(RichText::new("Mã ID").strong());
                    ui.label(RichText::new("Tên bộ gõ").strong());
                    ui.label(RichText::new("Mô tả quy tắc gõ").strong());
                    ui.label(RichText::new("Thao tác").strong());
                    ui.end_row();

                    let mut plugin_tab_switch_engine: Option<String> = None;
                    for (id, name, desc) in self.engine_plugins.available_engines() {
                        let is_active = self.engine_plugins.active_id() == id;
                        ui.label(&id);
                        ui.label(&name);
                        ui.label(&desc);
                        if is_active {
                            ui.label(RichText::new("Đang dùng").color(Color32::from_rgb(34, 197, 94)).strong());
                        } else if ui.button("Kích hoạt").clicked() {
                            plugin_tab_switch_engine = Some(id);
                        }
                        ui.end_row();
                    }
                    if let Some(id) = plugin_tab_switch_engine {
                        self.engine_plugins.set_active(&id);
                    }
                });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("Nạp Plugin Gói Ngôn ngữ (JSON)").strong().size(15.0));
            ui.label(RichText::new("Dán chuỗi JSON của Language Pack để thêm ngôn ngữ mới ngay lập tức:").weak().size(12.0));
            ui.add_space(6.0);

            ui.add(
                egui::TextEdit::multiline(&mut self.plugin_custom_json)
                    .hint_text("{\n  \"meta\": {\n    \"id\": \"it\",\n    \"name\": \"Italiano\",\n    \"author\": \"Community\",\n    \"version\": \"1.0.0\"\n  },\n  \"strings\": {\n    \"app_title\": \"TurtKey\",\n    \"tab_general\": \"Motore\"\n  }\n}")
                    .desired_rows(6)
                    .desired_width(f32::INFINITY),
            );

            ui.horizontal(|ui| {
                if ui.button(RichText::new("Nạp Plugin JSON").strong()).clicked() {
                    match self.i18n_mgr.load_pack_from_json(&self.plugin_custom_json) {
                        Ok(id) => {
                            self.plugin_status_msg = Some(format!("Đã nạp thành công plugin: {}", id));
                            self.i18n_mgr.set_language(&id);
                        }
                        Err(e) => {
                            self.plugin_status_msg = Some(format!("Lỗi cú pháp JSON: {}", e));
                        }
                    }
                }
                if ui.button("Xóa khung nhập").clicked() {
                    self.plugin_custom_json.clear();
                    self.plugin_status_msg = None;
                }
            });

            if let Some(msg) = &self.plugin_status_msg {
                ui.add_space(4.0);
                ui.label(RichText::new(msg).color(Color32::from_rgb(0, 180, 136)));
            }
        });
    }
}
