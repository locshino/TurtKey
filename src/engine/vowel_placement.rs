//
// TurtKey - Vowel Placement & Orthography Analysis
// Determines the precise target vowel index for tone mark placement (Modern vs Classic orthography)
//

use super::tables::get_vowel_combine_map;
use super::types::*;
use super::VietnameseEngine;

impl VietnameseEngine {
    /// Finds start and end indices of the vowel cluster in typing_word
    pub(crate) fn find_vowels(&self, for_grammar: bool) -> (usize, usize, usize) {
        let mut vowel_count = 0;
        let mut vsi = 0;
        let mut vei = 0;

        if self.index == 0 {
            return (0, 0, 0);
        }

        for iii in (0..self.index).rev() {
            let ch = self.chr(iii);
            if is_consonant(ch) {
                if vowel_count > 0 {
                    break;
                }
            } else {
                if vowel_count == 0 {
                    vei = iii;
                }
                if !for_grammar && iii > 0 {
                    let prev = self.chr(iii - 1);
                    if (ch == KEY_I && prev == KEY_G) || (ch == KEY_U && prev == KEY_Q) {
                        break;
                    }
                }
                vsi = iii;
                vowel_count += 1;
            }
        }

        if vsi > 0 && self.chr(vsi) == KEY_U && self.chr(vsi - 1) == KEY_Q {
            vsi += 1;
            if vowel_count > 0 {
                vowel_count -= 1;
            }
        }

        (vsi, vei, vowel_count)
    }

    pub(crate) fn can_has_end_consonant(&self, vsi: usize, vei: usize) -> bool {
        let v_comb = get_vowel_combine_map();
        let first_k = self.chr(vsi);
        if let Some(vo) = v_comb.get(&first_k) {
            for row in *vo {
                let mut kk = vsi;
                let mut match_row = true;
                for iii in 1..row.len() {
                    if kk > vei {
                        match_row = false;
                        break;
                    }
                    let current = (self.chr(kk) as u32)
                        | (self.typing_word[kk] & TONE_MASK)
                        | (self.typing_word[kk] & TONEW_MASK);
                    if current != row[iii] {
                        match_row = false;
                        break;
                    }
                    kk += 1;
                }
                if match_row {
                    return row[0] == 1;
                }
            }
        }
        false
    }

    pub(crate) fn handle_modern_mark(&self, vsi: usize, vei: usize, vowel_count: usize) -> usize {
        let mut vwsm = vei;

        // Rule 2
        if vowel_count == 3 {
            let (c0, c1, c2) = (self.chr(vsi), self.chr(vsi + 1), self.chr(vsi + 2));
            if (c0 == KEY_O && c1 == KEY_A && c2 == KEY_I)
                || (c0 == KEY_U && c1 == KEY_Y && c2 == KEY_U)
                || (c0 == KEY_O && c1 == KEY_E && c2 == KEY_O)
                || (c0 == KEY_U && c1 == KEY_Y && c2 == KEY_A)
            {
                vwsm = vsi + 1;
            }
        } else if vowel_count == 2 {
            let (c0, c1) = (self.chr(vsi), self.chr(vsi + 1));
            if (c0 == KEY_O && c1 == KEY_I) || (c0 == KEY_A && c1 == KEY_I) || (c0 == KEY_U && c1 == KEY_I) {
                vwsm = vsi;
            } else if vei > 0 && self.chr(vei - 1) == KEY_A && self.chr(vei) == KEY_Y {
                vwsm = vei - 1;
            } else if c0 == KEY_U && c1 == KEY_O {
                vwsm = vsi + 1;
            } else if c1 == KEY_O || c1 == KEY_U {
                vwsm = vei - 1;
            } else if c0 == KEY_O || c0 == KEY_U {
                vwsm = vei;
            }
        }

        // Rule 3.1: iê, yê, uô, ươ
        if vsi + 1 < self.index {
            let is_ie_complex = (self.chr(vsi) == KEY_I && (self.typing_word[vsi + 1] & (KEY_E as u32 | TONE_MASK)) != 0)
                || (self.chr(vsi) == KEY_Y && (self.typing_word[vsi + 1] & (KEY_E as u32 | TONE_MASK)) != 0)
                || (self.chr(vsi) == KEY_U && (self.typing_word[vsi + 1] == (KEY_O as u32 | TONE_MASK)))
                || ((self.typing_word[vsi] == (KEY_U as u32 | TONEW_MASK))
                    && (self.typing_word[vsi + 1] == (KEY_O as u32 | TONEW_MASK)));

            if is_ie_complex {
                if vsi + 2 < self.index {
                    let next = self.chr(vsi + 2);
                    if matches!(next, KEY_P | KEY_T | KEY_M | KEY_N | KEY_O | KEY_U | KEY_I | KEY_C)
                        || (vsi + 3 < self.index && next == KEY_C && self.chr(vsi + 3) == KEY_H)
                        || (vsi + 3 < self.index && next == KEY_N && self.chr(vsi + 3) == KEY_H)
                        || (vsi + 3 < self.index && next == KEY_N && self.chr(vsi + 3) == KEY_G)
                    {
                        vwsm = vsi + 1;
                    } else {
                        vwsm = vsi;
                    }
                } else {
                    vwsm = vsi;
                }
            } else if (self.chr(vsi) == KEY_I && self.chr(vsi + 1) == KEY_A)
                || (self.chr(vsi) == KEY_Y && self.chr(vsi + 1) == KEY_A)
                || (self.chr(vsi) == KEY_U && self.chr(vsi + 1) == KEY_A)
                || (self.chr(vsi) == KEY_U && self.typing_word[vsi + 1] == (KEY_U as u32 | TONEW_MASK))
            {
                vwsm = vsi;
            }
        }

        // Rule 4
        if vowel_count == 2 {
            let (c0, c1) = (self.chr(vsi), self.chr(vsi + 1));
            if (c0 == KEY_I && c1 == KEY_A) || (c0 == KEY_I && c1 == KEY_U) || (c0 == KEY_I && c1 == KEY_O) {
                if vsi == 0 || self.chr(vsi - 1) != KEY_G {
                    vwsm = vsi;
                } else {
                    vwsm = vsi + 1;
                }
            } else if c0 == KEY_U && c1 == KEY_A {
                if vsi == 0 || self.chr(vsi - 1) != KEY_Q {
                    if vei + 1 >= self.index || !self.can_has_end_consonant(vsi, vei) {
                        vwsm = vsi;
                    }
                } else {
                    vwsm = vsi + 1;
                }
            } else if c0 == KEY_O && c1 == KEY_O {
                vwsm = vei;
            }
        }

        vwsm
    }

    pub(crate) fn handle_old_mark(&self, vsi: usize, vei: usize, vowel_count: usize) -> usize {
        let mut vwsm = if vowel_count == 0 && self.chr(vei) == KEY_I {
            vei
        } else {
            vsi
        };

        if vowel_count == 3
            || (vei + 1 < self.index && is_consonant(self.chr(vei + 1)) && self.can_has_end_consonant(vsi, vei))
        {
            vwsm = vsi + 1;
        }

        for ii in vsi..=vei {
            if (self.chr(ii) == KEY_E && (self.typing_word[ii] & TONE_MASK) != 0)
                || (self.chr(ii) == KEY_O && (self.typing_word[ii] & TONEW_MASK) != 0)
            {
                vwsm = ii;
                break;
            }
        }

        vwsm
    }
}
