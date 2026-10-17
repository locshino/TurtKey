//
// TurtKey - Vietnamese Tone and Diacritic Modifiers
// Handlers for: Marks (Sắc/Huyền/Hỏi/Ngã/Nặng), Circumflex (â/ê/ô), Horn (ă/ơ/ư), Đ/d, and Z (unmark)
//

use super::types::*;
use super::VietnameseEngine;

impl VietnameseEngine {
    pub(crate) fn insert_mark(&mut self, mark_mask: u32, hook: &mut KeyHookState) {
        let (vsi, vei, vowel_count) = self.find_vowels(false);
        if vowel_count == 0 {
            hook.code = HookCodeState::DoNothing;
            return;
        }

        let mut vwsm = if vowel_count == 1 {
            vei
        } else if self.use_modern_orthography {
            self.handle_modern_mark(vsi, vei, vowel_count)
        } else {
            self.handle_old_mark(vsi, vei, vowel_count)
        };

        if (self.typing_word[vei] & TONE_MASK) != 0 || (self.typing_word[vei] & TONEW_MASK) != 0 {
            vwsm = vei;
        }

        // If duplicate same mark -> toggle off (restore)
        if (self.typing_word[vwsm] & mark_mask) != 0 {
            self.typing_word[vwsm] &= !MARK_MASK;
            hook.code = HookCodeState::Restore;
            self.temp_disable_key = true;
        } else {
            // Apply new mark, clear marks from other vowels
            hook.code = HookCodeState::WillProcess;
            for ii in vsi..self.index {
                self.typing_word[ii] &= !MARK_MASK;
            }
            self.typing_word[vwsm] |= mark_mask;
        }

        // Build replacement string
        let count = self.index - vsi;
        hook.backspace_count = count;
        hook.new_char_count = count;
        hook.char_data.clear();
        for ii in vsi..self.index {
            let ch = self.get_character_code(self.typing_word[ii]);
            hook.char_data.push(ch as u32);
        }
    }

    pub(crate) fn insert_aoe(&mut self, target_key: u16, hook: &mut KeyHookState) {
        let (vsi, vei, _) = self.find_vowels(false);
        for ii in vsi..=vei {
            self.typing_word[ii] &= !TONEW_MASK;
        }

        let mut pos = None;
        for ii in (0..self.index).rev() {
            if self.chr(ii) == target_key {
                pos = Some(ii);
                break;
            }
        }

        if let Some(p) = pos {
            hook.code = HookCodeState::WillProcess;
            if (self.typing_word[p] & TONE_MASK) != 0 {
                // Toggle off
                hook.code = HookCodeState::Restore;
                self.typing_word[p] &= !TONE_MASK;
                if target_key != KEY_O {
                    self.temp_disable_key = true;
                }
            } else {
                self.typing_word[p] |= TONE_MASK;
                if target_key != KEY_D {
                    self.typing_word[p] &= !TONEW_MASK;
                }
            }

            let count = self.index - p;
            hook.backspace_count = count;
            hook.new_char_count = count;
            hook.char_data.clear();
            for ii in p..self.index {
                let ch = self.get_character_code(self.typing_word[ii]);
                hook.char_data.push(ch as u32);
            }
        } else {
            hook.code = HookCodeState::DoNothing;
        }
    }

    pub(crate) fn insert_d(&mut self, hook: &mut KeyHookState) {
        let mut pos = None;
        for ii in (0..self.index).rev() {
            if self.chr(ii) == KEY_D {
                pos = Some(ii);
                break;
            }
        }

        if let Some(p) = pos {
            hook.code = HookCodeState::WillProcess;
            if (self.typing_word[p] & TONE_MASK) != 0 {
                hook.code = HookCodeState::Restore;
                self.typing_word[p] &= !TONE_MASK;
                self.temp_disable_key = true;
            } else {
                self.typing_word[p] |= TONE_MASK;
            }

            let count = self.index - p;
            hook.backspace_count = count;
            hook.new_char_count = count;
            hook.char_data.clear();
            for ii in p..self.index {
                let ch = self.get_character_code(self.typing_word[ii]);
                hook.char_data.push(ch as u32);
            }
        } else {
            hook.code = HookCodeState::DoNothing;
        }
    }

    pub(crate) fn insert_w(&mut self, hook: &mut KeyHookState) {
        let (vsi, vei, vowel_count) = self.find_vowels(false);
        for ii in vsi..=vei {
            self.typing_word[ii] &= !TONE_MASK;
        }

        if vowel_count > 1 {
            let both_w = (self.typing_word[vsi] & TONEW_MASK) != 0
                && (self.typing_word[vsi + 1] & TONEW_MASK) != 0;
            let u_and_i = (self.typing_word[vsi] & TONEW_MASK) != 0 && self.chr(vsi + 1) == KEY_I;
            let u_and_a = (self.typing_word[vsi] & TONEW_MASK) != 0 && self.chr(vsi + 1) == KEY_A;

            if both_w || u_and_i || u_and_a {
                // Toggle off
                hook.code = HookCodeState::Restore;
                for ii in vsi..self.index {
                    self.typing_word[ii] &= !TONEW_MASK;
                }
                self.temp_disable_key = true;
            } else {
                hook.code = HookCodeState::WillProcess;
                if self.chr(vsi) == KEY_U && self.chr(vsi + 1) == KEY_O {
                    // uow -> ươ
                    self.typing_word[vsi] |= TONEW_MASK;
                    self.typing_word[vsi + 1] |= TONEW_MASK;
                } else if self.chr(vsi) == KEY_U {
                    self.typing_word[vsi] |= TONEW_MASK;
                } else if self.chr(vsi + 1) == KEY_O || self.chr(vsi + 1) == KEY_A {
                    self.typing_word[vsi + 1] |= TONEW_MASK;
                }
            }

            let count = self.index - vsi;
            hook.backspace_count = count;
            hook.new_char_count = count;
            hook.char_data.clear();
            for ii in vsi..self.index {
                let ch = self.get_character_code(self.typing_word[ii]);
                hook.char_data.push(ch as u32);
            }
        } else if vowel_count == 1 {
            let target = vsi;
            let ch = self.chr(target);
            if ch == KEY_U || ch == KEY_O || ch == KEY_A {
                hook.code = HookCodeState::WillProcess;
                if (self.typing_word[target] & TONEW_MASK) != 0 {
                    hook.code = HookCodeState::Restore;
                    self.typing_word[target] &= !TONEW_MASK;
                    self.temp_disable_key = true;
                } else {
                    self.typing_word[target] |= TONEW_MASK;
                }

                let count = self.index - target;
                hook.backspace_count = count;
                hook.new_char_count = count;
                hook.char_data.clear();
                for ii in target..self.index {
                    let ch = self.get_character_code(self.typing_word[ii]);
                    hook.char_data.push(ch as u32);
                }
            } else {
                hook.code = HookCodeState::DoNothing;
            }
        } else {
            hook.code = HookCodeState::DoNothing;
        }
    }

    pub(crate) fn remove_marks(&mut self, hook: &mut KeyHookState) {
        let (vsi, vei, _) = self.find_vowels(true);
        let mut changed = false;
        for ii in vsi..=vei {
            if (self.typing_word[ii] & MARK_MASK) != 0 {
                self.typing_word[ii] &= !MARK_MASK;
                changed = true;
            }
        }

        if changed {
            hook.code = HookCodeState::WillProcess;
            let count = self.index - vsi;
            hook.backspace_count = count;
            hook.new_char_count = count;
            hook.char_data.clear();
            for ii in vsi..self.index {
                let ch = self.get_character_code(self.typing_word[ii]);
                hook.char_data.push(ch as u32);
            }
        } else {
            hook.code = HookCodeState::DoNothing;
        }
    }
}
