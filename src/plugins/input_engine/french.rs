//
// TurtKey - French Input Engine Plugin
// Handles French accent composition: e'->é, e`->è, e^->ê, c,->ç, oe->œ, ae->æ
//

use crate::engine::tables::key_code_to_char;
use crate::engine::types::{KeyHookState, TargetMode};
use super::InputEnginePlugin;

pub struct FrenchEnginePlugin {
    pub target_mode: TargetMode,
    prev_char: Option<char>,
}

impl FrenchEnginePlugin {
    pub fn new() -> Self {
        Self {
            target_mode: TargetMode::Win,
            prev_char: None,
        }
    }

    fn compose_french(&self, prev: char, cur: char) -> Option<char> {
        match (prev, cur) {
            // Acute accent
            ('e', '\'') => Some('é'),
            ('E', '\'') => Some('É'),
            // Grave accent
            ('e', '`') => Some('è'),
            ('E', '`') => Some('È'),
            ('a', '`') => Some('à'),
            ('A', '`') => Some('À'),
            ('u', '`') => Some('ù'),
            ('U', '`') => Some('Ù'),
            // Circumflex
            ('e', '^') => Some('ê'),
            ('E', '^') => Some('Ê'),
            ('a', '^') => Some('â'),
            ('A', '^') => Some('Â'),
            ('i', '^') => Some('î'),
            ('I', '^') => Some('Î'),
            ('o', '^') => Some('ô'),
            ('O', '^') => Some('Ô'),
            ('u', '^') => Some('û'),
            ('U', '^') => Some('Û'),
            // Trema / Diaeresis
            ('e', '"') => Some('ë'),
            ('E', '"') => Some('Ë'),
            ('i', '"') => Some('ï'),
            ('I', '"') => Some('Ï'),
            ('u', '"') => Some('ü'),
            ('U', '"') => Some('Ü'),
            // Cedilla
            ('c', ',') => Some('ç'),
            ('C', ',') => Some('Ç'),
            // Ligatures
            ('o', 'e') => Some('œ'),
            ('O', 'E') => Some('Œ'),
            ('a', 'e') => Some('æ'),
            ('A', 'E') => Some('Æ'),
            _ => None,
        }
    }
}

impl Default for FrenchEnginePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEnginePlugin for FrenchEnginePlugin {
    fn id(&self) -> &str {
        "french"
    }

    fn name(&self) -> &str {
        "Français (Accents: e'->é, c,->ç, e^->ê)"
    }

    fn description(&self) -> &str {
        "Quy tắc gõ dấu tiếng Pháp: e'->é, e`->è, e^->ê, c,->ç, oe->œ, ae->æ."
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
            if let Some(composed) = self.compose_french(prev, ch) {
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
