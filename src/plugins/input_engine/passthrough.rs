//
// TurtKey - Passthrough / English Input Engine Plugin
// Standard Latin typing without character transformations
//

use crate::engine::tables::key_code_to_char;
use crate::engine::types::{KeyHookState, TargetMode};
use super::InputEnginePlugin;

pub struct PassthroughEnginePlugin {
    pub target_mode: TargetMode,
}

impl PassthroughEnginePlugin {
    pub fn new() -> Self {
        Self {
            target_mode: TargetMode::Win,
        }
    }
}

impl Default for PassthroughEnginePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEnginePlugin for PassthroughEnginePlugin {
    fn id(&self) -> &str {
        "english"
    }

    fn name(&self) -> &str {
        "English (Passthrough)"
    }

    fn description(&self) -> &str {
        "Chế độ gõ ký tự gốc (Raw Passthrough), không biến đổi ký tự."
    }

    fn process_key(
        &mut self,
        key_code: u16,
        is_caps: bool,
        _other_control: bool,
    ) -> KeyHookState {
        if let Some(c) = key_code_to_char(key_code, is_caps) {
            KeyHookState::send_char(c)
        } else {
            KeyHookState::do_nothing()
        }
    }

    fn reset(&mut self) {}

    fn set_target_mode(&mut self, mode: TargetMode) {
        self.target_mode = mode;
    }

    fn target_mode(&self) -> TargetMode {
        self.target_mode
    }
}
