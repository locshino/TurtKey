//
// TurtKey - Character Code Generator
// Encodes internal typing buffer entries into characters according to the active CodeTable
//

use super::tables::*;
use super::types::*;
use super::VietnameseEngine;

impl VietnameseEngine {
    /// Converts internal encoded word element to a Unicode char
    pub fn get_character_code(&self, data: u32) -> char {
        let caps_elem: usize = if (data & CAPS_MASK) != 0 { 0 } else { 1 };
        let mut key = data & CHAR_MASK;

        if (data & MARK_MASK) != 0 {
            let mut mark_elem: i32 = match data & MARK_MASK {
                MARK1_MASK => 0,
                MARK2_MASK => 2,
                MARK3_MASK => 4,
                MARK4_MASK => 6,
                MARK5_MASK => 8,
                _ => -2,
            };
            mark_elem += caps_elem as i32;

            match key as u16 {
                KEY_A | KEY_O | KEY_U | KEY_E => {
                    if (data & TONE_MASK) == 0 && (data & TONEW_MASK) == 0 {
                        mark_elem += 4;
                    }
                }
                _ => {}
            }

            if (data & TONE_MASK) != 0 {
                key |= TONE_MASK;
            } else if (data & TONEW_MASK) != 0 {
                key |= TONEW_MASK;
            }

            if let Some(table) = get_code_table_entry(self.code_table, key) {
                if mark_elem >= 0 && (mark_elem as usize) < table.len() {
                    let code = table[mark_elem as usize];
                    if let Some(c) = char::from_u32(code as u32) {
                        return c;
                    }
                }
            }
        } else {
            if let Some(table) = get_code_table_entry(self.code_table, key) {
                if (data & TONE_MASK) != 0 && caps_elem < table.len() {
                    let code = table[caps_elem];
                    if let Some(c) = char::from_u32(code as u32) {
                        return c;
                    }
                } else if (data & TONEW_MASK) != 0 && (caps_elem + 2) < table.len() {
                    let code = table[caps_elem + 2];
                    if let Some(c) = char::from_u32(code as u32) {
                        return c;
                    }
                }
            }
        }

        // Fallback to basic character
        key_code_to_char((data & CHAR_MASK) as u16, (data & CAPS_MASK) != 0).unwrap_or('?')
    }
}
