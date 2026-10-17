//
// TurtKey - Scenario 1: Happy Case Tests
// Verifies standard / successful operations for Telex, VNI, Macros, and Convert Tool
//

use turtkey::engine::convert::{convert_text, remove_vietnamese_accents, CaseTransform, ConvertOptions};
use turtkey::engine::macro_table::MacroManager;
use turtkey::engine::types::{CodeTable, InputType};
use turtkey::engine::VietnameseEngine;

#[test]
fn test_happy_telex_common_words() {
    let mut engine = VietnameseEngine::new();
    engine.input_type = InputType::Telex;

    // "tieengs" -> "tiếng"
    assert_eq!(engine.type_string("tieengs"), "tiếng");

    // "vieetj" -> "việt"
    engine.start_new_session();
    assert_eq!(engine.type_string("vieetj"), "việt");

    // "nam" -> "nam"
    engine.start_new_session();
    assert_eq!(engine.type_string("nam"), "nam");

    // "ddoong" -> "đông"
    engine.start_new_session();
    assert_eq!(engine.type_string("ddoong"), "đông");

    // "ddooj" -> "độ"
    engine.start_new_session();
    assert_eq!(engine.type_string("ddooj"), "độ");

    // "thuwowng" -> "thương"
    engine.start_new_session();
    assert_eq!(engine.type_string("thuwowng"), "thương");
}

#[test]
fn test_happy_vni_common_words() {
    let mut engine = VietnameseEngine::new();
    engine.input_type = InputType::Vni;

    // "tie6ng1" -> "tiếng"
    assert_eq!(engine.type_string("tie6ng1"), "tiếng");

    // "to6i" -> "tôi"
    engine.start_new_session();
    assert_eq!(engine.type_string("to6i"), "tôi");

    // "d9o6" -> "đô"
    engine.start_new_session();
    assert_eq!(engine.type_string("d9o6"), "đô");
}

#[test]
fn test_happy_modern_vs_old_orthography() {
    let mut engine = VietnameseEngine::new();
    engine.input_type = InputType::Telex;

    // Modern: "hoaf" -> "hoà"
    engine.use_modern_orthography = true;
    engine.start_new_session();
    assert_eq!(engine.type_string("hoaf"), "hoà");

    // Old: "hoaf" -> "hòa"
    engine.use_modern_orthography = false;
    engine.start_new_session();
    assert_eq!(engine.type_string("hoaf"), "hòa");
}

#[test]
fn test_happy_macro_expansion_and_autocaps() {
    let mut mm = MacroManager::new();
    mm.add("vn", "Việt Nam");
    mm.add("ok", "OpenKey");

    // Lowercase
    assert_eq!(mm.lookup("vn"), Some("Việt Nam".to_string()));

    // ALL CAPS
    assert_eq!(mm.lookup("VN"), Some("VIỆT NAM".to_string()));

    // Capitalized first char
    assert_eq!(mm.lookup("Vn"), Some("Việt Nam".to_string()));
}

#[test]
fn test_happy_convert_tool() {
    let text = "Cộng hoà Xã hội Chủ nghĩa Việt Nam";
    let no_accents = remove_vietnamese_accents(text);
    assert_eq!(no_accents, "Cong hoa Xa hoi Chu nghia Viet Nam");

    let opts = ConvertOptions {
        from_code: CodeTable::Unicode,
        to_code: CodeTable::Unicode,
        case_transform: CaseTransform::Uppercase,
        remove_marks: true,
    };
    let converted = convert_text(text, &opts);
    assert_eq!(converted, "CONG HOA XA HOI CHU NGHIA VIET NAM");
}

#[test]
fn test_happy_win_and_linux_modes() {
    use turtkey::engine::tables::char_to_key_code;
    use turtkey::engine::types::TargetMode;

    let mut engine = VietnameseEngine::new();

    // Default is Win mode (0ms delay)
    assert_eq!(engine.target_mode, TargetMode::Win);
    assert_eq!(engine.target_mode.display_name(), "Win mode");

    let (code_a, caps_a) = char_to_key_code('a').unwrap();
    engine.process_key(code_a, caps_a, false);
    let (code_s, caps_s) = char_to_key_code('s').unwrap();
    let hook_win = engine.process_key(code_s, caps_s, false);
    assert_eq!(hook_win.target_mode, TargetMode::Win);
    assert_eq!(hook_win.backspace_delay_ms, 0);

    // Switch to Linux mode (for WSL / Terminal)
    engine.start_new_session();
    engine.toggle_target_mode();
    assert_eq!(engine.target_mode, TargetMode::Linux);
    assert_eq!(engine.target_mode.display_name(), "Linux mode");

    engine.process_key(code_a, caps_a, false);
    let hook_linux = engine.process_key(code_s, caps_s, false);
    assert_eq!(hook_linux.target_mode, TargetMode::Linux);
    // Linux mode has backspace micro-delay for terminal PTY buffer redraw
    assert!(hook_linux.backspace_delay_ms > 0);

    // Typing in Linux mode correctly produces Vietnamese without corruption
    engine.start_new_session();
    assert_eq!(engine.type_string("tieengs"), "tiếng");
}

#[test]
fn test_happy_custom_hotkeys() {
    use turtkey::engine::types::HotkeyConfig;

    // Presets
    let ctrl_shift = HotkeyConfig::ctrl_shift();
    assert_eq!(ctrl_shift.display_name(), "Ctrl + Shift");
    assert!(ctrl_shift.matches(true, true, false, false, None));

    let alt_z = HotkeyConfig::alt_z();
    assert_eq!(alt_z.display_name(), "Alt + Z");
    assert!(alt_z.matches(false, false, true, false, Some('z')));
    assert!(alt_z.matches(false, false, true, false, Some('Z')));

    let win_space = HotkeyConfig::win_space();
    assert_eq!(win_space.display_name(), "Win + Space");
    assert!(win_space.matches(false, false, false, true, Some(' ')));

    let alt_m = HotkeyConfig::alt_m();
    assert_eq!(alt_m.display_name(), "Alt + M");
    assert!(alt_m.matches(false, false, true, false, Some('M')));

    let alt_c = HotkeyConfig::alt_c();
    assert_eq!(alt_c.display_name(), "Alt + C");
    assert!(alt_c.matches(false, false, true, false, Some('C')));

    // Custom user combination
    let custom = HotkeyConfig::new(true, true, true, false, Some('K'));
    assert_eq!(custom.display_name(), "Ctrl + Shift + Alt + K");
    assert!(custom.matches(true, true, true, false, Some('k')));
}

#[test]
fn test_happy_multi_language_i18n() {
    use turtkey::i18n::UiLanguage;
    use turtkey::engine::types::TargetMode;

    let vi = UiLanguage::Vietnamese;
    let en = UiLanguage::English;

    // Title has zero emojis
    assert_eq!(vi.title(), "TurtKey");
    assert_eq!(en.title(), "TurtKey");

    // Tab names
    assert_eq!(vi.tab_general(), "Bộ gõ");
    assert_eq!(en.tab_general(), "Engine");
    assert_eq!(vi.tab_macro(), "Gõ tắt");
    assert_eq!(en.tab_macro(), "Macros");
    assert_eq!(vi.tab_convert(), "Chuyển mã");
    assert_eq!(en.tab_convert(), "Convert");

    // Mode descriptions in both languages
    assert!(TargetMode::Win.description_with_lang(vi).contains("Windows"));
    assert!(TargetMode::Win.description_with_lang(en).contains("Windows"));
    assert!(TargetMode::Linux.description_with_lang(vi).contains("WSL"));
    assert!(TargetMode::Linux.description_with_lang(en).contains("WSL"));

    // Ensure zero emoji characters in all highlights
    for line in vi.highlights() {
        assert!(!line.chars().any(|c| (c as u32) >= 0x1F000));
    }
    for line in en.highlights() {
        assert!(!line.chars().any(|c| (c as u32) >= 0x1F000));
    }
}
