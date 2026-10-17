//
// TurtKey - Internationalization (i18n) Module
// Clean, professional multi-language support (Zero emojis)
// Supports Vietnamese (Tiếng Việt) and English
//

pub mod blacklist;
pub mod general;
pub mod macro_tab;
pub mod tools_about;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UiLanguage {
    #[default]
    Vietnamese,
    English,
}

impl UiLanguage {
    pub fn all() -> &'static [UiLanguage] {
        &[UiLanguage::Vietnamese, UiLanguage::English]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Tiếng Việt",
            UiLanguage::English => "English",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "VI",
            UiLanguage::English => "EN",
        }
    }

    // App header
    pub fn title(&self) -> &'static str {
        "TurtKey"
    }

    pub fn lang_mode_btn(&self, enabled: bool) -> &'static str {
        match (self, enabled) {
            (UiLanguage::Vietnamese, true) => "[V] Tiếng Việt (Bật)",
            (UiLanguage::Vietnamese, false) => "[E] English (Tắt)",
            (UiLanguage::English, true) => "[V] Vietnamese (On)",
            (UiLanguage::English, false) => "[E] English (Off)",
        }
    }

    pub fn lang_mode_tooltip(&self, hotkey: &str) -> String {
        match self {
            UiLanguage::Vietnamese => format!("Bấm (hoặc nhấn {}) để bật/tắt gõ Tiếng Việt", hotkey),
            UiLanguage::English => format!("Click (or press {}) to toggle Vietnamese typing", hotkey),
        }
    }

    pub fn target_mode_tooltip(&self, hotkey: &str) -> String {
        match self {
            UiLanguage::Vietnamese => format!("Bấm (hoặc nhấn {}) để chuyển giữa Win mode và Linux mode (cho WSL/Terminal)", hotkey),
            UiLanguage::English => format!("Click (or press {}) to switch between Win mode and Linux mode (for WSL/Terminal)", hotkey),
        }
    }

    // Tabs
    pub fn tab_general(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bộ gõ",
            UiLanguage::English => "Engine",
        }
    }

    pub fn tab_macro(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gõ tắt",
            UiLanguage::English => "Macros",
        }
    }

    pub fn tab_convert(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Chuyển mã",
            UiLanguage::English => "Convert",
        }
    }

    pub fn tab_playground(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Thử nghiệm",
            UiLanguage::English => "Playground",
        }
    }

    pub fn tab_about(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Thông tin",
            UiLanguage::English => "About",
        }
    }

    pub fn tab_blacklist(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Loại trừ",
            UiLanguage::English => "Exclude Apps",
        }
    }
}
