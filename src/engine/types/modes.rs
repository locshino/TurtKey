//
// TurtKey - Engine Input Modes and Character Sets
// Supports InputType (Telex, VNI), CodeTable (Unicode, TCVN3), and TargetMode (Win, Linux)
//

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    Telex = 0,
    Vni = 1,
    SimpleTelex1 = 2,
    SimpleTelex2 = 3,
}

impl InputType {
    pub fn all() -> &'static [InputType] {
        &[
            InputType::Telex,
            InputType::Vni,
            InputType::SimpleTelex1,
            InputType::SimpleTelex2,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            InputType::Telex => "Telex",
            InputType::Vni => "VNI",
            InputType::SimpleTelex1 => "Simple Telex 1",
            InputType::SimpleTelex2 => "Simple Telex 2",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeTable {
    Unicode = 0,
    Tcvn3 = 1,
    VniWindows = 2,
    UnicodeCompound = 3,
    Cp1258 = 4,
}

impl CodeTable {
    pub fn all() -> &'static [CodeTable] {
        &[
            CodeTable::Unicode,
            CodeTable::Tcvn3,
            CodeTable::VniWindows,
            CodeTable::UnicodeCompound,
            CodeTable::Cp1258,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CodeTable::Unicode => "Unicode (Dựng sẵn)",
            CodeTable::Tcvn3 => "TCVN3 (ABC)",
            CodeTable::VniWindows => "VNI-Windows",
            CodeTable::UnicodeCompound => "Unicode Tổ hợp",
            CodeTable::Cp1258 => "Vietnamese Locale CP 1258",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TargetMode {
    #[default]
    Win,
    Linux,
    Game,
}

impl TargetMode {
    pub fn all() -> &'static [TargetMode] {
        &[TargetMode::Win, TargetMode::Linux, TargetMode::Game]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TargetMode::Win => "Win mode",
            TargetMode::Linux => "Linux mode",
            TargetMode::Game => "Game mode",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            TargetMode::Win => "Win",
            TargetMode::Linux => "Linux",
            TargetMode::Game => "Game",
        }
    }

    pub fn description(&self) -> &'static str {
        self.description_with_lang(crate::i18n::UiLanguage::Vietnamese)
    }

    pub fn description_with_lang(&self, lang: crate::i18n::UiLanguage) -> &'static str {
        match (self, lang) {
            (TargetMode::Win, crate::i18n::UiLanguage::Vietnamese) => "Tối ưu cho phần mềm Windows (Tốc độ cao nhất)",
            (TargetMode::Win, crate::i18n::UiLanguage::English) => "Optimized for Windows applications (Maximum speed)",
            (TargetMode::Linux, crate::i18n::UiLanguage::Vietnamese) => "Tối ưu cho WSL, Terminal, WSLg (Chống nhảy xóa chữ)",
            (TargetMode::Linux, crate::i18n::UiLanguage::English) => "Optimized for WSL, Terminal, WSLg (Prevents character jumping and deletion)",
            (TargetMode::Game, crate::i18n::UiLanguage::Vietnamese) => "Tối ưu cho Game (Phím bấm mượt, không can thiệp bộ gõ, gõ tắt chuyên dụng)",
            (TargetMode::Game, crate::i18n::UiLanguage::English) => "Optimized for Gaming (Zero IME interference, raw smooth keystrokes, dedicated game shortcuts)",
        }
    }

    pub fn backspace_delay_ms(&self) -> u32 {
        match self {
            TargetMode::Win => 0,
            TargetMode::Linux => 8,
            TargetMode::Game => 0,
        }
    }
}
