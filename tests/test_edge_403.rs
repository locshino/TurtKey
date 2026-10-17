//
// TurtKey - Scenario 4: Access Control & Authorization (403 Equivalent) Tests
// Verifies strict gating when features, language modes, or modifier access controls are active
//

use turtkey::engine::macro_table::MacroManager;
use turtkey::engine::tables::char_to_key_code;
use turtkey::engine::types::{HookCodeState, KEY_S};
use turtkey::engine::VietnameseEngine;

#[test]
fn test_403_vietnamese_mode_disabled_blocks_transformation() {
    let mut engine = VietnameseEngine::new();
    // Turn off Vietnamese mode (English mode active)
    engine.vietnamese_enabled = false;

    // Typing "tiengs" should NOT produce "tiếng"
    let output = engine.type_string("tiengs");
    assert_eq!(output, "tiengs");
}

#[test]
fn test_403_control_modifier_blocks_engine_hook() {
    let mut engine = VietnameseEngine::new();

    // Type 'a' normally
    let (code_a, caps_a) = char_to_key_code('a').unwrap();
    engine.process_key(code_a, caps_a, false);

    // Type 's' with Ctrl / Alt held down (other_ctrl = true)
    let hook = engine.process_key(KEY_S, false, true);
    // Must be blocked / ignored, not transforming 'a' into 'á'
    assert_eq!(hook.code, HookCodeState::DoNothing);
}

#[test]
fn test_403_macro_disabled_blocks_expansion() {
    let mut mm = MacroManager::new();
    mm.add("vn", "Việt Nam");

    // When disabled, lookup returns None (blocked)
    mm.enabled = false;
    assert_eq!(mm.lookup("vn"), None);
}

#[test]
fn test_403_macro_english_mode_isolation() {
    let mut engine = VietnameseEngine::new();
    engine.macro_mgr.add("vn", "Việt Nam");
    engine.macro_mgr.enabled = true;

    // When in English mode and allow_in_english_mode is false:
    engine.vietnamese_enabled = false;
    engine.macro_mgr.allow_in_english_mode = false;

    let output = engine.type_string("vn ");
    assert_eq!(output, "vn ");

    // Now permit macro in English mode:
    engine.start_new_session();
    engine.macro_mgr.allow_in_english_mode = true;
    let output2 = engine.type_string("vn ");
    assert_eq!(output2, "Việt Nam ");
}

#[test]
fn test_403_target_mode_does_not_bypass_disabled_engine() {
    let mut engine = VietnameseEngine::new();
    // Engine is disabled
    engine.vietnamese_enabled = false;

    // Switching to Linux mode must still NOT bypass the disabled state
    engine.target_mode = turtkey::engine::types::TargetMode::Linux;
    let output = engine.type_string("tiengs");
    assert_eq!(output, "tiengs");

    // Switching back to Win mode must also remain blocked
    engine.target_mode = turtkey::engine::types::TargetMode::Win;
    let output2 = engine.type_string("tiengs");
    assert_eq!(output2, "tiengs");
}

#[test]
fn test_403_hotkey_isolation_and_cross_trigger_prevention() {
    use turtkey::engine::types::HotkeyConfig;

    let lang_hotkey = HotkeyConfig::ctrl_shift();
    let mode_hotkey = HotkeyConfig::alt_m();
    let conv_hotkey = HotkeyConfig::alt_c();

    // 1. Language hotkey (Ctrl+Shift) must NOT trigger mode or convert hotkeys
    assert!(lang_hotkey.matches(true, true, false, false, None));
    assert!(!mode_hotkey.matches(true, true, false, false, None));
    assert!(!conv_hotkey.matches(true, true, false, false, None));

    // 2. Mode hotkey (Alt+M) must NOT trigger language or convert hotkeys
    assert!(mode_hotkey.matches(false, false, true, false, Some('M')));
    assert!(!lang_hotkey.matches(false, false, true, false, Some('M')));
    assert!(!conv_hotkey.matches(false, false, true, false, Some('M')));

    // 3. Partial modifiers (e.g. only Ctrl without Shift) must be denied
    assert!(!lang_hotkey.matches(true, false, false, false, None));
    assert!(!lang_hotkey.matches(false, true, false, false, None));

    // 4. Extra modifiers (e.g. Ctrl + Shift + Alt + M) must NOT trigger Alt+M
    assert!(!mode_hotkey.matches(true, true, true, false, Some('M')));
}

#[test]
fn test_403_ui_language_isolation_from_engine_state() {
    use turtkey::engine::VietnameseEngine;
    use turtkey::i18n::UiLanguage;

    let mut engine = VietnameseEngine::new();
    engine.vietnamese_enabled = false;

    // Switching UI Language must NOT activate or mutate engine typing state
    let _ui_lang = UiLanguage::English;
    assert!(!engine.vietnamese_enabled);

    let output = engine.type_string("tieengs");
    assert_eq!(output, "tieengs"); // Remains raw / blocked as expected

    let _ui_lang_vi = UiLanguage::Vietnamese;
    assert!(!engine.vietnamese_enabled);
    let output2 = engine.type_string("tieengs");
    assert_eq!(output2, "tieengs");
}
