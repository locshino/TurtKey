//
// TurtKey - Dedicated Input Method Engine Plugin System
// Modular architecture for multi-language typing engines (Zero emojis)
//

pub mod french;
pub mod german;
pub mod passthrough;
pub mod spanish;
pub mod vietnamese;

pub use french::FrenchEnginePlugin;
pub use german::GermanEnginePlugin;
pub use passthrough::PassthroughEnginePlugin;
pub use spanish::SpanishEnginePlugin;
pub use vietnamese::VietnameseEnginePlugin;

use crate::engine::tables::char_to_key_code;
use crate::engine::types::{HookCodeState, KeyHookState, TargetMode};

/// Trait defining an Input Method Engine Plugin
pub trait InputEnginePlugin: Send + Sync {
    /// Unique identifier (e.g. "vietnamese", "french", "german", "spanish", "english")
    fn id(&self) -> &str;

    /// User-friendly display name (e.g. "Tiếng Việt (Telex/VNI)", "Français (Accents)")
    fn name(&self) -> &str;

    /// Brief description of the input method rules
    fn description(&self) -> &str;

    /// Process a single keystroke and return an action state
    fn process_key(
        &mut self,
        key_code: u16,
        is_caps: bool,
        other_control: bool,
    ) -> KeyHookState;

    /// Reset internal state buffer (on space, punctuation, or escape)
    fn reset(&mut self);

    /// Set target mode (Win mode or Linux mode for WSL)
    fn set_target_mode(&mut self, mode: TargetMode);

    /// Get current target mode
    fn target_mode(&self) -> TargetMode;

    /// Helper to type a full string into the engine and return the emitted result
    fn type_string(&mut self, text: &str) -> String {
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
                    HookCodeState::BreakWord | HookCodeState::RestoreAndStartNewSession => {
                        buffer.push(ch);
                    }
                    HookCodeState::DoNothing => {}
                }
            } else {
                buffer.push(ch);
            }
        }
        buffer
    }
}

/// Runtime Manager for Input Engine Plugins
pub struct EnginePluginManager {
    plugins: Vec<Box<dyn InputEnginePlugin>>,
    active_index: usize,
}

impl Default for EnginePluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EnginePluginManager {
    /// Initialize with default built-in engine plugins: Vietnamese and English (Passthrough)
    pub fn new() -> Self {
        let mut mgr = Self {
            plugins: Vec::new(),
            active_index: 0,
        };

        // Default engine plugins: Vietnamese and English only (for simplicity)
        mgr.register(Box::new(VietnameseEnginePlugin::new()));
        mgr.register(Box::new(PassthroughEnginePlugin::new()));

        mgr
    }

    /// Initialize manager with all extended built-in engine plugins
    pub fn with_all_builtins() -> Self {
        let mut mgr = Self::new();
        mgr.register(Box::new(FrenchEnginePlugin::new()));
        mgr.register(Box::new(GermanEnginePlugin::new()));
        mgr.register(Box::new(SpanishEnginePlugin::new()));
        mgr
    }

    /// Register a new engine plugin
    pub fn register(&mut self, plugin: Box<dyn InputEnginePlugin>) {
        self.plugins.push(plugin);
    }

    /// Select active engine by ID
    pub fn set_active(&mut self, id: &str) -> bool {
        if let Some(idx) = self.plugins.iter().position(|p| p.id() == id) {
            self.active_index = idx;
            self.plugins[idx].reset();
            true
        } else {
            false
        }
    }

    pub fn active_id(&self) -> &str {
        self.plugins[self.active_index].id()
    }

    pub fn active_name(&self) -> &str {
        self.plugins[self.active_index].name()
    }

    pub fn active_plugin(&self) -> &dyn InputEnginePlugin {
        self.plugins[self.active_index].as_ref()
    }

    pub fn active_plugin_mut(&mut self) -> &mut dyn InputEnginePlugin {
        self.plugins[self.active_index].as_mut()
    }

    /// List all available input engines: (id, name, description)
    pub fn available_engines(&self) -> Vec<(String, String, String)> {
        self.plugins
            .iter()
            .map(|p| (p.id().to_string(), p.name().to_string(), p.description().to_string()))
            .collect()
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Process key through active engine
    pub fn process_key(
        &mut self,
        key_code: u16,
        is_caps: bool,
        other_control: bool,
    ) -> KeyHookState {
        self.plugins[self.active_index].process_key(key_code, is_caps, other_control)
    }

    /// Reset active engine session
    pub fn reset(&mut self) {
        self.plugins[self.active_index].reset();
    }

    /// Set target mode across all plugins
    pub fn set_target_mode(&mut self, mode: TargetMode) {
        for plugin in &mut self.plugins {
            plugin.set_target_mode(mode);
        }
    }

    /// Helper to type a full string with the active engine
    pub fn type_string(&mut self, text: &str) -> String {
        self.plugins[self.active_index].type_string(text)
    }
}
