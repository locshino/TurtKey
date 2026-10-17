//
// TurtKey - Scenario 2: Exception Case Tests
// Verifies proper handling of faulty inputs, boundaries, empty buffers, and file exceptions
//

use turtkey::engine::convert::{convert_text, remove_vietnamese_accents, ConvertOptions};
use turtkey::engine::macro_table::MacroManager;
use turtkey::engine::types::{HookCodeState, KEY_DELETE};
use turtkey::engine::VietnameseEngine;

#[test]
fn test_exception_empty_inputs() {
    let mut engine = VietnameseEngine::new();
    assert_eq!(engine.type_string(""), "");

    let mm = MacroManager::new();
    assert_eq!(mm.lookup(""), None);

    let opts = ConvertOptions::default();
    assert_eq!(convert_text("", &opts), "");
    assert_eq!(remove_vietnamese_accents(""), "");
}

#[test]
fn test_exception_repeated_backspaces_on_empty_buffer() {
    let mut engine = VietnameseEngine::new();
    // Simulate 50 consecutive backspaces on a freshly initialized buffer
    for _ in 0..50 {
        let hook = engine.process_key(KEY_DELETE, false, false);
        assert_eq!(hook.code, HookCodeState::DoNothing);
    }
    // Afterwards, normal typing should still work flawlessly
    assert_eq!(engine.type_string("ba"), "ba");
}

#[test]
fn test_exception_unmapped_invalid_keycodes() {
    let mut engine = VietnameseEngine::new();
    // 9999 is outside valid key code range
    let hook = engine.process_key(9999, false, false);
    assert_eq!(hook.code, HookCodeState::DoNothing);
}

#[test]
fn test_exception_macro_invalid_entries_and_missing_file() {
    let mut mm = MacroManager::new();
    // Adding empty key or replacement must be rejected
    assert!(!mm.add("", "valid"));
    assert!(!mm.add("valid", ""));
    assert!(!mm.add("   ", "   "));

    // Attempting to import from non-existent path returns std::io::Error instead of panicking
    let result = mm.import_from_file("/invalid/path/that/does/not/exist.txt", false);
    assert!(result.is_err());
}

#[test]
fn test_exception_buffer_overflow_wrap() {
    let mut engine = VietnameseEngine::new();
    // Type 60 characters without breaking word (MAX_BUFF is 32)
    let long_word = "b".repeat(60);
    let output = engine.type_string(&long_word);
    assert_eq!(output.len(), 60);
}

#[test]
fn test_exception_hotkey_empty_and_mismatched() {
    use turtkey::engine::types::HotkeyConfig;

    // Completely empty hotkey config
    let empty_hotkey = HotkeyConfig::new(false, false, false, false, None);
    assert_eq!(empty_hotkey.display_name(), "Chưa đặt");

    // Empty hotkey should not match any active modifiers or keys
    assert!(!empty_hotkey.matches(true, false, false, false, None));
    assert!(!empty_hotkey.matches(false, false, false, false, Some('A')));
    // But accurately matches the unconfigured idle state
    assert!(empty_hotkey.matches(false, false, false, false, None));

    // Hotkey with modifier and character should not match without modifier
    let alt_m = HotkeyConfig::alt_m();
    assert!(!alt_m.matches(false, false, false, false, Some('M')));
    // Hotkey with modifier should not match wrong character
    assert!(!alt_m.matches(false, false, true, false, Some('X')));
    // Hotkey should not match when extra modifier is pressed
    assert!(!alt_m.matches(true, false, true, false, Some('M')));
}

#[test]
fn test_exception_i18n_hotkey_unconfigured() {
    use turtkey::engine::types::HotkeyConfig;
    use turtkey::i18n::UiLanguage;

    let empty = HotkeyConfig::new(false, false, false, false, None);
    assert_eq!(empty.display_name_with_lang(UiLanguage::Vietnamese), "Chưa đặt");
    assert_eq!(empty.display_name_with_lang(UiLanguage::English), "Not set");
}
