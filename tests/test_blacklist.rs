//
// TurtKey - Excluded Applications (Blacklist) Test Suite
// Rigorous verification covering all 4 mandatory scenarios:
// 1. Happy Case (Standard add, match, auto-switching, state restoration)
// 2. Exception Case (Empty, whitespace, duplicate, invalid indices)
// 3. Hacking Case (Path traversal, null byte injection, fuzzing, buffer overflow)
// 4. 403 Case (Disabled blacklist isolation, individual rule disablement, config isolation)
// Clean, professional architecture (Zero emojis)
//

use turtkey::engine::blacklist::BlacklistManager;

// ============================================================================
// 1. HAPPY CASE: Standard Success Flow
// ============================================================================
#[test]
fn test_happy_blacklist_add_and_match() {
    let mut mgr = BlacklistManager::new();

    // Add games and apps
    assert!(mgr.add_app("minecraft.exe", None).is_ok());
    assert!(mgr.add_app("steam", Some("/usr/bin/steam".to_string())).is_ok());

    // Case-insensitive matching
    assert!(mgr.is_blacklisted("minecraft.exe"));
    assert!(mgr.is_blacklisted("MINECRAFT.EXE"));
    assert!(mgr.is_blacklisted("minecraft"));
    assert!(mgr.is_blacklisted("steam"));
    assert!(mgr.is_blacklisted("/usr/bin/steam"));

    // Normal apps should not be blacklisted
    assert!(!mgr.is_blacklisted("discord.exe"));
    assert!(!mgr.is_blacklisted("chrome.exe"));
    assert!(!mgr.is_blacklisted("word.exe"));
}

#[test]
fn test_happy_auto_switching_and_restoration() {
    let mut mgr = BlacklistManager::new();
    let mut vietnamese_enabled = true;

    // Initially in normal app (Discord): Vietnamese is ON
    let changed = mgr.update_active_app(Some("discord.exe"), &mut vietnamese_enabled);
    assert!(!changed);
    assert!(vietnamese_enabled, "Vietnamese should remain ON in normal app");

    // Alt-Tab to Game (cs2.exe): TurtKey automatically turns OFF Vietnamese
    let changed = mgr.update_active_app(Some("cs2.exe"), &mut vietnamese_enabled);
    assert!(changed, "Mode should change when entering blacklisted app");
    assert!(!vietnamese_enabled, "Vietnamese must be automatically disabled for game");

    // While inside the game: stays OFF
    let changed = mgr.update_active_app(Some("cs2.exe"), &mut vietnamese_enabled);
    assert!(!changed);
    assert!(!vietnamese_enabled);

    // Alt-Tab back to Discord: TurtKey automatically RESTORES Vietnamese!
    let changed = mgr.update_active_app(Some("discord.exe"), &mut vietnamese_enabled);
    assert!(changed, "Mode should change when returning to normal app");
    assert!(vietnamese_enabled, "Vietnamese must be automatically restored to previous ON state");
}

#[test]
fn test_happy_toggle_and_remove_rules() {
    let mut mgr = BlacklistManager::new();
    let count = mgr.apps.len();

    // Toggle rule
    assert!(mgr.toggle_app(0));
    assert!(!mgr.apps[0].enabled);
    assert!(mgr.toggle_app(0));
    assert!(mgr.apps[0].enabled);

    // Remove rule
    assert!(mgr.remove_app(0));
    assert_eq!(mgr.apps.len(), count - 1);
}

// ============================================================================
// 2. EXCEPTION CASE: Faulty Inputs & Error Handling
// ============================================================================
#[test]
fn test_exception_empty_and_whitespace_app_names() {
    let mut mgr = BlacklistManager::new();

    assert!(mgr.add_app("", None).is_err());
    assert!(mgr.add_app("   ", None).is_err());
    assert!(mgr.add_app("\t\n", None).is_err());
}

#[test]
fn test_exception_duplicate_app_names() {
    let mut mgr = BlacklistManager::new();

    assert!(mgr.add_app("mygame.exe", None).is_ok());
    // Duplicate case-insensitive
    let res = mgr.add_app("MYGAME.EXE", None);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("đã tồn tại"));
}

#[test]
fn test_exception_invalid_indices_and_paths() {
    let mut mgr = BlacklistManager::new();

    // Out of bound remove/toggle
    assert!(!mgr.remove_app(999));
    assert!(!mgr.toggle_app(999));

    // Invalid path names like . or ..
    assert!(BlacklistManager::clean_process_name(".").is_err());
    assert!(BlacklistManager::clean_process_name("..").is_err());
}

// ============================================================================
// 3. HACKING CASE: Exploitation & Vulnerability Resistance
// ============================================================================
#[test]
fn test_hacking_path_traversal_sanitization() {
    let mut mgr = BlacklistManager::new();

    // Attempt path traversal attacks to escape or corrupt system
    let res = mgr.add_app("../../../../etc/passwd", None);
    assert!(res.is_ok(), "Must extract basename safely");
    assert!(mgr.is_blacklisted("passwd"));

    let res2 = mgr.add_app(r"..\..\..\Windows\System32\cmd.exe", None);
    assert!(res2.is_ok());
    assert!(mgr.is_blacklisted("cmd.exe"));
}

#[test]
fn test_hacking_null_byte_and_injection_payloads() {
    let mut mgr = BlacklistManager::new();

    // Null-byte injection
    assert!(mgr.add_app("game.exe\0malicious", None).is_err());

    // XSS / SQLi payload as process name
    let xss = "<script>alert('pwned')</script>";
    assert!(mgr.add_app(xss, None).is_ok());
    assert!(mgr.is_blacklisted(xss));

    let sqli = "' OR '1'='1";
    assert!(mgr.add_app(sqli, None).is_ok());
    assert!(mgr.is_blacklisted(sqli));
}

#[test]
fn test_hacking_massive_buffer_fuzzing() {
    let mut mgr = BlacklistManager::new();

    // 10,000 characters process name fuzzing
    let large_name = "A".repeat(10_000);
    assert!(mgr.add_app(&large_name, None).is_ok());
    assert!(mgr.is_blacklisted(&large_name));
    assert!(!mgr.is_blacklisted("normal_app.exe"));
}

// ============================================================================
// 4. 403 CASE: Authorization & Boundary Isolation
// ============================================================================
#[test]
fn test_403_disabled_blacklist_blocks_auto_switching() {
    let mut mgr = BlacklistManager::new();
    mgr.enabled = false; // Globally disabled!

    let mut vietnamese_enabled = true;

    // Even if active process is cs2.exe, disabled blacklist MUST NOT change IME mode
    let changed = mgr.update_active_app(Some("cs2.exe"), &mut vietnamese_enabled);
    assert!(!changed, "Disabled blacklist must not affect engine state");
    assert!(vietnamese_enabled, "Vietnamese mode must remain intact");
    assert!(!mgr.is_blacklisted("cs2.exe"));
}

#[test]
fn test_403_disabled_individual_rule_isolation() {
    let mut mgr = BlacklistManager::new();

    // Find cs2.exe rule and disable it
    if let Some(pos) = mgr.apps.iter().position(|a| a.name == "cs2.exe") {
        mgr.apps[pos].enabled = false;
    }

    assert!(!mgr.is_blacklisted("cs2.exe"), "Disabled rule must be blocked from matching");

    // Other enabled rules must still match
    assert!(mgr.is_blacklisted("valorant.exe"));
}

#[test]
fn test_403_preserves_user_explicit_off_preference() {
    let mut mgr = BlacklistManager::new();
    let mut vietnamese_enabled = false; // User explicitly turned OFF Vietnamese in Discord!

    // Switch to game
    mgr.update_active_app(Some("cs2.exe"), &mut vietnamese_enabled);
    assert!(!vietnamese_enabled);

    // Switch back to Discord: since user previously had it OFF, it should NOT be forced ON!
    mgr.update_active_app(Some("discord.exe"), &mut vietnamese_enabled);
    assert!(!vietnamese_enabled, "Must not force ON if user had explicitly turned it OFF");
}
