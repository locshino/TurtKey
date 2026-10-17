//
// TurtKey - Application GUI: About Tab
// Clean, professional multi-language UI (Zero emojis)
//

use super::TurtKeyApp;
use eframe::egui::{Color32, RichText, Ui};

impl TurtKeyApp {
    pub(crate) fn render_about_tab(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(16.0);
            ui.heading(
                RichText::new(self.ui_lang.title())
                    .size(28.0)
                    .color(Color32::from_rgb(0, 180, 136))
                    .strong(),
            );
            ui.label(RichText::new(self.ui_lang.about_subtitle()).size(16.0));
            ui.label(RichText::new(self.ui_lang.version_label("0.1.0")).weak());
            ui.add_space(12.0);

            ui.label(self.ui_lang.credit_openkey());
            ui.label(self.ui_lang.license_text());
            ui.add_space(12.0);

            ui.group(|ui| {
                ui.set_width(450.0);
                ui.label(RichText::new(self.ui_lang.highlights_title()).strong());
                for line in self.ui_lang.highlights() {
                    ui.label(*line);
                }
            });
        });
    }
}
