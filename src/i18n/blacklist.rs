//
// TurtKey - i18n Strings for Excluded Applications (Blacklist) Tab
// Clean, professional multi-language support (Zero emojis)
//

use super::UiLanguage;

impl UiLanguage {
    pub fn blacklist_title(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Quản lý ứng dụng loại trừ (Tự động tắt cho Game / App)",
            UiLanguage::English => "Excluded Applications Manager (Auto-Disable for Games/Apps)",
        }
    }

    pub fn blacklist_enable_checkbox(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Bật tính năng tự động nhận diện ứng dụng loại trừ",
            UiLanguage::English => "Enable auto-detection for excluded applications",
        }
    }

    pub fn blacklist_search_placeholder(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Nhập tên ứng dụng hoặc tiến trình để tìm kiếm (VD: cs2, valorant, steam, dota)...",
            UiLanguage::English => "Type application or process name to filter (e.g. cs2, valorant, steam, dota)...",
        }
    }

    pub fn blacklist_refresh_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Làm mới danh sách tiến trình",
            UiLanguage::English => "Refresh Running Processes",
        }
    }

    pub fn blacklist_browse_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Chọn file từ máy tính (Browse...)",
            UiLanguage::English => "Browse File on Computer (Browse...)",
        }
    }

    pub fn blacklist_manual_add_btn(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Thêm thủ công",
            UiLanguage::English => "Add Manual",
        }
    }

    pub fn blacklist_manual_hint(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Nhập tên file thực thi (VD: game.exe hoặc đường dẫn đầy đủ):",
            UiLanguage::English => "Enter executable file name (e.g. game.exe or full path):",
        }
    }

    pub fn blacklist_running_apps_header(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Các ứng dụng đang chạy trên hệ thống (Bấm để thêm ngay):",
            UiLanguage::English => "Currently Running Applications (Click to add directly):",
        }
    }

    pub fn blacklist_active_list_header(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Danh sách ứng dụng đang bị loại trừ:",
            UiLanguage::English => "Active Excluded Applications List:",
        }
    }

    pub fn blacklist_tip(&self) -> &'static str {
        match self {
            UiLanguage::Vietnamese => "Gợi ý: Khi bạn vào bất kỳ ứng dụng nào trong danh sách trên (ví dụ chơi game dùng WASD), bộ gõ sẽ tự động tắt tiếng Việt để phím đi thẳng vào game không bị khựng hay xóa chữ. Khi bạn chuyển lại Discord, trình duyệt hoặc văn bản, bộ gõ sẽ tự động bật lại tiếng Việt!",
            UiLanguage::English => "Tip: When you switch to any application in the list above (e.g. games using WASD), the IME automatically disables Vietnamese so movement keys pass through smoothly without lag or deletion. When you switch back to Discord, browser, or documents, Vietnamese is automatically restored!",
        }
    }
}
