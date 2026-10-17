//
// TurtKey - Application GUI: Interactive Playground Tab
// Clean, professional multi-language UI (Zero emojis)
//

use super::TurtKeyApp;
use crate::engine::tables::char_to_key_code;
use crate::engine::types::{HookCodeState, TargetMode, KEY_DELETE};
use crate::i18n::UiLanguage;
use eframe::egui::{self, Color32, RichText, Ui};

pub fn egui_key_to_char(key: egui::Key) -> Option<char> {
    match key {
        egui::Key::Space => Some(' '),
        egui::Key::Num0 => Some('0'),
        egui::Key::Num1 => Some('1'),
        egui::Key::Num2 => Some('2'),
        egui::Key::Num3 => Some('3'),
        egui::Key::Num4 => Some('4'),
        egui::Key::Num5 => Some('5'),
        egui::Key::Num6 => Some('6'),
        egui::Key::Num7 => Some('7'),
        egui::Key::Num8 => Some('8'),
        egui::Key::Num9 => Some('9'),
        egui::Key::A => Some('A'),
        egui::Key::B => Some('B'),
        egui::Key::C => Some('C'),
        egui::Key::D => Some('D'),
        egui::Key::E => Some('E'),
        egui::Key::F => Some('F'),
        egui::Key::G => Some('G'),
        egui::Key::H => Some('H'),
        egui::Key::I => Some('I'),
        egui::Key::J => Some('J'),
        egui::Key::K => Some('K'),
        egui::Key::L => Some('L'),
        egui::Key::M => Some('M'),
        egui::Key::N => Some('N'),
        egui::Key::O => Some('O'),
        egui::Key::P => Some('P'),
        egui::Key::Q => Some('Q'),
        egui::Key::R => Some('R'),
        egui::Key::S => Some('S'),
        egui::Key::T => Some('T'),
        egui::Key::U => Some('U'),
        egui::Key::V => Some('V'),
        egui::Key::W => Some('W'),
        egui::Key::X => Some('X'),
        egui::Key::Y => Some('Y'),
        egui::Key::Z => Some('Z'),
        _ => {
            let name = key.name();
            if name.len() == 1 {
                name.chars().next()
            } else {
                None
            }
        }
    }
}

impl TurtKeyApp {
    pub(crate) fn render_playground_tab(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.group(|ui| {
            ui.label(
                RichText::new(match self.ui_lang {
                    UiLanguage::Vietnamese => "Trình gõ thử nghiệm trực tiếp (Interactive Rust Engine Playground)",
                    UiLanguage::English => "Interactive Rust Engine Playground",
                })
                .strong()
                .size(15.0),
            );
            ui.label(
                match self.ui_lang {
                    UiLanguage::Vietnamese => "Bạn có thể gõ tiếng Việt trực tiếp vào ô bên dưới. Mỗi phím gõ sẽ được nạp qua bộ máy state machine Rust của TurtKey để xử lý đặt dấu, gõ tắt và khôi phục từ.",
                    UiLanguage::English => "You can test Vietnamese typing directly below. Each keystroke passes through the TurtKey Rust state machine for tone placement, macro expansion, and spelling recovery.",
                }
            );
        });

        ui.add_space(8.0);

        // Process key events from egui input
        let events = ctx.input(|i| i.events.clone());
        for ev in events {
            if let egui::Event::Key {
                key,
                pressed,
                modifiers,
                ..
            } = ev
            {
                if pressed {
                    let key_char = match key {
                        egui::Key::A => Some('a'),
                        egui::Key::B => Some('b'),
                        egui::Key::C => Some('c'),
                        egui::Key::D => Some('d'),
                        egui::Key::E => Some('e'),
                        egui::Key::F => Some('f'),
                        egui::Key::G => Some('g'),
                        egui::Key::H => Some('h'),
                        egui::Key::I => Some('i'),
                        egui::Key::J => Some('j'),
                        egui::Key::K => Some('k'),
                        egui::Key::L => Some('l'),
                        egui::Key::M => Some('m'),
                        egui::Key::N => Some('n'),
                        egui::Key::O => Some('o'),
                        egui::Key::P => Some('p'),
                        egui::Key::Q => Some('q'),
                        egui::Key::R => Some('r'),
                        egui::Key::S => Some('s'),
                        egui::Key::T => Some('t'),
                        egui::Key::U => Some('u'),
                        egui::Key::V => Some('v'),
                        egui::Key::W => Some('w'),
                        egui::Key::X => Some('x'),
                        egui::Key::Y => Some('y'),
                        egui::Key::Z => Some('z'),
                        egui::Key::Num0 => Some('0'),
                        egui::Key::Num1 => Some('1'),
                        egui::Key::Num2 => Some('2'),
                        egui::Key::Num3 => Some('3'),
                        egui::Key::Num4 => Some('4'),
                        egui::Key::Num5 => Some('5'),
                        egui::Key::Num6 => Some('6'),
                        egui::Key::Num7 => Some('7'),
                        egui::Key::Num8 => Some('8'),
                        egui::Key::Num9 => Some('9'),
                        egui::Key::Space => Some(' '),
                        egui::Key::Enter => Some('\n'),
                        _ => None,
                    };

                    if let Some(mut ch) = key_char {
                        if modifiers.shift {
                            ch = ch.to_ascii_uppercase();
                        }
                        if let Some((code, caps)) = char_to_key_code(ch) {
                            let hook = self.engine_plugins.process_key(code, caps, modifiers.ctrl || modifiers.alt);
                            match hook.code {
                                HookCodeState::WillProcess | HookCodeState::Restore => {
                                    for _ in 0..hook.backspace_count {
                                        self.playground_text.pop();
                                    }
                                    for &cd in &hook.char_data {
                                        if let Some(c) = char::from_u32(cd) {
                                            self.playground_text.push(c);
                                        }
                                    }
                                    self.playground_last_event = format!(
                                        "[{}] [{}] Phím '{}' -> Xóa {} ký tự (Delay: {}ms), chèn lại {:?} (Hook: {:?})",
                                        self.engine_plugins.active_name(),
                                        self.engine.target_mode.display_name(),
                                        ch, hook.backspace_count, hook.backspace_delay_ms, hook.char_data, hook.code
                                    );
                                }
                                HookCodeState::ReplaceMacro => {
                                    for _ in 0..hook.backspace_count {
                                        self.playground_text.pop();
                                    }
                                    for &cd in &hook.char_data {
                                        if let Some(c) = char::from_u32(cd) {
                                            self.playground_text.push(c);
                                        }
                                    }
                                    self.playground_text.push(' ');
                                    self.playground_last_event = format!(
                                        "[{}] [{}] Gõ tắt kích hoạt -> Xóa {} ký tự, thay thế bằng macro",
                                        self.engine_plugins.active_name(),
                                        self.engine.target_mode.display_name(),
                                        hook.backspace_count
                                    );
                                }
                                HookCodeState::DoNothing | HookCodeState::BreakWord | HookCodeState::RestoreAndStartNewSession => {
                                    self.playground_text.push(ch);
                                    self.playground_last_event = format!(
                                        "[{}] [{}] Phím '{}' được nhập trực tiếp",
                                        self.engine_plugins.active_name(),
                                        self.engine.target_mode.display_name(),
                                        ch
                                    );
                                }
                            }
                        }
                    } else if key == egui::Key::Backspace {
                        self.playground_text.pop();
                        self.engine_plugins.reset();
                        self.engine.process_key(KEY_DELETE, false, false);
                        self.playground_last_event = format!("[{}] Backspace", self.engine.target_mode.display_name());
                    }
                }
            }
        }

        let mut pg_switch_engine_to: Option<String> = None;
        ui.horizontal(|ui| {
            ui.label(RichText::new("Bộ gõ:").strong());
            egui::ComboBox::from_id_salt("pg_engine_plugin_combo")
                .selected_text(self.engine_plugins.active_name())
                .show_ui(ui, |ui| {
                    for (id, name, _) in self.engine_plugins.available_engines() {
                        let is_active = self.engine_plugins.active_id() == id;
                        if ui.selectable_label(is_active, &name).clicked() {
                            pg_switch_engine_to = Some(id);
                        }
                    }
                });
            if let Some(id) = pg_switch_engine_to {
                self.engine_plugins.set_active(&id);
            }
            ui.separator();

            ui.label(RichText::new(self.ui_lang.mode_label()).strong());
            if ui.selectable_label(self.engine.target_mode == TargetMode::Win, TargetMode::Win.display_name()).clicked() {
                self.engine.target_mode = TargetMode::Win;
                self.engine_plugins.set_target_mode(TargetMode::Win);
            }
            if ui.selectable_label(self.engine.target_mode == TargetMode::Linux, TargetMode::Linux.display_name()).clicked() {
                self.engine.target_mode = TargetMode::Linux;
                self.engine_plugins.set_target_mode(TargetMode::Linux);
            }
            if ui.selectable_label(self.engine.target_mode == TargetMode::Game, TargetMode::Game.display_name()).clicked() {
                self.engine.target_mode = TargetMode::Game;
                self.engine_plugins.set_target_mode(TargetMode::Game);
            }
            ui.separator();
            if ui.button(self.ui_lang.clear_playground_btn()).clicked() {
                self.playground_text.clear();
                self.engine.start_new_session();
                self.engine_plugins.reset();
                self.playground_last_event = self.ui_lang.playground_cleared().to_string();
            }
            ui.label(RichText::new(&self.playground_last_event).weak().size(12.0));
        });

        ui.add_space(4.0);

        egui::Frame::none()
            .fill(ui.visuals().extreme_bg_color)
            .stroke(ui.visuals().window_stroke)
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.add(
                    egui::Label::new(
                        RichText::new(if self.playground_text.is_empty() {
                            self.ui_lang.playground_placeholder()
                        } else {
                            &self.playground_text
                        })
                        .size(18.0)
                        .color(if self.playground_text.is_empty() {
                            Color32::from_gray(128)
                        } else {
                            ui.visuals().text_color()
                        }),
                    )
                    .wrap(),
                );
            });
    }
}
