//
// TurtKey - Engine Hook States and Hotkey Configuration
// Clean, extensible key-hook state machine
//

use serde::{Deserialize, Serialize};
use super::keys::MAX_BUFF;
use super::modes::TargetMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub win: bool,
    pub key_char: Option<char>,
}

impl HotkeyConfig {
    pub const fn new(ctrl: bool, shift: bool, alt: bool, win: bool, key_char: Option<char>) -> Self {
        Self {
            ctrl,
            shift,
            alt,
            win,
            key_char,
        }
    }

    pub const fn ctrl_shift() -> Self {
        Self::new(true, true, false, false, None)
    }

    pub const fn alt_z() -> Self {
        Self::new(false, false, true, false, Some('Z'))
    }

    pub const fn win_space() -> Self {
        Self::new(false, false, false, true, Some(' '))
    }

    pub const fn alt_m() -> Self {
        Self::new(false, false, true, false, Some('M'))
    }

    pub const fn alt_c() -> Self {
        Self::new(false, false, true, false, Some('C'))
    }

    pub const fn ctrl_alt_l() -> Self {
        Self::new(true, false, true, false, Some('L'))
    }

    pub const fn ctrl_shift_c() -> Self {
        Self::new(true, true, false, false, Some('C'))
    }

    pub fn matches(&self, ctrl: bool, shift: bool, alt: bool, win: bool, key_char: Option<char>) -> bool {
        self.ctrl == ctrl
            && self.shift == shift
            && self.alt == alt
            && self.win == win
            && self.key_char.map(|c| c.to_ascii_uppercase()) == key_char.map(|c| c.to_ascii_uppercase())
    }

    pub fn display_name(&self) -> String {
        self.display_name_with_lang(crate::i18n::UiLanguage::Vietnamese)
    }

    pub fn display_name_with_lang(&self, lang: crate::i18n::UiLanguage) -> String {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl".to_string());
        }
        if self.shift {
            parts.push("Shift".to_string());
        }
        if self.alt {
            parts.push("Alt".to_string());
        }
        if self.win {
            parts.push("Win".to_string());
        }
        if let Some(ch) = self.key_char {
            if ch == ' ' {
                parts.push("Space".to_string());
            } else {
                parts.push(ch.to_ascii_uppercase().to_string());
            }
        }
        if parts.is_empty() {
            match lang {
                crate::i18n::UiLanguage::Vietnamese => "Chưa đặt".to_string(),
                crate::i18n::UiLanguage::English => "Not set".to_string(),
            }
        } else {
            parts.join(" + ")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookCodeState {
    DoNothing = 0,
    WillProcess = 1,
    BreakWord = 2,
    Restore = 3,
    ReplaceMacro = 4,
    RestoreAndStartNewSession = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyHookState {
    pub code: HookCodeState,
    pub backspace_count: usize,
    pub new_char_count: usize,
    pub ext_code: u8,
    pub char_data: Vec<u32>,
    pub macro_key: Vec<u32>,
    pub macro_data: Vec<u32>,
    pub target_mode: TargetMode,
    pub backspace_delay_ms: u32,
}

impl Default for KeyHookState {
    fn default() -> Self {
        Self {
            code: HookCodeState::DoNothing,
            backspace_count: 0,
            new_char_count: 0,
            ext_code: 0,
            char_data: vec![0; MAX_BUFF],
            macro_key: Vec::new(),
            macro_data: Vec::new(),
            target_mode: TargetMode::Win,
            backspace_delay_ms: 0,
        }
    }
}

impl KeyHookState {
    pub fn do_nothing() -> Self {
        Self::default()
    }

    pub fn send_char(c: char) -> Self {
        Self {
            code: HookCodeState::WillProcess,
            backspace_count: 0,
            new_char_count: 1,
            ext_code: 0,
            char_data: vec![c as u32],
            macro_key: Vec::new(),
            macro_data: Vec::new(),
            target_mode: TargetMode::Win,
            backspace_delay_ms: 0,
        }
    }

    pub fn backspace_and_send(bs: usize, c: char, delay_ms: u32) -> Self {
        Self {
            code: HookCodeState::WillProcess,
            backspace_count: bs,
            new_char_count: 1,
            ext_code: 0,
            char_data: vec![c as u32],
            macro_key: Vec::new(),
            macro_data: Vec::new(),
            target_mode: if delay_ms > 0 { TargetMode::Linux } else { TargetMode::Win },
            backspace_delay_ms: delay_ms,
        }
    }

    pub fn replace_word(bs: usize, word: &str, delay_ms: u32) -> Self {
        Self {
            code: HookCodeState::ReplaceMacro,
            backspace_count: bs,
            new_char_count: word.chars().count(),
            ext_code: 0,
            char_data: word.chars().map(|c| c as u32).collect(),
            macro_key: Vec::new(),
            macro_data: Vec::new(),
            target_mode: if delay_ms > 0 { TargetMode::Linux } else { TargetMode::Win },
            backspace_delay_ms: delay_ms,
        }
    }
}
