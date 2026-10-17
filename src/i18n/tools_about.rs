//
// TurtKey - i18n Strings for Tools (Convert, Playground) & About
// Clean, professional multi-language support (Zero emojis)
//

use super::UiLanguage;

impl UiLanguage {
    // Convert tab
    pub fn convert_tool_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Công cụ chuyển đổi mã và kiểu chữ",
            UiLanguage::English => "Encoding & Case Conversion Tool",
        }
    }

    pub fn convert_source_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bảng mã nguồn:",
            UiLanguage::English => "Source charset:",
        }
    }

    pub fn convert_dest_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Đích:",
            UiLanguage::English => "Target:",
        }
    }

    pub fn convert_case_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Định dạng chữ:",
            UiLanguage::English => "Letter case:",
        }
    }

    pub fn case_none(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Giữ nguyên",
            UiLanguage::English => "Keep original",
        }
    }

    pub fn case_upper(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "CHỮ HOA",
            UiLanguage::English => "UPPERCASE",
        }
    }

    pub fn case_lower(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "chữ thường",
            UiLanguage::English => "lowercase",
        }
    }

    pub fn case_first_letter(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Hoa đầu câu",
            UiLanguage::English => "Sentence case",
        }
    }

    pub fn case_each_word(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Hoa Mỗi Từ",
            UiLanguage::English => "Capitalize Words",
        }
    }

    pub fn remove_marks_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bỏ dấu tiếng Việt (Remove accents)",
            UiLanguage::English => "Remove accents (Strip diacritics)",
        }
    }

    pub fn source_text_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Văn bản gốc:",
            UiLanguage::English => "Source text:",
        }
    }

    pub fn convert_now_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Chuyển mã ngay",
            UiLanguage::English => "Convert Now",
        }
    }

    pub fn paste_and_convert_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Dán từ Clipboard & Chuyển",
            UiLanguage::English => "Paste from Clipboard & Convert",
        }
    }

    pub fn result_text_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Kết quả chuyển đổi:",
            UiLanguage::English => "Converted result:",
        }
    }

    pub fn copy_result_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Sao chép kết quả",
            UiLanguage::English => "Copy Result",
        }
    }

    pub fn clear_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Xóa trắng",
            UiLanguage::English => "Clear",
        }
    }

    pub fn converted_and_copied(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Đã chuyển mã và sao chép vào Clipboard!",
            UiLanguage::English => "Converted and copied to Clipboard!",
        }
    }

    // Playground tab
    pub fn mode_label(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Chế độ:",
            UiLanguage::English => "Mode:",
        }
    }

    pub fn clear_playground_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Xóa khung gõ",
            UiLanguage::English => "Clear editor",
        }
    }

    pub fn playground_ready(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Sẵn sàng gõ thử nghiệm...",
            UiLanguage::English => "Ready for testing...",
        }
    }

    pub fn playground_cleared(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Đã làm sạch khung gõ.",
            UiLanguage::English => "Editor cleared.",
        }
    }

    pub fn playground_placeholder(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Nhấn vào đây và bắt đầu gõ thử... (VD: Tiếng Việt, đường trường, hoà bình, v.v.)",
            UiLanguage::English => "Click here and start typing to test... (e.g. Tiếng Việt, đường trường, hoà bình, etc.)",
        }
    }

    // About tab
    pub fn about_subtitle(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bản Rust thuần của bộ gõ Tiếng Việt OpenKey",
            UiLanguage::English => "Pure Rust Vietnamese Input Method based on OpenKey",
        }
    }

    pub fn version_label(&self, ver: &str) -> String {
        match self {
            UiLanguage::Vietnamese => format!("Phiên bản: {}", ver),
            UiLanguage::English => format!("Version: {}", ver),
        }
    }

    pub fn credit_openkey(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Dựa trên dự án OpenKey gốc của tác giả Mai Vũ Tuyên (maivutuyen.91@gmail.com).",
            UiLanguage::English => "Based on the original OpenKey project by Mai Vu Tuyen (maivutuyen.91@gmail.com).",
        }
    }

    pub fn license_text(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Phát triển mã nguồn mở theo giấy phép GNU General Public License v3.0 (GPL-3.0).",
            UiLanguage::English => "Open-source software licensed under GNU General Public License v3.0 (GPL-3.0).",
        }
    }

    pub fn highlights_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Đặc điểm nổi bật của TurtKey:",
            UiLanguage::English => "Key Features of TurtKey:",
        }
    }

    pub fn highlights(&self) -> &'static [&'static str] {
        match self {
            UiLanguage::Vietnamese => &[
                "• 100% Rust an toàn bộ nhớ, không phụ thuộc thư viện C phức tạp.",
                "• Win mode & Linux mode: giải quyết triệt để vấn đề nhảy/xóa chữ trên WSL/WSLg/Terminal.",
                "• Tự do tùy biến phím tắt (Hotkeys) với tính năng ghi phím trực tiếp.",
                "• Hỗ trợ đa ngôn ngữ giao diện (Tiếng Việt & English).",
                "• Đầy đủ 5 bảng mã: Unicode, TCVN3, VNI-Windows, Unicode Tổ hợp, CP 1258.",
                "• Đầy đủ 4 kiểu gõ: Telex, VNI, Simple Telex 1 & 2.",
                "• Tính năng gõ tắt (Macro) thông minh tự động viết hoa (Auto Caps).",
                "• Công cụ chuyển mã văn bản đa năng, chuyển đổi chữ HOA/thường/bỏ dấu.",
                "• Giao diện đồ họa egui mượt mà, hỗ trợ cả X11 và Wayland trên Linux.",
            ],
            UiLanguage::English => &[
                "• 100% memory-safe Rust, zero complex C dependencies.",
                "• Win mode & Linux mode: eliminates cursor jumps and backspace sync issues in WSL/WSLg/Terminal.",
                "• Fully customizable hotkeys with interactive keyboard recording.",
                "• Multi-language UI support (Vietnamese & English).",
                "• 5 character encodings: Unicode, TCVN3, VNI-Windows, Unicode Compound, CP 1258.",
                "• 4 input methods: Telex, VNI, Simple Telex 1 & 2.",
                "• Smart macro expansion with case-matching (Auto Caps).",
                "• All-in-one text converter: charset conversion, letter case, diacritics removal.",
                "• High-performance egui GUI with full X11 and Wayland support on Linux.",
            ],
        }
    }
}
