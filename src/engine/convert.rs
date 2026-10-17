//
// TurtKey - Convert Tool (Công cụ chuyển mã và định dạng văn bản)
// Ported and enhanced from OpenKey ConvertTool.cpp
// Copyright (c) 2019 Mai Vu Tuyen (OpenKey)
// Rust rewrite (c) 2026 TurtKey
//

use super::tables::*;
use super::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseTransform {
    None,
    Uppercase,
    Lowercase,
    CapitalizeFirstLetter,
    CapitalizeEachWord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertOptions {
    pub from_code: CodeTable,
    pub to_code: CodeTable,
    pub case_transform: CaseTransform,
    pub remove_marks: bool,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            from_code: CodeTable::Unicode,
            to_code: CodeTable::Unicode,
            case_transform: CaseTransform::None,
            remove_marks: false,
        }
    }
}

pub fn remove_vietnamese_accents(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for ch in text.chars() {
        let mapped = match ch {
            'a' | 'á' | 'à' | 'ả' | 'ã' | 'ạ' | 'ă' | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' | 'â' | 'ấ'
            | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => 'a',
            'A' | 'Á' | 'À' | 'Ả' | 'Ã' | 'Ạ' | 'Ă' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Â' | 'Ấ'
            | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' => 'A',
            'd' | 'đ' => 'd',
            'D' | 'Đ' => 'D',
            'e' | 'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' | 'ê' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => 'e',
            'E' | 'É' | 'È' | 'Ẻ' | 'Ẽ' | 'Ẹ' | 'Ê' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' => 'E',
            'i' | 'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => 'i',
            'I' | 'Í' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Ị' => 'I',
            'o' | 'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' | 'ô' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' | 'ơ' | 'ớ'
            | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'o',
            'O' | 'Ó' | 'Ò' | 'Ỏ' | 'Õ' | 'Ọ' | 'Ô' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' | 'Ơ' | 'Ớ'
            | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' => 'O',
            'u' | 'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' | 'ư' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => 'u',
            'U' | 'Ú' | 'Ù' | 'Ủ' | 'Ũ' | 'Ụ' | 'Ư' | 'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' => 'U',
            'y' | 'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
            'Y' | 'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'Y',
            // Also filter combining marks
            '\u{0300}' | '\u{0301}' | '\u{0303}' | '\u{0309}' | '\u{0323}' => continue,
            other => other,
        };
        result.push(mapped);
    }
    result
}

/// Convert text between encodings and apply casing options
pub fn convert_text(source: &str, options: &ConvertOptions) -> String {
    if options.remove_marks && options.from_code == CodeTable::Unicode && options.to_code == CodeTable::Unicode {
        let no_accents = remove_vietnamese_accents(source);
        return apply_case_transform(&no_accents, options.case_transform);
    }

    // Keys that exist in code tables:
    let table_keys = [
        KEY_A as u32,
        KEY_O as u32,
        KEY_U as u32,
        KEY_E as u32,
        KEY_D as u32,
        (KEY_A as u32) | TONE_MASK,
        (KEY_A as u32) | TONEW_MASK,
        (KEY_O as u32) | TONE_MASK,
        (KEY_O as u32) | TONEW_MASK,
        (KEY_U as u32) | TONEW_MASK,
        (KEY_E as u32) | TONE_MASK,
        KEY_I as u32,
        KEY_Y as u32,
    ];

    let chars: Vec<char> = source.chars().collect();
    let mut result_chars: Vec<char> = Vec::with_capacity(chars.len());
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        let code_u16 = ch as u32;

        let mut found = false;
        let mut target_val: Option<u16> = None;

        // Try single char match in from_code
        for &k in &table_keys {
            if let Some(src_entries) = get_code_table_entry(options.from_code, k) {
                if let Some(pos) = src_entries.iter().position(|&val| val as u32 == code_u16) {
                    if let Some(dst_entries) = get_code_table_entry(options.to_code, k) {
                        if pos < dst_entries.len() {
                            target_val = Some(dst_entries[pos]);
                            found = true;
                            break;
                        }
                    }
                }
            }
        }

        if found {
            if let Some(tv) = target_val {
                if let Some(c) = char::from_u32(tv as u32) {
                    result_chars.push(c);
                } else {
                    result_chars.push(ch);
                }
            }
            i += 1;
        } else {
            result_chars.push(ch);
            i += 1;
        }
    }

    let mut output: String = result_chars.into_iter().collect();

    if options.remove_marks {
        output = remove_vietnamese_accents(&output);
    }

    apply_case_transform(&output, options.case_transform)
}

fn apply_case_transform(text: &str, transform: CaseTransform) -> String {
    match transform {
        CaseTransform::None => text.to_string(),
        CaseTransform::Uppercase => text.to_uppercase(),
        CaseTransform::Lowercase => text.to_lowercase(),
        CaseTransform::CapitalizeFirstLetter => {
            let mut result = String::with_capacity(text.len());
            let mut capitalize_next = true;
            for ch in text.chars() {
                if capitalize_next && ch.is_alphabetic() {
                    result.extend(ch.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                    if ch == '.' || ch == '?' || ch == '!' || ch == '\n' {
                        capitalize_next = true;
                    }
                }
            }
            result
        }
        CaseTransform::CapitalizeEachWord => {
            let mut result = String::with_capacity(text.len());
            let mut capitalize_next = true;
            for ch in text.chars() {
                if ch.is_whitespace() || ch == '-' || ch == '/' || ch == '(' {
                    capitalize_next = true;
                    result.push(ch);
                } else if capitalize_next && ch.is_alphabetic() {
                    result.extend(ch.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                }
            }
            result
        }
    }
}
