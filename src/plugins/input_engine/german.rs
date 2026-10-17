//
// TurtKey - German Input Engine Plugin
// Handles German umlauts and eszett: ae->ä, oe->ö, ue->ü, ss->ß, Ae->Ä
//

use crate::engine::tables::key_code_to_char;
use crate::engine::types::{KeyHookState, TargetMode};
use super::InputEnginePlugin;

pub struct GermanEnginePlugin {
    pub target_mode: TargetMode,
    prev_char: Option<char>,
}

impl GermanEnginePlugin {
    pub fn new() -> Self {
        Self {
            target_mode: TargetMode::Win,
            prev_char: None,
        }
    }

    fn compose_german(&self, prev: char, cur: char) -> Option<char> {
        match (prev, cur) {
            ('a', 'e') => Some('ä'),
            ('o', 'e') => Some('ö'),
            ('u', 'e') => Some('ü'),
            ('s', 's') => Some('ß'),
            ('A', 'e') | ('A', 'E') => Some('Ä'),
            ('O', 'e') | ('O', 'E') => Some('Ö'),
            ('U', 'e') | ('U', 'E') => Some('Ü'),
            _ => None,
        }
    }
}

impl Default for GermanEnginePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEnginePlugin for GermanEnginePlugin {
    fn id(&self) -> &str {
        "german"
    }

    fn name(&self) -> &str {
        "Deutsch (Umlaute: ae->ä, oe->ö, ue->ü, ss->ß)"
    }

    fn description(&self) -> &str {
        "Quy tắc gõ tiếng Đức: ae->ä, oe->ö, ue->ü, ss->ß, Ae->Ä, Oe->Ö, Ue->Ü."
    }

    fn process_key(
        &mut self,
        key_code: u16,
        is_caps: bool,
        other_control: bool,
    ) -> KeyHookState {
        if other_control {
            self.reset();
            return KeyHookState::do_nothing();
        }

        let ch = match key_code_to_char(key_code, is_caps) {
            Some(c) => c,
            None => return KeyHookState::do_nothing(),
        };

        if let Some(prev) = self.prev_char {
            if let Some(composed) = self.compose_german(prev, ch) {
                self.prev_char = Some(composed);
                return KeyHookState::backspace_and_send(
                    1,
                    composed,
                    self.target_mode.backspace_delay_ms(),
                );
            }
        }

        if ch.is_whitespace() || ch.is_ascii_punctuation() {
            self.prev_char = None;
        } else {
            self.prev_char = Some(ch);
        }

        KeyHookState::send_char(ch)
    }

    fn reset(&mut self) {
        self.prev_char = None;
    }

    fn set_target_mode(&mut self, mode: TargetMode) {
        self.target_mode = mode;
    }

    fn target_mode(&self) -> TargetMode {
        self.target_mode
    }
}
