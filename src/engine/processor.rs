//
// TurtKey - Engine Keystroke Processor
// Handles real-time key processing, word breaks, macros, and typing simulation
//

use super::*;
use crate::engine::tables::*;
use crate::engine::types::*;

impl VietnameseEngine {
    /// Primary entry point: process a single keyboard key event
    pub fn process_key(&mut self, key_code: u16, is_caps: bool, other_ctrl: bool) -> KeyHookState {
        let mut hook = KeyHookState::default();
        hook.target_mode = self.target_mode;
        if self.target_mode == TargetMode::Linux {
            hook.backspace_delay_ms = 8;
        }

        if other_ctrl {
            self.start_new_session();
            hook.code = HookCodeState::DoNothing;
            return hook;
        }

        // GAME MODE:
        // Pure keystroke passthrough (no diacritics / no IME interference / zero latency)
        // Word breaks expand Game mode specific macros.
        if self.target_mode == TargetMode::Game {
            if matches!(key_code, KEY_SPACE | KEY_ENTER | KEY_TAB) {
                if self.macro_mgr.enabled && self.index > 0 {
                    let word: String = (0..self.index)
                        .map(|i| key_code_to_char(self.chr(i), (self.typing_word[i] & CAPS_MASK) != 0).unwrap_or(' '))
                        .collect();
                    if let Some(replacement) = self.macro_mgr.lookup_for_mode(&word, TargetMode::Game) {
                        hook.code = HookCodeState::ReplaceMacro;
                        hook.backspace_count = self.index;
                        hook.new_char_count = replacement.chars().count();
                        hook.char_data = replacement.chars().map(|c| c as u32).collect();
                        self.start_new_session();
                        return hook;
                    }
                }
                self.start_new_session();
                hook.code = HookCodeState::DoNothing;
                return hook;
            }

            if matches!(key_code, KEY_ESC | KEY_LEFT | KEY_RIGHT | KEY_UP | KEY_DOWN) {
                self.start_new_session();
                hook.code = HookCodeState::DoNothing;
                return hook;
            }

            if key_code == KEY_DELETE {
                if self.index > 0 {
                    self.index -= 1;
                }
                hook.code = HookCodeState::DoNothing;
                return hook;
            }

            if let Some(_) = key_code_to_char(key_code, is_caps) {
                if self.index < MAX_BUFF {
                    self.set_key_data(self.index, key_code, is_caps);
                    self.index += 1;
                }
            }

            hook.code = HookCodeState::DoNothing;
            return hook;
        }

        // If Vietnamese mode is disabled
        if !self.vietnamese_enabled {
            if self.macro_mgr.allow_in_english_mode && self.macro_mgr.enabled {
                // Macro check in english mode
                if matches!(key_code, KEY_SPACE | KEY_ENTER | KEY_TAB) && self.index > 0 {
                    let word: String = (0..self.index)
                        .map(|i| key_code_to_char(self.chr(i), (self.typing_word[i] & CAPS_MASK) != 0).unwrap_or(' '))
                        .collect();
                    if let Some(replacement) = self.macro_mgr.lookup_for_mode(&word, self.target_mode) {
                        hook.code = HookCodeState::ReplaceMacro;
                        hook.backspace_count = self.index;
                        hook.new_char_count = replacement.chars().count();
                        hook.char_data = replacement.chars().map(|c| c as u32).collect();
                        self.start_new_session();
                        return hook;
                    }
                }
            }
            if matches!(key_code, KEY_SPACE | KEY_ENTER | KEY_TAB | KEY_ESC) {
                self.start_new_session();
            } else if key_code == KEY_DELETE {
                if self.index > 0 {
                    self.index -= 1;
                }
            } else if let Some(_) = key_code_to_char(key_code, is_caps) {
                if self.index < MAX_BUFF {
                    self.set_key_data(self.index, key_code, is_caps);
                    self.index += 1;
                }
            }
            hook.code = HookCodeState::DoNothing;
            return hook;
        }

        // Backspace handling
        if key_code == KEY_DELETE {
            if self.index > 0 {
                self.index -= 1;
            }
            if self.state_index > 0 {
                self.state_index -= 1;
            }
            hook.code = HookCodeState::DoNothing;
            return hook;
        }

        // Word breaking keys (Space, Enter, Tab, Escape, etc.)
        let is_break = matches!(
            key_code,
            KEY_SPACE | KEY_ENTER | KEY_TAB | KEY_ESC | KEY_LEFT | KEY_RIGHT | KEY_UP | KEY_DOWN
        );

        if is_break {
            // Check macro expansion upon word break
            if self.macro_mgr.enabled && self.index > 0 {
                let word: String = (0..self.index)
                    .map(|i| self.get_character_code(self.typing_word[i]))
                    .collect();

                if let Some(replacement) = self.macro_mgr.lookup_for_mode(&word, self.target_mode) {
                    hook.code = HookCodeState::ReplaceMacro;
                    hook.backspace_count = self.index;
                    hook.new_char_count = replacement.chars().count();
                    hook.char_data = replacement.chars().map(|c| c as u32).collect();
                    self.start_new_session();
                    return hook;
                }
            }

            self.start_new_session();
            hook.code = HookCodeState::BreakWord;
            return hook;
        }

        // Quick start consonant (e.g. f -> ph, j -> gi, w -> qu)
        if self.quick_start_consonant && self.index == 0 {
            let m = get_quick_start_consonant_map();
            if let Some(&replacement) = m.get(&key_code) {
                self.insert_key(replacement[0], is_caps);
                self.insert_key(replacement[1], false);
                hook.code = HookCodeState::WillProcess;
                hook.backspace_count = 0;
                hook.new_char_count = 2;
                hook.char_data = vec![
                    key_code_to_char(replacement[0], is_caps).unwrap() as u32,
                    key_code_to_char(replacement[1], false).unwrap() as u32,
                ];
                return hook;
            }
        }

        // Quick telex (cc -> ch, gg -> gi, kk -> kh, nn -> ng, qq -> qu, pp -> ph, tt -> th)
        if self.quick_telex && self.index > 0 && self.chr(self.index - 1) == key_code {
            let m = get_quick_telex_map();
            if let Some(&replacement) = m.get(&(key_code as u32)) {
                self.typing_word[self.index - 1] = (replacement[0] as u32) | if is_caps { CAPS_MASK } else { 0 };
                self.insert_key(replacement[1], false);
                hook.code = HookCodeState::WillProcess;
                hook.backspace_count = 1;
                hook.new_char_count = 2;
                hook.char_data = vec![
                    key_code_to_char(replacement[0], is_caps).unwrap() as u32,
                    key_code_to_char(replacement[1], false).unwrap() as u32,
                ];
                return hook;
            }
        }

        // Processing based on input type (Telex, VNI, Simple Telex)
        match self.input_type {
            InputType::Telex | InputType::SimpleTelex1 | InputType::SimpleTelex2 => {
                let is_special = match key_code {
                    KEY_S | KEY_F | KEY_R | KEY_X | KEY_J | KEY_Z | KEY_A | KEY_E | KEY_O | KEY_W | KEY_D => true,
                    KEY_LEFT_BRACKET | KEY_RIGHT_BRACKET => self.input_type == InputType::Telex,
                    _ => false,
                };

                if is_special && !self.temp_disable_key {
                    match key_code {
                        KEY_S => self.insert_mark(MARK1_MASK, &mut hook),
                        KEY_F => self.insert_mark(MARK2_MASK, &mut hook),
                        KEY_R => self.insert_mark(MARK3_MASK, &mut hook),
                        KEY_X => self.insert_mark(MARK4_MASK, &mut hook),
                        KEY_J => self.insert_mark(MARK5_MASK, &mut hook),
                        KEY_Z => self.remove_marks(&mut hook),
                        KEY_A => self.insert_aoe(KEY_A, &mut hook),
                        KEY_E => self.insert_aoe(KEY_E, &mut hook),
                        KEY_O => self.insert_aoe(KEY_O, &mut hook),
                        KEY_D => self.insert_d(&mut hook),
                        KEY_W => self.insert_w(&mut hook),
                        _ => {}
                    }
                }

                if hook.code == HookCodeState::DoNothing {
                    self.insert_key(key_code, is_caps);
                    self.insert_state(key_code, is_caps);
                }
            }
            InputType::Vni => {
                let is_special = matches!(
                    key_code,
                    KEY_1 | KEY_2 | KEY_3 | KEY_4 | KEY_5 | KEY_6 | KEY_7 | KEY_8 | KEY_9 | KEY_0
                );

                if is_special && !self.temp_disable_key {
                    match key_code {
                        KEY_1 => self.insert_mark(MARK1_MASK, &mut hook),
                        KEY_2 => self.insert_mark(MARK2_MASK, &mut hook),
                        KEY_3 => self.insert_mark(MARK3_MASK, &mut hook),
                        KEY_4 => self.insert_mark(MARK4_MASK, &mut hook),
                        KEY_5 => self.insert_mark(MARK5_MASK, &mut hook),
                        KEY_0 => self.remove_marks(&mut hook),
                        KEY_6 => {
                            // Circumflex for A, O, E
                            let (vsi, vei, _) = self.find_vowels(false);
                            let mut applied = false;
                            for ii in (vsi..=vei).rev() {
                                let c = self.chr(ii);
                                if c == KEY_A || c == KEY_O || c == KEY_E {
                                    self.insert_aoe(c, &mut hook);
                                    applied = true;
                                    break;
                                }
                            }
                            if !applied {
                                hook.code = HookCodeState::DoNothing;
                            }
                        }
                        KEY_7 => self.insert_w(&mut hook),
                        KEY_8 => self.insert_w(&mut hook), // Breve for A
                        KEY_9 => self.insert_d(&mut hook),
                        _ => {}
                    }
                }

                if hook.code == HookCodeState::DoNothing {
                    self.insert_key(key_code, is_caps);
                    self.insert_state(key_code, is_caps);
                }
            }
        }

        hook
    }

    /// High level simulation helper for strings: types a whole string through the engine
    pub fn type_string(&mut self, text: &str) -> String {
        let mut buffer = String::new();
        for ch in text.chars() {
            if let Some((code, caps)) = char_to_key_code(ch) {
                let hook = self.process_key(code, caps, false);
                match hook.code {
                    HookCodeState::WillProcess | HookCodeState::Restore => {
                        for _ in 0..hook.backspace_count {
                            buffer.pop();
                        }
                        for &cd in &hook.char_data {
                            if let Some(c) = char::from_u32(cd) {
                                buffer.push(c);
                            }
                        }
                    }
                    HookCodeState::ReplaceMacro => {
                        for _ in 0..hook.backspace_count {
                            buffer.pop();
                        }
                        for &cd in &hook.char_data {
                            if let Some(c) = char::from_u32(cd) {
                                buffer.push(c);
                            }
                        }
                        buffer.push(' ');
                    }
                    HookCodeState::DoNothing | HookCodeState::BreakWord | HookCodeState::RestoreAndStartNewSession => {
                        buffer.push(ch);
                    }
                }
            } else {
                buffer.push(ch);
            }
        }
        buffer
    }
}
