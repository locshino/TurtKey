//
// TurtKey - Application GUI: Excluded Applications (Blacklist) Tab
// Allows searching running processes, browsing files, and managing excluded games/apps.
// Clean, professional multi-language UI (Zero emojis)
//

use super::TurtKeyApp;
use eframe::egui::{self, Color32, RichText, Ui};

impl TurtKeyApp {
    pub(crate) fn render_blacklist_tab(&mut self, ui: &mut Ui) {
        // Initialize running processes list if empty
        if self.running_processes.is_empty() {
            self.running_processes = self.process_monitor.list_running_processes();
        }

        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.blacklist_title()).strong().size(15.0));
            ui.add_space(4.0);

            ui.checkbox(
                &mut self.blacklist_mgr.enabled,
                self.ui_lang.blacklist_enable_checkbox(),
            );

            ui.add_space(4.0);
            let (status_text, status_color) = if !self.blacklist_mgr.enabled {
                ("Trạng thái: Tính năng tự động loại trừ đang tắt", Color32::from_rgb(148, 163, 184))
            } else if self.blacklist_mgr.was_previously_blacklisted {
                (
                    "Trạng thái: Đang trong ứng dụng loại trừ -> Tự động tắt [V] để phím WASD di chuyển mượt",
                    Color32::from_rgb(234, 88, 12),
                )
            } else {
                (
                    "Trạng thái: Đang trong ứng dụng bình thường -> Tự động duy trì chế độ gõ [V]",
                    Color32::from_rgb(34, 197, 94),
                )
            };

            ui.label(RichText::new(status_text).color(status_color).strong().size(12.0));
        });

        ui.add_space(8.0);

        // Section: Add Application
        ui.group(|ui| {
            ui.label(RichText::new("Thêm ứng dụng vào danh sách loại trừ").strong().size(14.0));
            ui.add_space(4.0);

            // Manual Browse or Direct Input
            ui.horizontal(|ui| {
                ui.label(self.ui_lang.blacklist_manual_hint());
                ui.add(
                    egui::TextEdit::singleline(&mut self.blacklist_manual_input)
                        .desired_width(180.0)
                        .hint_text("VD: cs2.exe hoặc đường dẫn"),
                );

                if ui.button(self.ui_lang.blacklist_browse_btn()).clicked() {
                    let file = rfd::FileDialog::new()
                        .add_filter("Executable", &["exe", "bin", "sh", "*"])
                        .pick_file();

                    if let Some(path) = file {
                        let path_str = path.to_string_lossy().to_string();
                        if let Some(filename) = path.file_name() {
                            let name_str = filename.to_string_lossy().to_string();
                            match self.blacklist_mgr.add_app(&name_str, Some(path_str)) {
                                Ok(_) => {
                                    self.blacklist_status_msg = Some(format!("Đã thêm: {}", name_str));
                                    self.blacklist_manual_input.clear();
                                }
                                Err(err) => {
                                    self.blacklist_status_msg = Some(err);
                                }
                            }
                        }
                    }
                }

                if ui.button(self.ui_lang.blacklist_manual_add_btn()).clicked() {
                    if !self.blacklist_manual_input.is_empty() {
                        match self.blacklist_mgr.add_app(&self.blacklist_manual_input, None) {
                            Ok(_) => {
                                self.blacklist_status_msg = Some(format!("Đã thêm: {}", self.blacklist_manual_input));
                                self.blacklist_manual_input.clear();
                            }
                            Err(err) => {
                                self.blacklist_status_msg = Some(err);
                            }
                        }
                    }
                }
            });

            if let Some(msg) = &self.blacklist_status_msg {
                ui.add_space(2.0);
                ui.label(RichText::new(msg).color(Color32::from_rgb(0, 180, 136)).size(12.0));
            }

            ui.separator();
            ui.add_space(4.0);

            // Search running processes
            ui.horizontal(|ui| {
                ui.label(RichText::new("Tìm trong ứng dụng đang chạy:").strong());
                ui.add(
                    egui::TextEdit::singleline(&mut self.blacklist_search_query)
                        .desired_width(220.0)
                        .hint_text("Lọc theo tên: steam, game, discord..."),
                );

                if ui.button(self.ui_lang.blacklist_refresh_btn()).clicked() {
                    self.running_processes = self.process_monitor.list_running_processes();
                }
            });

            ui.add_space(4.0);
            let query_lower = self.blacklist_search_query.trim().to_lowercase();
            let mut proc_to_add: Option<(String, Option<String>)> = None;

            egui::ScrollArea::vertical()
                .max_height(140.0)
                .id_salt("running_proc_scroll")
                .show(ui, |ui| {
                    egui::Grid::new("running_proc_grid")
                        .striped(true)
                        .num_columns(3)
                        .spacing([18.0, 4.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new("Tên tiến trình (Process)").strong());
                            ui.label(RichText::new("PID").strong());
                            ui.label(RichText::new("Thao tác").strong());
                            ui.end_row();

                            for proc in &self.running_processes {
                                if !query_lower.is_empty() && !proc.name.to_lowercase().contains(&query_lower) {
                                    continue;
                                }

                                let is_already_added = self.blacklist_mgr.is_blacklisted(&proc.name);

                                ui.label(&proc.name);
                                ui.label(proc.pid.to_string());

                                if is_already_added {
                                    ui.label(RichText::new("Đã trong danh sách").weak());
                                } else if ui.small_button("+ Thêm loại trừ").clicked() {
                                    proc_to_add = Some((proc.name.clone(), proc.exe_path.clone()));
                                }
                                ui.end_row();
                            }
                        });
                });

            if let Some((name, path)) = proc_to_add {
                match self.blacklist_mgr.add_app(&name, path) {
                    Ok(_) => self.blacklist_status_msg = Some(format!("Đã thêm: {}", name)),
                    Err(e) => self.blacklist_status_msg = Some(e),
                }
            }
        });

        ui.add_space(8.0);

        // Section: Active Blacklist
        ui.group(|ui| {
            ui.label(RichText::new(self.ui_lang.blacklist_active_list_header()).strong().size(14.0));
            ui.add_space(4.0);

            let mut to_remove: Option<usize> = None;

            egui::ScrollArea::vertical()
                .max_height(160.0)
                .id_salt("blacklist_apps_scroll")
                .show(ui, |ui| {
                    egui::Grid::new("blacklist_apps_grid")
                        .striped(true)
                        .num_columns(4)
                        .spacing([18.0, 6.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new("Kích hoạt").strong());
                            ui.label(RichText::new("Tên ứng dụng").strong());
                            ui.label(RichText::new("Đường dẫn").strong());
                            ui.label(RichText::new("Thao tác").strong());
                            ui.end_row();

                            for (idx, app) in self.blacklist_mgr.apps.iter_mut().enumerate() {
                                ui.checkbox(&mut app.enabled, "");
                                ui.label(RichText::new(&app.name).strong());
                                ui.label(app.path.as_deref().unwrap_or("-"));

                                if ui.button("Xóa").clicked() {
                                    to_remove = Some(idx);
                                }
                                ui.end_row();
                            }
                        });
                });

            if let Some(idx) = to_remove {
                self.blacklist_mgr.remove_app(idx);
            }
        });

        ui.add_space(8.0);

        // Interactive simulation tester for user peace of mind
        ui.group(|ui| {
            ui.label(RichText::new("Thử nghiệm mô phỏng chuyển cửa sổ").strong().size(13.0));
            ui.horizontal(|ui| {
                ui.label("Giả lập cửa sổ đang mở:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.simulated_active_process)
                        .desired_width(160.0)
                        .hint_text("VD: cs2.exe hoặc chrome.exe"),
                );

                if ui.small_button("Giả lập Game (cs2.exe)").clicked() {
                    self.simulated_active_process = "cs2.exe".to_string();
                }
                if ui.small_button("Giả lập Discord").clicked() {
                    self.simulated_active_process = "discord.exe".to_string();
                }
                if ui.small_button("Hủy giả lập (Auto)").clicked() {
                    self.simulated_active_process.clear();
                }
            });
            ui.label(
                RichText::new(self.ui_lang.blacklist_tip())
                    .size(11.0)
                    .weak(),
            );
        });
    }
}
