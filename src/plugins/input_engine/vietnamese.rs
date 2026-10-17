//
// TurtKey - Vietnamese Input Engine Plugin
// Wraps core VietnameseEngine (Telex, VNI, Simple Telex, Macros, Spelling)
//

use crate::engine::types::{KeyHookState, TargetMode};
use crate::engine::VietnameseEngine;
use super::InputEnginePlugin;

pub struct VietnameseEnginePlugin {
    pub engine: VietnameseEngine,
}

impl VietnameseEnginePlugin {
    pub fn new() -> Self {
        Self {
            engine: VietnameseEngine::new(),
        }
    }
}

impl Default for VietnameseEnginePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEnginePlugin for VietnameseEnginePlugin {
    fn id(&self) -> &str {
        "vietnamese"
    }

    fn name(&self) -> &str {
        "Tiếng Việt (Telex/VNI/Simple)"
    }

    fn description(&self) -> &str {
        "Bộ gõ tiếng Việt hoàn chỉnh dựa trên OpenKey với Telex, VNI, gõ tắt và kiểm tra chính tả."
    }

    fn process_key(
        &mut self,
        key_code: u16,
        is_caps: bool,
        other_control: bool,
    ) -> KeyHookState {
        self.engine.process_key(key_code, is_caps, other_control)
    }

    fn reset(&mut self) {
        self.engine.start_new_session();
    }

    fn set_target_mode(&mut self, mode: TargetMode) {
        self.engine.target_mode = mode;
    }

    fn target_mode(&self) -> TargetMode {
        self.engine.target_mode
    }
}
