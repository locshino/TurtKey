//
// TurtKey - i18n Strings for Macro / Shorthand Tab
// Clean, professional multi-language support (Zero emojis)
//

use super::UiLanguage;

impl UiLanguage {
    // Macro tab
    pub fn macro_settings_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Cài đặt gõ tắt",
            UiLanguage::English => "Macro Settings",
        }
    }

    pub fn macro_enabled(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bật tính năng gõ tắt (Macro)",
            UiLanguage::English => "Enable shorthand macros",
        }
    }

    pub fn macro_allow_in_english(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Cho phép gõ tắt ngay cả khi ở chế độ Tiếng Anh",
            UiLanguage::English => "Allow macros in English mode",
        }
    }

    pub fn macro_auto_caps(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Tự động viết hoa theo từ viết tắt (VD: vn -> Việt Nam, VN -> VIỆT NAM)",
            UiLanguage::English => "Auto-match case (e.g. vn -> Việt Nam, VN -> VIỆT NAM)",
        }
    }

    pub fn quick_consonants_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gõ nhanh phụ âm & Telex",
            UiLanguage::English => "Quick Consonants & Telex",
        }
    }

    pub fn quick_start_consonant(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gõ tắt phụ âm đầu (f -> ph, j -> gi, w -> qu)",
            UiLanguage::English => "Quick initial consonants (f -> ph, j -> gi, w -> qu)",
        }
    }

    pub fn quick_end_consonant(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gõ tắt phụ âm cuối (g -> ng, h -> nh, k -> ch)",
            UiLanguage::English => "Quick ending consonants (g -> ng, h -> nh, k -> ch)",
        }
    }

    pub fn quick_telex(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gõ nhanh Telex (cc->ch, gg->gi, kk->kh, nn->ng, qq->qu, pp->ph, tt->th)",
            UiLanguage::English => "Quick Telex (cc->ch, gg->gi, kk->kh, nn->ng, qq->qu, pp->ph, tt->th)",
        }
    }

    pub fn macro_table_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bảng danh sách từ gõ tắt",
            UiLanguage::English => "Macro Dictionary",
        }
    }

    pub fn macro_short_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Từ viết tắt:",
            UiLanguage::English => "Shortcut:",
        }
    }

    pub fn macro_replace_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Thay thế bằng:",
            UiLanguage::English => "Replace with:",
        }
    }

    pub fn macro_add_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Thêm từ",
            UiLanguage::English => "Add word",
        }
    }

    pub fn col_shortcut(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Từ tắt",
            UiLanguage::English => "Shortcut",
        }
    }

    pub fn col_replacement(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Nội dung thay thế",
            UiLanguage::English => "Replacement",
        }
    }

    pub fn col_action(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Thao tác",
            UiLanguage::English => "Action",
        }
    }

    pub fn delete_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Xóa",
            UiLanguage::English => "Delete",
        }
    }

    pub fn macro_empty_error(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Vui lòng nhập đầy đủ từ viết tắt và nội dung thay thế!",
            UiLanguage::English => "Please enter both shortcut key and replacement text!",
        }
    }

    pub fn macro_added_success(&self, key: &str, val: &str) -> String {
        match self {
            UiLanguage::Vietnamese => format!("Đã thêm: {} -> {}", key, val),
            UiLanguage::English => format!("Added: {} -> {}", key, val),
        }
    }

    pub fn macro_mode_select_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bảng gõ tắt theo chế độ:",
            UiLanguage::English => "Macro table for mode:",
        }
    }

    pub fn macro_mode_game_hint(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gợi ý: Game mode có bảng gõ tắt riêng (gg, wp, ss, mid...) giúp game thủ chat nhanh trong trận đấu mà không làm xáo trộn từ tắt văn phòng.",
            UiLanguage::English => "Tip: Game mode has a dedicated macro table (gg, wp, ss, mid...) for fast in-game chat shortcuts without cluttering office macros.",
        }
    }
}
