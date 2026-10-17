//
// TurtKey - Vietnamese Grammar and Phonetics Tables
// Ported directly from OpenKey Vietnamese.cpp
//

use crate::engine::types::*;
use std::collections::HashMap;
use std::sync::OnceLock;

// Sắc, Huyền, Hỏi, Ngã, Nặng - for Unicode Compound
pub const UNICODE_COMPOUND_MARKS: [u16; 5] = [0x0301, 0x0300, 0x0309, 0x0303, 0x0323];

pub fn get_quick_telex_map() -> &'static HashMap<u32, &'static [u16]> {
    static MAP: OnceLock<HashMap<u32, &'static [u16]>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(KEY_C as u32, &[KEY_C, KEY_H][..]);
        m.insert(KEY_G as u32, &[KEY_G, KEY_I][..]);
        m.insert(KEY_K as u32, &[KEY_K, KEY_H][..]);
        m.insert(KEY_N as u32, &[KEY_N, KEY_G][..]);
        m.insert(KEY_Q as u32, &[KEY_Q, KEY_U][..]);
        m.insert(KEY_P as u32, &[KEY_P, KEY_H][..]);
        m.insert(KEY_T as u32, &[KEY_T, KEY_H][..]);
        m.insert(KEY_U as u32, &[KEY_U, KEY_U][..]);
        m
    })
}

pub fn get_quick_start_consonant_map() -> &'static HashMap<u16, &'static [u16]> {
    static MAP: OnceLock<HashMap<u16, &'static [u16]>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(KEY_F, &[KEY_P, KEY_H][..]);
        m.insert(KEY_J, &[KEY_G, KEY_I][..]);
        m.insert(KEY_W, &[KEY_Q, KEY_U][..]);
        m
    })
}

pub fn get_quick_end_consonant_map() -> &'static HashMap<u16, &'static [u16]> {
    static MAP: OnceLock<HashMap<u16, &'static [u16]>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(KEY_G, &[KEY_N, KEY_G][..]);
        m.insert(KEY_H, &[KEY_N, KEY_H][..]);
        m.insert(KEY_K, &[KEY_C, KEY_H][..]);
        m
    })
}

pub fn get_consonant_table() -> &'static [&'static [u16]] {
    &[
        &[KEY_N, KEY_G, KEY_H],
        &[KEY_P, KEY_H],
        &[KEY_T, KEY_H],
        &[KEY_T, KEY_R],
        &[KEY_G, KEY_I],
        &[KEY_C, KEY_H],
        &[KEY_N, KEY_H],
        &[KEY_N, KEY_G],
        &[KEY_K, KEY_H],
        &[KEY_G, KEY_H],
        &[KEY_G],
        &[KEY_C],
        &[KEY_Q],
        &[KEY_K],
        &[KEY_T],
        &[KEY_R],
        &[KEY_H],
        &[KEY_B],
        &[KEY_M],
        &[KEY_V],
        &[KEY_N],
        &[KEY_L],
        &[KEY_X],
        &[KEY_P],
        &[KEY_S],
        &[KEY_D],
        &[KEY_F | CONSONANT_ALLOW_MASK],
        &[KEY_W | CONSONANT_ALLOW_MASK],
        &[KEY_Z | CONSONANT_ALLOW_MASK],
        &[KEY_J | CONSONANT_ALLOW_MASK],
        &[KEY_F | END_CONSONANT_MASK],
        &[KEY_W | END_CONSONANT_MASK],
        &[KEY_J | END_CONSONANT_MASK],
    ]
}

pub fn get_end_consonant_table() -> &'static [&'static [u16]] {
    &[
        &[KEY_T],
        &[KEY_P],
        &[KEY_C],
        &[KEY_N],
        &[KEY_M],
        &[KEY_G | END_CONSONANT_MASK],
        &[KEY_K | END_CONSONANT_MASK],
        &[KEY_H | END_CONSONANT_MASK],
        &[KEY_C, KEY_H],
        &[KEY_N, KEY_H],
        &[KEY_N, KEY_G],
    ]
}

pub fn get_double_w_allowed() -> &'static [&'static [u16]] {
    &[
        &[KEY_T, KEY_R],
        &[KEY_T, KEY_H],
        &[KEY_C, KEY_H],
        &[KEY_N, KEY_H],
        &[KEY_N, KEY_G],
        &[KEY_K, KEY_H],
        &[KEY_G, KEY_I],
        &[KEY_P, KEY_H],
        &[KEY_G, KEY_H],
    ]
}

const KA: u32 = KEY_A as u32;
const KE: u32 = KEY_E as u32;
const KI: u32 = KEY_I as u32;
const KO: u32 = KEY_O as u32;
const KU: u32 = KEY_U as u32;
const KY: u32 = KEY_Y as u32;

static VOWEL_COMB_A: [&[u32]; 6] = [
    &[0, KA, KI],
    &[0, KA, KO],
    &[0, KA, KU],
    &[0, KA | TONE_MASK, KU],
    &[0, KA, KY],
    &[0, KA | TONE_MASK, KY],
];

static VOWEL_COMB_E: [&[u32]; 2] = [
    &[0, KE, KO],
    &[0, KE | TONE_MASK, KU],
];

static VOWEL_COMB_I: [&[u32]; 4] = [
    &[1, KI, KE | TONE_MASK, KU],
    &[0, KI, KA],
    &[1, KI, KE | TONE_MASK],
    &[0, KI, KU],
];

static VOWEL_COMB_O: [&[u32]; 12] = [
    &[0, KO, KA, KI],
    &[0, KO, KA, KO],
    &[0, KO, KA, KY],
    &[0, KO, KE, KO],
    &[1, KO, KA],
    &[1, KO, KA | TONEW_MASK],
    &[1, KO, KE],
    &[0, KO, KI],
    &[0, KO | TONE_MASK, KI],
    &[0, KO | TONEW_MASK, KI],
    &[1, KO, KO],
    &[1, KO | TONE_MASK, KO | TONE_MASK],
];

static VOWEL_COMB_U: [&[u32]; 21] = [
    &[0, KU, KY, KU],
    &[1, KU, KY, KE | TONE_MASK],
    &[0, KU, KY, KA],
    &[0, KU | TONEW_MASK, KO | TONEW_MASK, KU],
    &[0, KU | TONEW_MASK, KO | TONEW_MASK, KI],
    &[0, KU, KO | TONE_MASK, KI],
    &[0, KU, KA | TONE_MASK, KY],
    &[1, KU, KA, KO],
    &[1, KU, KA],
    &[1, KU, KA | TONEW_MASK],
    &[1, KU, KA | TONE_MASK],
    &[0, KU | TONEW_MASK, KA],
    &[1, KU, KE | TONE_MASK],
    &[0, KU, KI],
    &[0, KU | TONEW_MASK, KI],
    &[1, KU, KO],
    &[1, KU, KO | TONE_MASK],
    &[0, KU, KO | TONEW_MASK],
    &[1, KU | TONEW_MASK, KO | TONEW_MASK],
    &[0, KU | TONEW_MASK, KU],
    &[1, KU, KY],
];

static VOWEL_COMB_Y: [&[u32]; 2] = [
    &[0, KY, KE | TONE_MASK, KU],
    &[1, KY, KE | TONE_MASK],
];

pub fn get_vowel_combine_map() -> &'static HashMap<u16, &'static [&'static [u32]]> {
    static MAP: OnceLock<HashMap<u16, &'static [&'static [u32]]>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(KEY_A, &VOWEL_COMB_A[..]);
        m.insert(KEY_E, &VOWEL_COMB_E[..]);
        m.insert(KEY_I, &VOWEL_COMB_I[..]);
        m.insert(KEY_O, &VOWEL_COMB_O[..]);
        m.insert(KEY_U, &VOWEL_COMB_U[..]);
        m.insert(KEY_Y, &VOWEL_COMB_Y[..]);
        m
    })
}
