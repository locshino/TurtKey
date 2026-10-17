//
// TurtKey - Dedicated Plugin Architecture Test Suite
// Covers both UI i18n Language Plugins and Input Method Engine Plugins
// Mandatory 4-Scenario Rule: Happy, Exception, Hacking, 403
// Clean, professional, zero emojis
//

use turtkey::engine::types::TargetMode;
use turtkey::plugins::i18n::{I18nManager, LanguagePack};
use turtkey::plugins::input_engine::EnginePluginManager;

// ============================================================================
// 1. HAPPY CASES (Standard / Success Flow)
// ============================================================================

#[test]
fn test_happy_ui_i18n_plugin_system_all_builtin_languages() {
    let mut mgr = I18nManager::new();

    // Verify default built-in language packs: only Vietnamese and English for simplicity
    assert_eq!(mgr.pack_count(), 2, "Default should only have 2 built-in packs (vi, en)");
    let available = mgr.available_languages();
    let ids: Vec<&str> = available.iter().map(|(id, _)| id.as_str()).collect();
    assert!(ids.contains(&"vi"), "Must include Vietnamese");
    assert!(ids.contains(&"en"), "Must include English");

    // Test Vietnamese
    assert!(mgr.set_language("vi"));
    assert_eq!(mgr.current_language_id(), "vi");
    assert_eq!(mgr.t("tab_general"), "Bộ gõ");
    assert_eq!(mgr.t("app_title"), "TurtKey");

    // Test English
    assert!(mgr.set_language("en"));
    assert_eq!(mgr.current_language_id(), "en");
    assert_eq!(mgr.t("tab_general"), "Engine");

    // Test extended built-in packs via with_all_builtins()
    let mut all_mgr = I18nManager::with_all_builtins();
    assert!(all_mgr.pack_count() >= 6);
    assert!(all_mgr.set_language("fr"));
    assert_eq!(all_mgr.t("tab_general"), "Moteur");
    assert!(all_mgr.set_language("ja"));
    assert_eq!(all_mgr.t("tab_general"), "一般設定");
    assert!(all_mgr.set_language("de"));
    assert_eq!(all_mgr.t("tab_general"), "Eingabe");
    assert!(all_mgr.set_language("es"));
    assert_eq!(all_mgr.t("tab_general"), "Motor");
}

#[test]
fn test_happy_custom_json_language_plugin_loading() {
    let mut mgr = I18nManager::new();

    let italian_json = r#"{
        "meta": {
            "id": "it",
            "name": "Italiano",
            "author": "Community Italian",
            "version": "1.0.0"
        },
        "strings": {
            "app_title": "TurtKey",
            "tab_general": "Configurazione",
            "tab_macro": "Macro",
            "tab_convert": "Convertitore"
        }
    }"#;

    let loaded_id = mgr.load_pack_from_json(italian_json);
    assert_eq!(loaded_id.unwrap(), "it");
    assert!(mgr.set_language("it"));
    assert_eq!(mgr.current_language_name(), "Italiano");
    assert_eq!(mgr.t("tab_general"), "Configurazione");
    assert_eq!(mgr.t("tab_convert"), "Convertitore");
}

#[test]
fn test_happy_input_engine_plugin_multilingual_typing() {
    // Verify default engine plugins: only Vietnamese and English (Passthrough) for simplicity
    let default_mgr = EnginePluginManager::new();
    assert_eq!(default_mgr.plugin_count(), 2, "Default should only have 2 engine plugins (vi, en)");
    let engine_ids: Vec<String> = default_mgr.available_engines().into_iter().map(|(id, _, _)| id).collect();
    assert!(engine_ids.contains(&"vietnamese".to_string()));
    assert!(engine_ids.contains(&"english".to_string()));

    // Verify extended engine plugins via with_all_builtins()
    let mut mgr = EnginePluginManager::with_all_builtins();
    assert_eq!(mgr.plugin_count(), 5);

    // 1. Vietnamese Plugin
    assert!(mgr.set_active("vietnamese"));
    let vi_res = mgr.type_string("as ");
    assert_eq!(vi_res, "á ");

    // 2. French Plugin
    assert!(mgr.set_active("french"));
    let fr_res1 = mgr.type_string("e' ");
    assert_eq!(fr_res1, "é ");
    let fr_res2 = mgr.type_string("c, ");
    assert_eq!(fr_res2, "ç ");
    let fr_res3 = mgr.type_string("e^ ");
    assert_eq!(fr_res3, "ê ");
    let fr_res4 = mgr.type_string("oe ");
    assert_eq!(fr_res4, "œ ");

    // 3. German Plugin
    assert!(mgr.set_active("german"));
    let de_res1 = mgr.type_string("ae ");
    assert_eq!(de_res1, "ä ");
    let de_res2 = mgr.type_string("oe ");
    assert_eq!(de_res2, "ö ");
    let de_res3 = mgr.type_string("ue ");
    assert_eq!(de_res3, "ü ");
    let de_res4 = mgr.type_string("ss ");
    assert_eq!(de_res4, "ß ");
    let de_res5 = mgr.type_string("Ae ");
    assert_eq!(de_res5, "Ä ");

    // 4. Spanish Plugin
    assert!(mgr.set_active("spanish"));
    let es_res1 = mgr.type_string("n~ ");
    assert_eq!(es_res1, "ñ ");
    let es_res2 = mgr.type_string("a' ");
    assert_eq!(es_res2, "á ");
    let es_res3 = mgr.type_string("??");
    assert_eq!(es_res3, "¿");
    let es_res4 = mgr.type_string("!!");
    assert_eq!(es_res4, "¡");

    // 5. English Passthrough Plugin
    assert!(mgr.set_active("english"));
    let en_res = mgr.type_string("hello world");
    assert_eq!(en_res, "hello world");
}

#[test]
fn test_happy_target_mode_propagation_across_plugins() {
    let mut mgr = EnginePluginManager::with_all_builtins();

    // Set target mode to Linux (WSL mode)
    mgr.set_target_mode(TargetMode::Linux);
    assert_eq!(mgr.active_plugin().target_mode(), TargetMode::Linux);

    // Switch between all engines and verify they inherited Linux target mode
    mgr.set_active("french");
    assert_eq!(mgr.active_plugin().target_mode(), TargetMode::Linux);

    mgr.set_active("german");
    assert_eq!(mgr.active_plugin().target_mode(), TargetMode::Linux);

    mgr.set_active("spanish");
    assert_eq!(mgr.active_plugin().target_mode(), TargetMode::Linux);

    // Switch back to Win mode
    mgr.set_target_mode(TargetMode::Win);
    assert_eq!(mgr.active_plugin().target_mode(), TargetMode::Win);
}

// ============================================================================
// 2. EXCEPTION CASES (Faulty Inputs / Error Handling)
// ============================================================================

#[test]
fn test_exception_malformed_json_language_pack() {
    let mut mgr = I18nManager::new();

    let broken_jsons = [
        "",                                         // Empty string
        "{ not valid json }",                       // Syntax error
        r#"{"meta": {"id": "xx"}}"#,                // Missing required fields
        r#"{"meta": 12345, "strings": "bad"}"#,     // Type mismatch
        "null",                                     // Null root
    ];

    for broken in broken_jsons {
        let res = mgr.load_pack_from_json(broken);
        assert!(res.is_err(), "Must return Err for malformed JSON: {}", broken);
    }
}

#[test]
fn test_exception_missing_translation_keys_multi_tier_fallback() {
    let mut mgr = I18nManager::new();

    // Register a pack with only one translated key
    let sparse_pack = LanguagePack::new("sparse", "Sparse Lang", "Tester", "1.0")
        .with_string("tab_general", "Custom General");

    mgr.register_pack(sparse_pack);
    mgr.set_language("sparse");

    // 1. Tier 1: Present in sparse pack
    assert_eq!(mgr.t("tab_general"), "Custom General");

    // 2. Tier 2: Missing in sparse pack -> falls back to base pack (vi)
    assert_eq!(mgr.t("tab_macro"), "Gõ tắt");
    assert_eq!(mgr.t("tab_convert"), "Chuyển mã");

    // 3. Tier 3: Key doesn't exist anywhere in any pack -> returns key itself
    let non_existent_key = "completely_unknown_key_xyz_999";
    assert_eq!(mgr.t(non_existent_key), non_existent_key);
}

#[test]
fn test_exception_switching_to_nonexistent_language_or_engine() {
    let mut i18n = I18nManager::new();
    let current_id = i18n.current_language_id().to_string();

    // Non-existent UI language ID
    assert!(!i18n.set_language("klingon"));
    assert_eq!(i18n.current_language_id(), current_id);

    // Non-existent Input Engine ID
    let mut engines = EnginePluginManager::new();
    let active_id = engines.active_id().to_string();
    assert!(!engines.set_active("elvish_engine"));
    assert_eq!(engines.active_id(), active_id);
}

// ============================================================================
// 3. HACKING CASES (Exploitation & Vulnerability Resistance)
// ============================================================================

#[test]
fn test_hacking_poisoned_json_language_pack_injection() {
    let mut mgr = I18nManager::new();

    // Payload containing XSS, SQLi, Path Traversal, and oversized buffer strings
    let massive_string = "A".repeat(100_000);
    let poisoned_json = format!(
        r#"{{
            "meta": {{
                "id": "hack_lang",
                "name": "<script>alert('pwned')</script>",
                "author": "'; DROP TABLE users; --",
                "version": "../../../etc/shadow"
            }},
            "strings": {{
                "app_title": "<img src=x onerror=steal()>",
                "tab_general": "{}",
                "null_byte": "text\u0000with_null"
            }}
        }}"#,
        massive_string
    );

    let res = mgr.load_pack_from_json(&poisoned_json);
    assert!(res.is_ok(), "Must parse without crashing or executing payload");

    assert!(mgr.set_language("hack_lang"));
    // Payload is treated purely as inert text
    assert_eq!(mgr.t("app_title"), "<img src=x onerror=steal()>");
    assert_eq!(mgr.t("tab_general").len(), 100_000);
    assert!(mgr.t("null_byte").contains("text"));
}

#[test]
fn test_hacking_rapid_plugin_switching_mid_composition() {
    use turtkey::engine::tables::char_to_key_code;

    let mut mgr = EnginePluginManager::with_all_builtins();

    // Start typing in French ('e' waiting for accent)
    mgr.set_active("french");
    let (c1, k1) = char_to_key_code('e').unwrap();
    mgr.process_key(c1, k1, false);

    // Suddenly switch to German mid-composition
    mgr.set_active("german");
    // German buffer must be clean, typing 'e' shouldn't produce corrupted French accent
    let res = mgr.type_string("ae ");
    assert_eq!(res, "ä ");

    // Rapidly cycle all engines 100 times without leaking memory or crashing
    let engine_ids = ["vietnamese", "french", "german", "spanish", "english"];
    for i in 0..100 {
        let id = engine_ids[i % engine_ids.len()];
        assert!(mgr.set_active(id));
        mgr.reset();
    }
}

// ============================================================================
// 4. 403 CASES (Authorization & Boundary Enforcement)
// ============================================================================

#[test]
fn test_403_restricted_directory_plugin_loading_denial() {
    let mut mgr = I18nManager::new();

    // Attempt to load from non-existent / restricted path
    let restricted_paths = [
        "/root/protected_plugins_dir_xyz",
        "/proc/sys/kernel/plugins",
        "/non_existent_folder_abc_123/locales",
    ];

    for path in restricted_paths {
        let count = mgr.load_packs_from_dir(path).unwrap_or(0);
        assert_eq!(count, 0, "Must safely return 0 loaded packs for inaccessible dir");
    }
}

#[test]
fn test_403_plugin_boundary_and_control_modifiers_isolation() {
    let mut mgr = EnginePluginManager::with_all_builtins();

    for id in ["vietnamese", "french", "german", "spanish", "english"] {
        mgr.set_active(id);
        // Keystroke with Ctrl/Alt pressed (e.g. Ctrl+C, Alt+F4)
        let state = mgr.process_key('c' as u16, false, true);
        assert_eq!(
            state.backspace_count, 0,
            "Engine '{}' must not emit backspaces when modifier keys are pressed",
            id
        );
    }
}
