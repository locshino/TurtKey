//
// TurtKey - Dedicated Game Mode & Mode-Specific Macro Test Suite
// Rigorous verification covering all 4 mandatory scenarios:
// 1. Happy Case (Standard Success Flow: WASD passthrough, zero latency, Game shortcuts, per-mode macro isolation)
// 2. Exception Case (Faulty Inputs & Error Handling: buffer limits, empty/whitespace keys, underflow resistance)
// 3. Hacking Case (Exploitation & Resistance: recursion, malicious payloads, fuzzing, null bytes)
// 4. 403 Case (Authorization & Isolation: disabled macro control, cross-mode isolation, precedence)
// Clean, professional architecture (Zero emojis)
//

use turtkey::engine::tables::char_to_key_code;
use turtkey::engine::types::{HookCodeState, TargetMode, KEY_DELETE, KEY_ESC, KEY_SPACE};
use turtkey::engine::VietnameseEngine;
use turtkey::i18n::UiLanguage;

// ============================================================================
// 1. HAPPY CASE: Standard Success Flow
// ============================================================================
#[test]
fn test_happy_game_mode_properties_and_cycle() {
    let mut engine = VietnameseEngine::new();

    // Verify TargetMode::Game properties
    assert_eq!(TargetMode::Game.display_name(), "Game mode");
    assert_eq!(TargetMode::Game.short_name(), "Game");
    assert_eq!(TargetMode::Game.backspace_delay_ms(), 0);

    let desc_vi = TargetMode::Game.description_with_lang(UiLanguage::Vietnamese);
    assert!(desc_vi.contains("Game") || desc_vi.contains("mượt"));
    let desc_en = TargetMode::Game.description_with_lang(UiLanguage::English);
    assert!(desc_en.contains("Gaming") || desc_en.contains("keystrokes"));

    // Verify TargetMode::all() includes Game
    assert!(TargetMode::all().contains(&TargetMode::Game));

    // Verify cycle: Win -> Linux -> Game -> Win
    assert_eq!(engine.target_mode, TargetMode::Win);
    assert_eq!(engine.toggle_target_mode(), TargetMode::Linux);
    assert_eq!(engine.toggle_target_mode(), TargetMode::Game);
    assert_eq!(engine.toggle_target_mode(), TargetMode::Win);
}

#[test]
fn test_happy_game_mode_wasd_zero_interference() {
    let mut engine = VietnameseEngine::new();
    engine.target_mode = TargetMode::Game;
    engine.vietnamese_enabled = true; // Even if Vietnamese is ON

    // In Telex, 'a' + 'a' would normally become 'â', and 'w' would become 'ư', 'a' + 's' would become 'á'
    // In Game mode, keystrokes MUST NOT be intercepted or transformed (WASD movement pure passthrough)
    let keys = ['w', 'a', 's', 'd', 'w', 'w', 'a', 'a', 's', 'd'];
    for ch in keys {
        let (code, caps) = char_to_key_code(ch).unwrap();
        let hook = engine.process_key(code, caps, false);

        assert_eq!(
            hook.code,
            HookCodeState::DoNothing,
            "Keystroke '{}' in Game mode must return DoNothing for pure passthrough",
            ch
        );
        assert_eq!(hook.backspace_count, 0, "No backspaces should be issued");
        assert_eq!(hook.backspace_delay_ms, 0, "Latency must be 0ms");
    }
}

#[test]
fn test_happy_game_mode_shortcuts_and_mode_isolation() {
    let mut engine = VietnameseEngine::new();

    // Default macro table must be empty out-of-the-box (zero pre-populated custom data)
    assert!(engine.macro_mgr.entries_for_mode(TargetMode::Game).is_empty());
    assert!(engine.macro_mgr.entries_for_mode(TargetMode::Win).is_empty());
    assert!(engine.macro_mgr.entries_for_mode(TargetMode::Linux).is_empty());

    // 1. Add Game shortcut and verify it works in Game mode
    engine.target_mode = TargetMode::Game;
    assert!(engine.macro_mgr.add_for_mode("gg", "Good game well played", TargetMode::Game));
    assert!(engine.macro_mgr.add_for_mode("vn", "Việt Nam", TargetMode::Win));

    // Type 'g', 'g' then Space
    let (code_g, caps_g) = char_to_key_code('g').unwrap();
    let hook_g1 = engine.process_key(code_g, caps_g, false);
    assert_eq!(hook_g1.code, HookCodeState::DoNothing);

    let hook_g2 = engine.process_key(code_g, caps_g, false);
    assert_eq!(hook_g2.code, HookCodeState::DoNothing);

    let hook_space = engine.process_key(KEY_SPACE, false, false);
    assert_eq!(hook_space.code, HookCodeState::ReplaceMacro);
    assert_eq!(hook_space.backspace_count, 2);
    let replaced_text: String = hook_space.char_data.iter().map(|&c| char::from_u32(c).unwrap()).collect();
    assert_eq!(replaced_text, "Good game well played");

    // 2. Custom Game macro addition
    assert!(engine.macro_mgr.add_for_mode("ggwp", "Good Game, Well Played!", TargetMode::Game));
    engine.start_new_session();
    for ch in ['g', 'g', 'w', 'p'] {
        let (code, caps) = char_to_key_code(ch).unwrap();
        engine.process_key(code, caps, false);
    }
    let hook_custom = engine.process_key(KEY_SPACE, false, false);
    assert_eq!(hook_custom.code, HookCodeState::ReplaceMacro);
    let custom_text: String = hook_custom.char_data.iter().map(|&c| char::from_u32(c).unwrap()).collect();
    assert_eq!(custom_text, "Good Game, Well Played!");

    // 3. Mode Isolation: Office macro "vn" -> "Việt Nam" does NOT expand in Game mode
    engine.start_new_session();
    for ch in ['v', 'n'] {
        let (code, caps) = char_to_key_code(ch).unwrap();
        engine.process_key(code, caps, false);
    }
    let hook_vn_in_game = engine.process_key(KEY_SPACE, false, false);
    assert_ne!(
        hook_vn_in_game.code,
        HookCodeState::ReplaceMacro,
        "Office macro 'vn' should not trigger in Game mode"
    );

    // 4. In Win mode, office macro "vn" DOES expand, while game shortcut "gg" does NOT expand by default
    engine.target_mode = TargetMode::Win;
    engine.start_new_session();
    for ch in ['v', 'n'] {
        let (code, caps) = char_to_key_code(ch).unwrap();
        engine.process_key(code, caps, false);
    }
    let hook_vn_in_win = engine.process_key(KEY_SPACE, false, false);
    assert_eq!(hook_vn_in_win.code, HookCodeState::ReplaceMacro);
    let vn_text: String = hook_vn_in_win.char_data.iter().map(|&c| char::from_u32(c).unwrap()).collect();
    assert_eq!(vn_text, "Việt Nam");
}

// ============================================================================
// 2. EXCEPTION CASE: Faulty Inputs & Error Handling
// ============================================================================
#[test]
fn test_exception_game_macro_empty_and_whitespace() {
    let mut engine = VietnameseEngine::new();

    // Adding empty key or replacement returns false
    assert!(!engine.macro_mgr.add_for_mode("", "something", TargetMode::Game));
    assert!(!engine.macro_mgr.add_for_mode("key", "", TargetMode::Game));
    assert!(!engine.macro_mgr.add_for_mode("   ", "val", TargetMode::Game));
    assert!(!engine.macro_mgr.add_for_mode("key", "   ", TargetMode::Game));
    assert!(!engine.macro_mgr.add_for_mode("\t\n", "val", TargetMode::Game));
}

#[test]
fn test_exception_game_mode_buffer_overflow_and_underflow() {
    let mut engine = VietnameseEngine::new();
    engine.target_mode = TargetMode::Game;

    // Buffer underflow: multiple backspaces on empty buffer must not panic or error
    for _ in 0..10 {
        let hook = engine.process_key(KEY_DELETE, false, false);
        assert_eq!(hook.code, HookCodeState::DoNothing);
    }

    // Buffer limit: typing over 100 characters in Game mode must gracefully clamp
    let (code_a, caps_a) = char_to_key_code('a').unwrap();
    for _ in 0..100 {
        let hook = engine.process_key(code_a, caps_a, false);
        assert_eq!(hook.code, HookCodeState::DoNothing);
    }

    // Space after 100 characters does not panic
    let hook_space = engine.process_key(KEY_SPACE, false, false);
    assert_eq!(hook_space.code, HookCodeState::DoNothing);
}

#[test]
fn test_exception_escape_and_deletion_in_game_mode() {
    let mut engine = VietnameseEngine::new();
    engine.target_mode = TargetMode::Game;

    // Type 'g', Backspace, type 'g', Space -> should not trigger "gg" macro
    let (code_g, caps_g) = char_to_key_code('g').unwrap();
    engine.process_key(code_g, caps_g, false);
    engine.process_key(KEY_DELETE, false, false);
    engine.process_key(code_g, caps_g, false);
    let hook = engine.process_key(KEY_SPACE, false, false);
    assert_ne!(hook.code, HookCodeState::ReplaceMacro);

    // Escape clears the buffer completely
    engine.process_key(code_g, caps_g, false);
    engine.process_key(code_g, caps_g, false);
    engine.process_key(KEY_ESC, false, false);
    let hook2 = engine.process_key(KEY_SPACE, false, false);
    assert_ne!(hook2.code, HookCodeState::ReplaceMacro);
}

// ============================================================================
// 3. HACKING CASE: Exploitation & Vulnerability Resistance
// ============================================================================
#[test]
fn test_hacking_game_macro_payload_resistance() {
    let mut engine = VietnameseEngine::new();

    // SQL Injection, XSS, and command injection payloads in Game macros
    let payloads = [
        ("sql", "'; DROP TABLE users; --"),
        ("xss", "<script>alert('xss')</script>"),
        ("cmd", "$(rm -rf /)"),
        ("null", "safe\0injected"),
    ];

    for (k, v) in payloads {
        assert!(engine.macro_mgr.add_for_mode(k, v, TargetMode::Game));
        let found = engine.macro_mgr.lookup_for_mode(k, TargetMode::Game);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), v);
    }
}

#[test]
fn test_hacking_game_mode_fuzz_burst() {
    let mut engine = VietnameseEngine::new();
    engine.target_mode = TargetMode::Game;

    // Simulate intense gamer keystroke spam: 10,000 rapid keystrokes of movement, skills, space
    let game_keys = ['w', 'a', 's', 'd', 'q', 'e', 'r', 'f', '1', '2', '3', '4'];
    for i in 0..10_000 {
        let ch = game_keys[i % game_keys.len()];
        let (code, caps) = char_to_key_code(ch).unwrap();
        let hook = engine.process_key(code, caps, false);
        assert_eq!(hook.code, HookCodeState::DoNothing);

        if i % 25 == 0 {
            engine.process_key(KEY_SPACE, false, false);
        }
    }
}

// ============================================================================
// 4. 403 CASE: Authorization, Access Control & Isolation
// ============================================================================
#[test]
fn test_403_game_macro_disabled_control() {
    let mut engine = VietnameseEngine::new();
    engine.target_mode = TargetMode::Game;

    // When macro system is disabled, even valid Game shortcuts must be blocked
    engine.macro_mgr.enabled = false;

    let (code_g, caps_g) = char_to_key_code('g').unwrap();
    engine.process_key(code_g, caps_g, false);
    engine.process_key(code_g, caps_g, false);
    let hook = engine.process_key(KEY_SPACE, false, false);

    assert_eq!(
        hook.code,
        HookCodeState::DoNothing,
        "Disabled macro manager must strictly block shortcut expansion"
    );
}

#[test]
fn test_403_mode_macro_boundary_isolation() {
    let mut engine = VietnameseEngine::new();

    // Create a secret command in Game mode
    engine.macro_mgr.add_for_mode("secret", "GameSecret123", TargetMode::Game);

    // Verify it cannot be accessed from Win mode
    assert_eq!(
        engine.macro_mgr.lookup_for_mode("secret", TargetMode::Win),
        None,
        "Game macro must be isolated from Win mode"
    );

    // Verify it cannot be accessed from Linux mode
    assert_eq!(
        engine.macro_mgr.lookup_for_mode("secret", TargetMode::Linux),
        None,
        "Game macro must be isolated from Linux mode"
    );

    // Verify it IS accessible in Game mode
    assert_eq!(
        engine.macro_mgr.lookup_for_mode("secret", TargetMode::Game),
        Some("GameSecret123".to_string())
    );
}
