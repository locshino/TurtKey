//
// TurtKey - Core Vietnamese Input Engine
// Ported and enhanced from OpenKey Engine.cpp
// Copyright (c) 2019 Mai Vu Tuyen (OpenKey)
// Rust rewrite (c) 2026 TurtKey
//

pub mod blacklist;
pub mod character_code;
pub mod convert;
pub mod macro_table;
pub mod modifiers;
pub mod process_monitor;
pub mod processor;
pub mod spelling;
pub mod tables;
pub mod types;
pub mod vowel_placement;

use macro_table::*;
use spelling::*;
use types::*;

#[derive(Debug, Clone)]
pub struct VietnameseEngine {
    pub input_type: InputType,
    pub code_table: CodeTable,
    pub vietnamese_enabled: bool,
    pub use_modern_orthography: bool,
    pub check_spelling: bool,
    pub restore_if_wrong_spelling: bool,
    pub quick_telex: bool,
    pub quick_start_consonant: bool,
    pub quick_end_consonant: bool,
    pub allow_consonant_zfwj: bool,
    pub upper_case_first_char: bool,
    pub target_mode: TargetMode,
    pub macro_mgr: MacroManager,

    // Internal state buffers
    typing_word: [u32; MAX_BUFF],
    index: usize,
    key_states: [u32; MAX_BUFF],
    state_index: usize,
    temp_disable_key: bool,
    long_word_helper: Vec<u32>,
    #[allow(dead_code)]
    typing_states: Vec<Vec<u32>>,
}

impl Default for VietnameseEngine {
    fn default() -> Self {
        Self {
            input_type: InputType::Telex,
            code_table: CodeTable::Unicode,
            vietnamese_enabled: true,
            use_modern_orthography: true,
            check_spelling: true,
            restore_if_wrong_spelling: true,
            quick_telex: false,
            quick_start_consonant: false,
            quick_end_consonant: false,
            allow_consonant_zfwj: false,
            upper_case_first_char: false,
            target_mode: TargetMode::Win,
            macro_mgr: MacroManager::default(),

            typing_word: [0; MAX_BUFF],
            index: 0,
            key_states: [0; MAX_BUFF],
            state_index: 0,
            temp_disable_key: false,
            long_word_helper: Vec::new(),
            typing_states: Vec::new(),
        }
    }
}

impl VietnameseEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_new_session(&mut self) {
        self.typing_word.fill(0);
        self.key_states.fill(0);
        self.index = 0;
        self.state_index = 0;
        self.temp_disable_key = false;
        self.long_word_helper.clear();
    }

    pub fn is_buffer_empty(&self) -> bool {
        self.index == 0
    }

    pub fn raw_typing_buffer(&self) -> &[u32] {
        &self.typing_word
    }

    pub fn toggle_target_mode(&mut self) -> TargetMode {
        self.target_mode = match self.target_mode {
            TargetMode::Win => TargetMode::Linux,
            TargetMode::Linux => TargetMode::Game,
            TargetMode::Game => TargetMode::Win,
        };
        self.target_mode
    }

    #[inline]
    pub(crate) fn chr(&self, idx: usize) -> u16 {
        (self.typing_word[idx] & CHAR_MASK) as u16
    }

    pub(crate) fn set_key_data(&mut self, idx: usize, key_code: u16, is_caps: bool) {
        if idx < MAX_BUFF {
            self.typing_word[idx] = (key_code as u32) | if is_caps { CAPS_MASK } else { 0 };
        }
    }

    pub(crate) fn insert_key(&mut self, key_code: u16, is_caps: bool) {
        if self.index >= MAX_BUFF {
            self.long_word_helper.push(self.typing_word[0]);
            for i in 0..MAX_BUFF - 1 {
                self.typing_word[i] = self.typing_word[i + 1];
            }
            self.set_key_data(self.index - 1, key_code, is_caps);
        } else {
            self.set_key_data(self.index, key_code, is_caps);
            self.index += 1;
        }

        if self.check_spelling {
            let (sp_ok, sp_v_ok) = SpellingChecker::is_spelling_ok(
                &self.typing_word,
                self.index,
                self.quick_start_consonant,
                self.quick_end_consonant,
                self.allow_consonant_zfwj,
                false,
            );
            self.temp_disable_key = !(sp_ok && sp_v_ok);
        }

        if key_code == KEY_D && self.index >= 2 && is_consonant(self.chr(self.index - 2)) {
            self.temp_disable_key = false;
        }
    }

    pub(crate) fn insert_state(&mut self, key_code: u16, is_caps: bool) {
        if self.state_index >= MAX_BUFF {
            for i in 0..MAX_BUFF - 1 {
                self.key_states[i] = self.key_states[i + 1];
            }
            self.key_states[self.state_index - 1] = (key_code as u32) | if is_caps { CAPS_MASK } else { 0 };
        } else {
            self.key_states[self.state_index] = (key_code as u32) | if is_caps { CAPS_MASK } else { 0 };
            self.state_index += 1;
        }
    }
}

