//
// TurtKey - i18n Strings for General Settings & Modes
// Clean, professional multi-language support (Zero emojis)
//

use super::UiLanguage;

impl UiLanguage {
    // General tab
    pub fn core_settings(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Cấu hình chính",
            UiLanguage::English => "Core Settings",
        }
    }

    pub fn input_method_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Kiểu gõ:",
            UiLanguage::English => "Input Method:",
        }
    }

    pub fn code_table_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bảng mã:",
            UiLanguage::English => "Character Set:",
        }
    }

    pub fn ui_language_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Ngôn ngữ giao diện:",
            UiLanguage::English => "UI Language:",
        }
    }

    // Hotkey section
    pub fn hotkey_section_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Cấu hình phím tắt (Custom Hotkeys)",
            UiLanguage::English => "Custom Shortcuts (Hotkeys)",
        }
    }

    pub fn recording_banner(&self, target_name: &str) -> String {
        match self {
            UiLanguage::Vietnamese => format!("Đang chờ nhấn phím cho: {}...", target_name),
            UiLanguage::English => format!("Waiting for key combination for: {}...", target_name),
        }
    }

    pub fn cancel_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Hủy (Esc)",
            UiLanguage::English => "Cancel (Esc)",
        }
    }

    pub fn recording_hint(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Nhấn tổ hợp phím bất kỳ trên bàn phím để gán ngay...",
            UiLanguage::English => "Press any key combination on keyboard to assign...",
        }
    }

    pub fn lang_switch_hotkey_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Chuyển Tiếng Việt [V] / [E]:",
            UiLanguage::English => "Toggle Vietnamese [V] / [E]:",
        }
    }

    pub fn mode_switch_hotkey_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Chuyển Win / Linux mode:",
            UiLanguage::English => "Toggle Win / Linux mode:",
        }
    }

    pub fn convert_hotkey_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Mở Chuyển mã (Convert):",
            UiLanguage::English => "Open Convert Tool:",
        }
    }

    pub fn recording_status(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Đang ghi...",
            UiLanguage::English => "Recording...",
        }
    }

    pub fn record_prefix(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Ghi phím:",
            UiLanguage::English => "Record:",
        }
    }

    pub fn hotkey_hint(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gợi ý: Bấm nút Ghi phím để gán tổ hợp phím bất kỳ hoặc bấm nút chọn nhanh phím tắt mẫu.",
            UiLanguage::English => "Tip: Click Record button to assign any combination or pick a quick preset.",
        }
    }

    // Mode section
    pub fn target_mode_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Chế độ môi trường",
            UiLanguage::English => "Target Environment Mode",
        }
    }

    pub fn win_mode_radio(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Win mode (Windows)",
            UiLanguage::English => "Win mode (Windows)",
        }
    }

    pub fn linux_mode_radio(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Linux mode (WSL / Terminal / WSLg)",
            UiLanguage::English => "Linux mode (WSL / Terminal / WSLg)",
        }
    }

    pub fn game_mode_radio(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Game mode (Chơi game / Không can thiệp phím)",
            UiLanguage::English => "Game mode (Gaming / Pure keystrokes)",
        }
    }

    pub fn target_mode_hint(&self, hotkey: &str) -> String {
        match self {
            UiLanguage::Vietnamese => format!("Gợi ý: Nhấn {} hoặc bấm nút trên thanh tiêu đề để chuyển đổi tức thì giữa Win mode, Linux mode và Game mode.", hotkey),
            UiLanguage::English => format!("Tip: Press {} or click title bar button to toggle instantly between Win, Linux, and Game modes.", hotkey),
        }
    }

    // Typing options section
    pub fn typing_options_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Tùy chọn gõ tiếng Việt",
            UiLanguage::English => "Typing Options",
        }
    }

    pub fn opt_modern_orthography(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Đặt dấu kiểu mới (oà, uý thay vì òa, úy)",
            UiLanguage::English => "Modern tone placement (oà, uý instead of òa, úy)",
        }
    }

    pub fn opt_spelling_check(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bật kiểm tra chính tả tiếng Việt",
            UiLanguage::English => "Enable Vietnamese spelling checker",
        }
    }

    pub fn opt_restore_if_wrong(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Tự động phục hồi phím nếu gõ từ sai",
            UiLanguage::English => "Auto restore keystrokes on spelling errors",
        }
    }

    pub fn opt_allow_zfwj(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Cho phép dùng Z, F, W, J làm phụ âm đầu",
            UiLanguage::English => "Allow Z, F, W, J as initial consonants",
        }
    }

    pub fn opt_auto_caps_sentence(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Tự động viết hoa chữ cái đầu câu",
            UiLanguage::English => "Auto capitalize first letter of sentences",
        }
    }
}
