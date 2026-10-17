//
// TurtKey - Spanish Input Engine Plugin
// Handles Spanish accents and punctuation: n~->ñ, a'->á, ??->¿, !!-->¡
//

use crate::engine::tables::key_code_to_char;
use crate::engine::types::{KeyHookState, TargetMode};
use super::InputEnginePlugin;

pub struct SpanishEnginePlugin {
    pub target_mode: TargetMode,
    prev_char: Option<char>,
}

impl SpanishEnginePlugin {
    pub fn new() -> Self {
        Self {
            target_mode: TargetMode::Win,
            prev_char: None,
        }
    }

    fn compose_spanish(&self, prev: char, cur: char) -> Option<char> {
        match (prev, cur) {
            // Tilde on n
            ('n', '~') => Some('ñ'),
            ('N', '~') => Some('Ñ'),
            // Acute accents
            ('a', '\'') => Some('á'),
            ('e', '\'') => Some('é'),
            ('i', '\'') => Some('í'),
            ('o', '\'') => Some('ó'),
            ('u', '\'') => Some('ú'),
            ('A', '\'') => Some('Á'),
            ('E', '\'') => Some('É'),
            ('I', '\'') => Some('Í'),
            ('O', '\'') => Some('Ó'),
            ('U', '\'') => Some('Ú'),
            // Diaeresis
            ('u', '"') => Some('ü'),
            ('U', '"') => Some('Ü'),
            // Inverted punctuation
            ('?', '?') => Some('¿'),
            ('!', '!') => Some('¡'),
            _ => None,
        }
    }
}

impl Default for SpanishEnginePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEnginePlugin for SpanishEnginePlugin {
    fn id(&self) -> &str {
        "spanish"
    }

    fn name(&self) -> &str {
        "Español (Acentos: n~->ñ, a'->á, ??->¿, !!->¡)"
    }

    fn description(&self) -> &str {
        "Quy tắc gõ tiếng Tây Ban Nha: n~->ñ, a'->á, e'->é, i'->í, ??->¿, !!-->¡."
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
            if let Some(composed) = self.compose_spanish(prev, ch) {
                self.prev_char = Some(composed);
                return KeyHookState::backspace_and_send(
                    1,
                    composed,
                    self.target_mode.backspace_delay_ms(),
                );
            }
        }

        if ch.is_whitespace() {
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
