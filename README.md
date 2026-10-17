# TurtKey - Bộ gõ Tiếng Việt nguồn mở bằng Rust (Dựa trên OpenKey)

**TurtKey** là phiên bản viết lại 100% bằng ngôn ngữ **Rust** hiện đại của bộ gõ Tiếng Việt nổi tiếng [OpenKey](https://github.com/tuyenvm/OpenKey) (tác giả Mai Vũ Tuyên), tích hợp giao diện người dùng **Rust GUI** trực quan xây dựng trên nền tảng **eframe / egui**. Ứng dụng sở hữu thiết kế tối giản, sạch sẽ (không sử dụng emoji) và hỗ trợ đa ngôn ngữ giao diện (Tiếng Việt & English).

---

## Tính năng nổi bật

### 1. Đầy đủ các kiểu gõ thông dụng
- **Telex**: Đặt dấu `s, f, r, x, j`, mũ `aa, ee, oo`, râu `w, ow, uw`, chữ đ `dd`, xóa dấu `z`.
- **VNI**: Phím số `1-5` cho dấu thanh, `6` (mũ â, ê, ô), `7` (râu ơ, ư), `8` (ă), `9` (đ), `0` (xóa dấu).
- **Simple Telex 1 & 2**.

### 2. Quy tắc chính tả & ngữ âm tiếng Việt
- **Đặt dấu kiểu mới (Modern orthography)**: Tuỳ chọn đặt dấu kiểu mới (`oà, uý`) hoặc kiểu cũ (`òa, úy`).
- **Kiểm tra chính tả (Spelling check)**: Phân tích cấu trúc phụ âm đầu + tổ hợp nguyên âm + phụ âm cuối theo âm vị học tiếng Việt.
- **Tự động phục hồi phím (Restore key)**: Khôi phục phím nếu gõ từ sai chính tả.
- **Phụ âm đầu mở rộng**: Tuỳ chọn cho phép các phụ âm ngoại lai `Z, F, W, J`.
- **Gõ tắt phụ âm & Telex nhanh**:
  - Phụ âm đầu: `f -> ph`, `j -> gi`, `w -> qu`
  - Phụ âm cuối: `g -> ng`, `h -> nh`, `k -> ch`
  - Telex nhanh: `cc -> ch`, `gg -> gi`, `kk -> kh`, `nn -> ng`, `qq -> qu`, `pp -> ph`, `tt -> th`

### 3. Hệ thống gõ tắt (Macro) thông minh theo từng Mode
- Mặc định khởi đầu sạch hoàn toàn (không chứa dữ liệu gõ tắt tùy biến có sẵn), cho phép người dùng tự do định nghĩa theo nhu cầu.
- Hỗ trợ thêm/xóa/sửa từ viết tắt độc lập theo từng chế độ (**Win mode**, **Linux mode**, và **Game mode**).
- **Tự động viết hoa (Auto Caps)**:
  - `vn` -> `Việt Nam`
  - `Vn` -> `Việt Nam` (Viết hoa chữ đầu)
  - `VN` -> `VIỆT NAM` (Viết hoa toàn bộ)
- Tương thích định dạng file gõ tắt của OpenKey (`key:replacement`).

### 4. Bảng mã & Công cụ chuyển mã (Convert Tool)
- Hỗ trợ 5 bảng mã tiếng Việt phổ biến:
  1. **Unicode (Dựng sẵn - Precomposed)**
  2. **TCVN3 (ABC)**
  3. **VNI-Windows**
  4. **Unicode Tổ hợp (Compound / Decomposed)**
  5. **Vietnamese Locale CP 1258**
- Công cụ chuyển mã đa năng:
  - Chuyển đổi qua lại giữa các bảng mã.
  - Chuyển đổi định dạng chữ: **CHỮ HOA**, **chữ thường**, **Hoa Đầu Câu**, **Hoa Mỗi Từ**.
  - **Bỏ dấu tiếng Việt** tự động.
  - Tích hợp Clipboard: Dán và chuyển mã nhanh.

### 5. Chuyển đổi linh hoạt: Win mode, Linux mode & Game mode
- **Win mode**: Dành cho các phần mềm Windows tiêu chuẩn (tốc độ gửi phím tối đa 0ms delay).
- **Linux mode**: Dành riêng cho **WSL, Windows Terminal, WSLg (GUI app qua RDP rail)**:
  - Tự động kích hoạt cơ chế đệm đồng bộ **Micro-delay (8ms - 10ms)** giữa các lệnh Backspace và ký tự Unicode mới.
  - Ngăn chặn hoàn toàn hiện tượng lệch xung nhịp PTY làm **nhảy xóa chữ, mất ký tự hoặc lặp chữ**.
- **Game mode (Chuyên dụng cho game thủ)**:
  - **Không can thiệp ký tự**: Phím bấm (WASD, chiêu thức, phím số...) đi thẳng tới game với độ trễ 0ms, không bị biến đổi dấu tiếng Việt hay kẹt phím di chuyển.
  - **Bảng gõ tắt theo mode riêng biệt**: Game thủ cấu hình shortcut chat nhanh trong trận (ví dụ: `gg -> Good game well played`, `wp -> Well played`, `mid -> Enemy missing middle`, `b -> Back! Be careful`...) mà không ảnh hưởng tới bảng gõ tắt văn phòng.
- Chuyển đổi tức thì chỉ bằng 1 nút bấm trên thanh tiêu đề hoặc phím tắt nhanh **`Alt + M`**.

### 6. Tự do cấu hình phím tắt (Customizable Hotkeys)
- Cho phép người dùng tùy biến 100% các tổ hợp phím tắt:
  - **Phím chuyển [V] / [E]**: Mặc định `Ctrl + Shift` (Hỗ trợ đổi sang `Alt + Z`, `Win + Space`, hoặc bất kỳ phím nào).
  - **Phím chuyển Win mode / Linux mode**: Mặc định `Alt + M` (Hỗ trợ đổi sang `Ctrl + Alt + L`, `Alt + W`, v.v.).
  - **Phím mở nhanh Chuyển mã (Convert Tool)**: Mặc định `Alt + C` (Hỗ trợ `Ctrl + Shift + C`, v.v.).
- **Tính năng Ghi phím tắt trực tiếp (Interactive Recorder)**: Bấm nút `Ghi phím`, gõ bất kỳ tổ hợp phím nào trên bàn phím là hệ thống tự động ghi nhận ngay lập tức, hoặc bấm các nút preset tạo sẵn một chạm.

### 7. Hệ thống Plugins Ngôn ngữ Giao diện (UI i18n Plugins)
- Nhằm giữ ứng dụng đơn giản và tinh gọn, mặc định TurtKey chỉ kích hoạt 2 ngôn ngữ: **Tiếng Việt (`vi`)** và **English (`en`)**.
- Hỗ trợ kiến trúc **Plugin Gói Ngôn ngữ (Language Packs)** độc lập và mở rộng:
  - Đi kèm sẵn các gói ngôn ngữ tùy chọn: Français, 日本語, Deutsch, Español.
  - **Cơ chế nạp động từ file JSON**: Người dùng hoặc cộng đồng có thể tạo file `.json` ngôn ngữ bất kỳ (tiếng Ý, Hàn, Nga, Thái...) và nạp trực tiếp qua giao diện hoặc thư mục plugins mà không cần sửa mã nguồn hay build lại ứng dụng.
  - **Fallback an toàn 3 tầng**: Ngôn ngữ được chọn -> Ngôn ngữ gốc -> Tên khóa (Key name), đảm bảo giao diện không bao giờ bị lỗi hiển thị hay crash.

### 8. Kiến trúc Plugin Bộ gõ Đa Ngôn ngữ (Input Engine Plugins)
- Mặc định chỉ kích hoạt 2 bộ gõ cốt lõi: **Tiếng Việt** (Telex/VNI/Simple Telex) và **English** (Passthrough).
- Hỗ trợ mở rộng các bộ gõ nhập liệu độc lập thông qua trait `InputEnginePlugin`:
  - **Tiếng Việt**: Bộ gõ Telex/VNI/Simple Telex, gõ tắt và kiểm tra chính tả.
  - **English (Passthrough)**: Chế độ gõ ký tự gốc không biến đổi.
  - Hỗ trợ cắm thêm các plugin ngôn ngữ mở rộng (Français, Deutsch, Español,...) tùy theo nhu cầu.
  - Dễ dàng cắm thêm các engine ngôn ngữ khác bằng cách triển khai trait `InputEnginePlugin`.

### 9. Giao diện Rust UI (eframe / egui)
- Giao diện đồ họa đa nền tảng (Linux Wayland & X11, Windows, macOS).
- Bảng điều khiển gồm 6 tab chức năng:
  - **Bộ gõ (Engine)**: Cấu hình kiểu gõ, bảng mã, ngôn ngữ giao diện (i18n), bộ gõ nhập liệu (Input Engine), tùy biến phím tắt [V]/[E] & Win/Linux mode, tùy chọn dấu hiện đại, kiểm tra chính tả.
  - **Gõ tắt (Macros)**: Quản lý danh sách Macro, gõ nhanh phụ âm.
  - **Chuyển mã (Convert)**: Công cụ chuyển đổi bảng mã văn bản và định dạng chữ.
  - **Thử nghiệm (Playground)**: Khung gõ trực tiếp tương tác với Rust Engine và các Engine Plugins để kiểm chứng kết quả ngay tức thì.
  - **Plugins**: Quản lý danh sách các gói ngôn ngữ UI đã nạp, danh sách các bộ gõ nhập liệu và công cụ nạp Language Pack JSON tùy biến.
  - **Thông tin (About)**: Thông tin bản quyền, giấy phép, tác giả.

---

## Kiến trúc mã nguồn

```
TurtKey/
├── Cargo.toml               # Cấu hình dependency (eframe, egui, serde, serde_json, arboard)
├── src/
│   ├── lib.rs               # Library root export engine, app, i18n, plugins
│   ├── main.rs              # Binary entrypoint khởi chạy GUI
│   ├── i18n.rs              # Mô-đun đa ngôn ngữ gốc (Tiếng Việt & English, Zero emojis)
│   ├── app.rs               # Giao diện người dùng egui (6 tabs, tích hợp plugin selector)
│   ├── plugins/
│   │   ├── mod.rs           # Plugin architecture root
│   │   ├── i18n.rs          # UI i18n Plugin system (LanguagePack, I18nManager, JSON loader, 6 ngôn ngữ)
│   │   └── input_engine.rs  # Input Engine Plugins (Vietnamese, French, German, Spanish, English)
│   └── engine/
│       ├── mod.rs           # State machine VietnameseEngine & vòng lặp xử lý phím
│       ├── types.rs         # Định nghĩa Key code, Mask, InputType, CodeTable, HookState, HotkeyConfig
│       ├── tables.rs        # Bảng ánh xạ ký tự, nguyên âm, phụ âm, bảng mã chuyển đổi
│       ├── spelling.rs      # Bộ kiểm tra chính tả & ngữ pháp tiếng Việt
│       ├── macro_table.rs   # Quản lý gõ tắt & Auto Caps
│       └── convert.rs       # Công cụ chuyển đổi mã và bỏ dấu
└── tests/
    ├── test_happy.rs        # Kịch bản 1: Happy Case (Telex, VNI, Orthography, Macro, Convert, Hotkeys, i18n)
    ├── test_exception.rs    # Kịch bản 2: Exception Case (Input rỗng, lỗi buffer, file lỗi, hotkey rỗng)
    ├── test_hacking.rs      # Kịch bản 3: Hacking Case (Spam phím, XSS/SQL payload, anti-eavesdropping)
    ├── test_edge_403.rs     # Kịch bản 4: 403 / Access Control (Tắt bộ gõ, cô lập phím tắt)
    └── test_plugins.rs      # Kịch bản 4 chuẩn: UI i18n Plugins & Input Engine Plugins (Happy-Exception-Hacking-403)
```

---

## Hướng dẫn cài đặt & chạy ứng dụng

### Yêu cầu hệ thống
- Rust toolchain (Rust 1.75+ hoặc mới hơn, khuyến nghị 1.92+)
- `cargo`

### Biên dịch và chạy ứng dụng GUI:
```bash
cd TurtKey
cargo run --release
```

### Chạy bộ kiểm thử tiêu chuẩn (4 Kịch bản):
```bash
cargo test --test test_happy --test test_exception --test test_hacking --test test_edge_403
```

---

## Bản quyền & Ghi nhận (Credits)

- **Nguyên bản**: Dự án [OpenKey](https://github.com/tuyenvm/OpenKey) của tác giả **Mai Vũ Tuyên** (`maivutuyen.91@gmail.com`).
- **Bản chuyển đổi Rust (TurtKey)**: Phát triển và duy trì mã nguồn mở dưới giấy phép **GNU General Public License v3.0 (GPL-3.0)**.
