//
// TurtKey - Engine Key Codes and Masks
// Based on OpenKey engine (DataType.h)
// Copyright (c) 2019 Mai Vu Tuyen (OpenKey)
// Rust rewrite (c) 2026 TurtKey
//

pub const MAX_BUFF: usize = 32;

// Internal engine masks
pub const CAPS_MASK: u32 = 0x10000;
pub const TONE_MASK: u32 = 0x20000; // circumflex ^ (â, ê, ô)
pub const TONEW_MASK: u32 = 0x40000; // horn/breve (ă, ơ, ư)

// Mark masks: Sắc, Huyền, Hỏi, Ngã, Nặng
pub const MARK1_MASK: u32 = 0x80000; // Dấu Sắc - á
pub const MARK2_MASK: u32 = 0x100000; // Dấu Huyền - à
pub const MARK3_MASK: u32 = 0x200000; // Dấu Hỏi - ả
pub const MARK4_MASK: u32 = 0x400000; // Dấu Ngã - ã
pub const MARK5_MASK: u32 = 0x800000; // Dấu Nặng - ạ

pub const MARK_MASK: u32 = 0xF80000; // Check whether has any mark
pub const CHAR_MASK: u32 = 0xFFFF; // First 16 bits character/key code
pub const STANDALONE_MASK: u32 = 0x1000000; // Standalone key (e.g. W, [, ])
pub const CHAR_CODE_MASK: u32 = 0x2000000; // Whether data is char code or key code
pub const PURE_CHARACTER_MASK: u32 = 0x80000000;

pub const END_CONSONANT_MASK: u16 = 0x4000;
pub const CONSONANT_ALLOW_MASK: u16 = 0x8000;

// Standard key codes used internally
pub const KEY_ESC: u16 = 9;
pub const KEY_DELETE: u16 = 22;
pub const KEY_TAB: u16 = 23;
pub const KEY_ENTER: u16 = 36;
pub const KEY_RETURN: u16 = 36;
pub const KEY_SPACE: u16 = 65;
pub const KEY_LEFT: u16 = 113;
pub const KEY_RIGHT: u16 = 114;
pub const KEY_DOWN: u16 = 116;
pub const KEY_UP: u16 = 111;

pub const KEY_EMPTY: u16 = 256;
pub const KEY_A: u16 = 38;
pub const KEY_B: u16 = 56;
pub const KEY_C: u16 = 54;
pub const KEY_D: u16 = 40;
pub const KEY_E: u16 = 26;
pub const KEY_F: u16 = 41;
pub const KEY_G: u16 = 42;
pub const KEY_H: u16 = 43;
pub const KEY_I: u16 = 31;
pub const KEY_J: u16 = 44;
pub const KEY_K: u16 = 45;
pub const KEY_L: u16 = 46;
pub const KEY_M: u16 = 58;
pub const KEY_N: u16 = 57;
pub const KEY_O: u16 = 32;
pub const KEY_P: u16 = 33;
pub const KEY_Q: u16 = 24;
pub const KEY_R: u16 = 27;
pub const KEY_S: u16 = 39;
pub const KEY_T: u16 = 28;
pub const KEY_U: u16 = 30;
pub const KEY_V: u16 = 55;
pub const KEY_W: u16 = 25;
pub const KEY_X: u16 = 53;
pub const KEY_Y: u16 = 29;
pub const KEY_Z: u16 = 52;

pub const KEY_1: u16 = 10;
pub const KEY_2: u16 = 11;
pub const KEY_3: u16 = 12;
pub const KEY_4: u16 = 13;
pub const KEY_5: u16 = 14;
pub const KEY_6: u16 = 15;
pub const KEY_7: u16 = 16;
pub const KEY_8: u16 = 17;
pub const KEY_9: u16 = 18;
pub const KEY_0: u16 = 19;

pub const KEY_LEFT_BRACKET: u16 = 34; // [
pub const KEY_RIGHT_BRACKET: u16 = 35; // ]
pub const KEY_LEFT_SHIFT: u16 = 50;
pub const KEY_RIGHT_SHIFT: u16 = 62;
pub const KEY_DOT: u16 = 60; // .
pub const KEY_BACKQUOTE: u16 = 49; // `
pub const KEY_MINUS: u16 = 20; // -
pub const KEY_EQUALS: u16 = 21; // =
pub const KEY_BACK_SLASH: u16 = 51; // \
pub const KEY_SEMICOLON: u16 = 47; // ;
pub const KEY_QUOTE: u16 = 48; // '
pub const KEY_COMMA: u16 = 59; // ,
pub const KEY_SLASH: u16 = 61; // /

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Keyboard,
    Mouse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventState {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
}

#[inline]
pub fn is_consonant(key_code: u16) -> bool {
    !matches!(
        key_code,
        KEY_A | KEY_E | KEY_I | KEY_O | KEY_U | KEY_Y
    )
}
