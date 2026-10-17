//
// TurtKey - Application GUI: Encoding & Case Conversion Tab
// Clean, professional multi-language UI (Zero emojis)
//

use super::TurtKeyApp;
use crate::engine::convert::{convert_text, CaseTransform, ConvertOptions};
use crate::engine::types::CodeTable;
use crate::i18n::UiLanguage;
use arboard::Clipboard;
use eframe::egui::{self, Color32, RichText, Ui};

impl TurtKeyApp {
    pub(crate) fn render_convert_tab(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.convert_tool_title()).strong().size(15.0));
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label(self.ui_lang.convert_source_label());
                egui::ComboBox::from_id_salt("convert_src_combo")
                    .selected_text(self.convert_from_code.display_name())
                    .show_ui(ui, |ui| {
                        for ct in CodeTable::all() {
                            ui.selectable_value(&mut self.convert_from_code, *ct, ct.display_name());
                        }
                    });

                ui.label(self.ui_lang.convert_dest_label());
                egui::ComboBox::from_id_salt("convert_dst_combo")
                    .selected_text(self.convert_to_code.display_name())
                    .show_ui(ui, |ui| {
                        for ct in CodeTable::all() {
                            ui.selectable_value(&mut self.convert_to_code, *ct, ct.display_name());
                        }
                    });
            });

            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label(self.ui_lang.convert_case_label());
                ui.selectable_value(&mut self.convert_case_transform, CaseTransform::None, self.ui_lang.case_none());
                ui.selectable_value(&mut self.convert_case_transform, CaseTransform::Uppercase, self.ui_lang.case_upper());
                ui.selectable_value(&mut self.convert_case_transform, CaseTransform::Lowercase, self.ui_lang.case_lower());
                ui.selectable_value(&mut self.convert_case_transform, CaseTransform::CapitalizeFirstLetter, self.ui_lang.case_first_letter());
                ui.selectable_value(&mut self.convert_case_transform, CaseTransform::CapitalizeEachWord, self.ui_lang.case_each_word());
            });

            ui.checkbox(&mut self.convert_remove_marks, self.ui_lang.remove_marks_label());
        });

        ui.add_space(8.0);

        ui.label(RichText::new(self.ui_lang.source_text_label()).strong());
        ui.add(
            egui::TextEdit::multiline(&mut self.convert_source_text)
                .desired_rows(4)
                .desired_width(f32::INFINITY),
        );

        ui.horizontal(|ui| {
            if ui.button(RichText::new(self.ui_lang.convert_now_btn()).strong()).clicked() {
                let opts = ConvertOptions {
                    from_code: self.convert_from_code,
                    to_code: self.convert_to_code,
                    case_transform: self.convert_case_transform,
                    remove_marks: self.convert_remove_marks,
                };
                self.convert_result_text = convert_text(&self.convert_source_text, &opts);
                self.convert_status_msg = Some(match self.ui_lang {
                    UiLanguage::Vietnamese => "Chuyển mã thành công!".to_string(),
                    UiLanguage::English => "Conversion successful!".to_string(),
                });
            }

            if ui.button(self.ui_lang.paste_and_convert_btn()).clicked() {
                if let Ok(mut cb) = Clipboard::new() {
                    if let Ok(text) = cb.get_text() {
                        self.convert_source_text = text;
                        let opts = ConvertOptions {
                            from_code: self.convert_from_code,
                            to_code: self.convert_to_code,
                            case_transform: self.convert_case_transform,
                            remove_marks: self.convert_remove_marks,
                        };
                        self.convert_result_text = convert_text(&self.convert_source_text, &opts);
                        self.convert_status_msg = Some(match self.ui_lang {
                            UiLanguage::Vietnamese => "Đã dán và chuyển mã từ Clipboard!".to_string(),
                            UiLanguage::English => "Pasted and converted from Clipboard!".to_string(),
                        });
                    }
                }
            }

            if ui.button(self.ui_lang.copy_result_btn()).clicked() {
                if let Ok(mut cb) = Clipboard::new() {
                    if cb.set_text(&self.convert_result_text).is_ok() {
                        self.convert_status_msg = Some(self.ui_lang.converted_and_copied().to_string());
                    }
                }
            }

            if ui.button(self.ui_lang.clear_btn()).clicked() {
                self.convert_source_text.clear();
                self.convert_result_text.clear();
                self.convert_status_msg = None;
            }
        });

        if let Some(msg) = &self.convert_status_msg {
            ui.label(RichText::new(msg).color(Color32::from_rgb(0, 180, 136)).size(12.0));
        }

        ui.add_space(4.0);
        ui.label(RichText::new(self.ui_lang.result_text_label()).strong());
        ui.add(
            egui::TextEdit::multiline(&mut self.convert_result_text)
                .desired_rows(4)
                .desired_width(f32::INFINITY),
        );
    }
}
