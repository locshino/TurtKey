//
// TurtKey - Spelling and Grammar Engine
// Ported directly from OpenKey Engine.cpp (checkSpelling & checkGrammar)
// Copyright (c) 2019 Mai Vu Tuyen (OpenKey)
// Rust rewrite (c) 2026 TurtKey
//

use super::tables::*;
use super::types::*;

pub struct SpellingChecker;

impl SpellingChecker {
    /// Validates whether the typed word buffer forms a valid Vietnamese syllable structure.
    pub fn is_spelling_ok(
        typing_word: &[u32],
        len: usize,
        quick_start_consonant: bool,
        quick_end_consonant: bool,
        allow_zfwj: bool,
        force_check_vowel: bool,
    ) -> (bool, bool) {
        if len == 0 {
            return (true, true);
        }

        let mut spelling_end_index = len;
        if len > 0 && ((typing_word[len - 1] & CHAR_MASK) as u16) == KEY_RIGHT_BRACKET {
            spelling_end_index = len - 1;
        }

        if spelling_end_index == 0 {
            return (true, true);
        }

        let first_char = (typing_word[0] & CHAR_MASK) as u16;
        let mut j = 0;

        // 1. Check initial consonant
        if is_consonant(first_char) {
            let consonant_table = get_consonant_table();
            let mut matched = false;

            for row in consonant_table {
                if spelling_end_index < row.len() {
                    continue;
                }
                let mut row_match = true;
                for idx in 0..row.len() {
                    if idx < spelling_end_index {
                        let cur_c = (typing_word[idx] & CHAR_MASK) as u16;
                        let mut expected = row[idx];
                        if quick_start_consonant {
                            expected &= !END_CONSONANT_MASK;
                        }
                        if allow_zfwj {
                            expected &= !CONSONANT_ALLOW_MASK;
                        }
                        if expected != cur_c {
                            row_match = false;
                            break;
                        }
                    }
                }
                if row_match {
                    j = row.len();
                    matched = true;
                    break;
                }
            }

            if !matched {
                return (false, false);
            }
        }

        if j == spelling_end_index {
            // Syllables with only consonants (like initial 'd' before vowel)
            return (true, true);
        }

        // 2. Check vowel sequence
        let mut k = j;
        let mut vsi = k;

        // Fix case "qu" + vowel
        if (typing_word[vsi] & CHAR_MASK) as u16 == KEY_U
            && k > 0
            && k < spelling_end_index
            && (typing_word[vsi - 1] & CHAR_MASK) as u16 == KEY_Q
        {
            k += 1;
            j = k;
            vsi = k;
        } else if len >= 2
            && (typing_word[0] & CHAR_MASK) as u16 == KEY_G
            && (typing_word[1] & CHAR_MASK) as u16 == KEY_I
            && (len == 2 || is_consonant((typing_word[2] & CHAR_MASK) as u16))
        {
            // Case "gìn", "giá"
            k = 1;
            j = 1;
            vsi = 1;
        }

        let mut vowel_count = 0;
        let mut _vei = k;
        while k < spelling_end_index && vowel_count < 3 {
            let ch = (typing_word[k] & CHAR_MASK) as u16;
            if !is_consonant(ch) {
                k += 1;
                _vei = k;
                vowel_count += 1;
            } else {
                break;
            }
        }

        if k > j {
            // Has vowel
            let mut spelling_vowel_ok = false;
            let v_comb = get_vowel_combine_map();
            let first_vowel_key = (typing_word[vsi] & CHAR_MASK) as u16;

            if k - j > 1 && force_check_vowel {
                if let Some(vowel_sets) = v_comb.get(&first_vowel_key) {
                    for vset in *vowel_sets {
                        let mut v_match = true;
                        let target_len = vset.len() - 1; // first element is can_has_end_consonant flag
                        for ii in 1..vset.len() {
                            let curr_idx = j + ii - 1;
                            if curr_idx < spelling_end_index {
                                let w = typing_word[curr_idx];
                                let actual = ((w & CHAR_MASK) as u32)
                                    | (w & TONEW_MASK)
                                    | (w & TONE_MASK);
                                if vset[ii] != actual {
                                    v_match = false;
                                    break;
                                }
                            } else {
                                v_match = false;
                                break;
                            }
                        }

                        if v_match {
                            if k < spelling_end_index && vset[0] == 0 {
                                continue;
                            }
                            if j + target_len < spelling_end_index
                                && !is_consonant((typing_word[j + target_len] & CHAR_MASK) as u16)
                            {
                                continue;
                            }
                            spelling_vowel_ok = true;
                            break;
                        }
                    }
                }
            } else if !is_consonant(first_vowel_key) {
                spelling_vowel_ok = true;
            }

            // 3. Check final consonant
            let mut spelling_ok = false;
            let end_consonant_table = get_end_consonant_table();

            for row in end_consonant_table {
                let mut row_match = true;
                for (idx, &item) in row.iter().enumerate() {
                    let cur_idx = k + idx;
                    if spelling_end_index > cur_idx {
                        let cur_c = (typing_word[cur_idx] & CHAR_MASK) as u16;
                        let mut expected = item;
                        if quick_end_consonant {
                            expected &= !END_CONSONANT_MASK;
                        }
                        if expected != cur_c {
                            row_match = false;
                            break;
                        }
                    } else {
                        row_match = false;
                        break;
                    }
                }

                if row_match && (k + row.len() >= spelling_end_index) {
                    spelling_ok = true;
                    break;
                }
            }

            if k == spelling_end_index {
                // Syllable ends with vowel
                spelling_ok = true;
            }

            // Vietnamese phonotactic rule: syllables ending in -c, -ch, -p, -t only take Sắc (MARK1) or Nặng (MARK5)
            if spelling_ok {
                let last_c = (typing_word[len - 1] & CHAR_MASK) as u16;
                let second_last_c = if len >= 2 {
                    (typing_word[len - 2] & CHAR_MASK) as u16
                } else {
                    0
                };

                let ends_in_stop = (len >= 3 && last_c == KEY_H && second_last_c == KEY_C)
                    || (last_c == KEY_T || last_c == KEY_P || last_c == KEY_C);

                if ends_in_stop {
                    // Check all vowels in buffer: if any has mark, it must only be MARK1 or MARK5
                    for i in 0..len {
                        let m = typing_word[i] & MARK_MASK;
                        if m != 0 && m != MARK1_MASK && m != MARK5_MASK {
                            spelling_ok = false;
                            break;
                        }
                    }
                }
            }

            (spelling_ok, spelling_vowel_ok)
        } else {
            (true, true)
        }
    }
}
