// Integration tests for buffalo-core typing engine

use buffalo_core::{ESTD_FLAGS, Engine, VIETNAMESE_MODE, get_input_method};

fn type_word(engine: &mut Engine, keystrokes: &str) -> String {
    engine.reset();
    for c in keystrokes.chars() {
        engine.process_key(c, VIETNAMESE_MODE);
    }
    engine.get_processed_string(VIETNAMESE_MODE)
}

#[test]
fn test_telex_basic() {
    let telex = get_input_method("Telex").expect("Failed to load Telex layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    assert_eq!(type_word(&mut engine, "dd"), "đ");
    assert_eq!(type_word(&mut engine, "aa"), "â");
    assert_eq!(type_word(&mut engine, "aw"), "ă");
    assert_eq!(type_word(&mut engine, "w"), "ư");
    assert_eq!(type_word(&mut engine, "oo"), "ô");
    assert_eq!(type_word(&mut engine, "ow"), "ơ");
    assert_eq!(type_word(&mut engine, "["), "ơ");
    assert_eq!(type_word(&mut engine, "[["), "[");
    assert_eq!(type_word(&mut engine, "]"), "ư");
    assert_eq!(type_word(&mut engine, "]]"), "]");
    assert_eq!(type_word(&mut engine, "{"), "Ơ");
    assert_eq!(type_word(&mut engine, "{{"), "{");
    assert_eq!(type_word(&mut engine, "}"), "Ư");
    assert_eq!(type_word(&mut engine, "}}"), "}");
}

#[test]
fn test_telex_uppercase() {
    let telex = get_input_method("Telex").expect("Failed to load Telex layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    assert_eq!(type_word(&mut engine, "DD"), "Đ");
    assert_eq!(type_word(&mut engine, "AA"), "Â");
    assert_eq!(type_word(&mut engine, "AW"), "Ă");
    assert_eq!(type_word(&mut engine, "W"), "Ư");
    assert_eq!(type_word(&mut engine, "OO"), "Ô");
    assert_eq!(type_word(&mut engine, "OW"), "Ơ");
}

#[test]
fn test_telex_words() {
    let telex = get_input_method("Telex").expect("Failed to load Telex layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    assert_eq!(type_word(&mut engine, "hoangs"), "hoáng");
    assert_eq!(type_word(&mut engine, "tuyeejt"), "tuyệt");
    assert_eq!(type_word(&mut engine, "chuyeenr"), "chuyển");
    assert_eq!(type_word(&mut engine, "dduongwf"), "đường");
    assert_eq!(type_word(&mut engine, "vieetj"), "việt");
}

#[test]
fn test_telex_free_tone() {
    let telex = get_input_method("Telex").expect("Failed to load Telex layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    // Free tone mark placement (typing accent later or in the middle)
    assert_eq!(type_word(&mut engine, "hoasng"), "hoáng");
    assert_eq!(type_word(&mut engine, "truyeenj"), "truyện");
}

#[test]
fn test_telex_undo() {
    let telex = get_input_method("Telex").expect("Failed to load Telex layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    // Typing dd once yields đ, typing d again undoes to dd
    assert_eq!(type_word(&mut engine, "ddd"), "dd");
    assert_eq!(type_word(&mut engine, "aas"), "ấ");
}

#[test]
fn test_simple_telex_basic() {
    let simple_telex =
        get_input_method("Simple Telex").expect("Failed to load Simple Telex layout");
    let mut engine = Engine::new(simple_telex, ESTD_FLAGS);

    // Basic transformations (same as Telex)
    assert_eq!(type_word(&mut engine, "dd"), "đ");
    assert_eq!(type_word(&mut engine, "aa"), "â");
    assert_eq!(type_word(&mut engine, "aw"), "ă");
    assert_eq!(type_word(&mut engine, "oo"), "ô");
    assert_eq!(type_word(&mut engine, "ow"), "ơ");
    assert_eq!(type_word(&mut engine, "uw"), "ư");

    // Key 'w' alone must NOT turn into 'ư' (unlike standard Telex)
    assert_eq!(type_word(&mut engine, "w"), "w");
    assert_eq!(type_word(&mut engine, "W"), "W");

    // Brackets '[' and ']' must NOT be recognized as input method keys (unlike standard Telex)
    assert!(!engine.can_process_key('['));
    assert!(!engine.can_process_key(']'));
}

// ===== Encoder Tests =====

use buffalo_core::{CharsetEncoding, encode};

#[test]
fn test_encode_unicode_identity() {
    let input = "Tiếng Việt đẹp lắm!";
    let result = encode(&CharsetEncoding::Unicode, input);
    assert_eq!(result, input);
}

#[test]
fn test_encode_unicode_ascii_passthrough() {
    let input = "Hello World 123 !@#";
    let result = encode(&CharsetEncoding::Unicode, input);
    assert_eq!(result, input);
}

#[test]
fn test_encode_tcvn3_basic() {
    // Test individual character mappings
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "đ"), "\u{00AE}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "â"), "\u{00A9}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "ă"), "\u{00A8}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "ê"), "\u{00AA}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "ô"), "\u{00AB}");
}

#[test]
fn test_encode_tcvn3_tones() {
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "á"), "\u{00B8}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "à"), "\u{00B5}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "ả"), "\u{00B6}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "ã"), "\u{00B7}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "ạ"), "\u{00B9}");
}

#[test]
fn test_encode_tcvn3_uppercase() {
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "Đ"), "\u{00A7}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "Â"), "\u{00A2}");
    assert_eq!(encode(&CharsetEncoding::Tcvn3, "Ă"), "\u{00A1}");
}

#[test]
fn test_encode_vni_basic() {
    assert_eq!(encode(&CharsetEncoding::VniWindows, "đ"), "\u{00F1}");
    assert_eq!(encode(&CharsetEncoding::VniWindows, "â"), "a\u{00E2}");
    assert_eq!(encode(&CharsetEncoding::VniWindows, "ă"), "a\u{00EA}");
    assert_eq!(encode(&CharsetEncoding::VniWindows, "ê"), "e\u{00E2}");
    assert_eq!(encode(&CharsetEncoding::VniWindows, "ô"), "o\u{00E2}");
}

#[test]
fn test_encode_vni_uppercase() {
    assert_eq!(encode(&CharsetEncoding::VniWindows, "Đ"), "\u{00D1}");
    assert_eq!(encode(&CharsetEncoding::VniWindows, "Â"), "A\u{00C2}");
}

#[test]
fn test_encode_viqr_basic() {
    assert_eq!(encode(&CharsetEncoding::Viqr, "đ"), "dd");
    assert_eq!(encode(&CharsetEncoding::Viqr, "â"), "a^");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ă"), "a(");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ê"), "e^");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ô"), "o^");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ơ"), "o+");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ư"), "u+");
}

#[test]
fn test_encode_viqr_tones() {
    assert_eq!(encode(&CharsetEncoding::Viqr, "á"), "a'");
    assert_eq!(encode(&CharsetEncoding::Viqr, "à"), "a`");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ả"), "a?");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ã"), "a~");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ạ"), "a.");
}

#[test]
fn test_encode_viqr_compound() {
    assert_eq!(encode(&CharsetEncoding::Viqr, "ấ"), "a^'");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ầ"), "a^`");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ế"), "e^'");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ớ"), "o+'");
    assert_eq!(encode(&CharsetEncoding::Viqr, "ứ"), "u+'");
}

#[test]
fn test_encode_viqr_uppercase() {
    assert_eq!(encode(&CharsetEncoding::Viqr, "Đ"), "DD");
    assert_eq!(encode(&CharsetEncoding::Viqr, "Â"), "A^");
    assert_eq!(encode(&CharsetEncoding::Viqr, "Ơ"), "O+");
    assert_eq!(encode(&CharsetEncoding::Viqr, "Ấ"), "A^'");
}

#[test]
fn test_encode_ascii_passthrough() {
    // Non-Vietnamese characters should pass through unchanged
    let input = "Hello World 123";
    assert_eq!(encode(&CharsetEncoding::Tcvn3, input), input);
    assert_eq!(encode(&CharsetEncoding::VniWindows, input), input);
    assert_eq!(encode(&CharsetEncoding::Viqr, input), input);
}

#[test]
fn test_encode_mixed_text() {
    // Mix of Vietnamese and ASCII characters
    let viqr_result = encode(&CharsetEncoding::Viqr, "Xin chào Việt Nam!");
    assert_eq!(viqr_result, "Xin cha`o Vie^.t Nam!");
}

#[test]
fn test_charset_encoding_display() {
    assert_eq!(CharsetEncoding::Unicode.to_string(), "Unicode");
    assert_eq!(CharsetEncoding::Tcvn3.to_string(), "TCVN3");
    assert_eq!(CharsetEncoding::VniWindows.to_string(), "VNI");
    assert_eq!(CharsetEncoding::Viqr.to_string(), "VIQR");
}

#[test]
fn test_charset_encoding_from_str() {
    use std::str::FromStr;
    assert_eq!(
        CharsetEncoding::from_str("Unicode").unwrap(),
        CharsetEncoding::Unicode
    );
    assert_eq!(
        CharsetEncoding::from_str("TCVN3").unwrap(),
        CharsetEncoding::Tcvn3
    );
    assert_eq!(
        CharsetEncoding::from_str("VNI").unwrap(),
        CharsetEncoding::VniWindows
    );
    assert_eq!(
        CharsetEncoding::from_str("VIQR").unwrap(),
        CharsetEncoding::Viqr
    );
    assert!(CharsetEncoding::from_str("Unknown").is_err());
}

#[test]
fn test_charset_display_name() {
    assert_eq!(CharsetEncoding::Unicode.display_name(), "Unicode");
    assert_eq!(CharsetEncoding::Tcvn3.display_name(), "TCVN3 (ABC)");
    assert_eq!(CharsetEncoding::VniWindows.display_name(), "VNI Windows");
    assert_eq!(CharsetEncoding::Viqr.display_name(), "VIQR");
}

#[test]
fn test_charset_all() {
    let all = CharsetEncoding::all();
    assert_eq!(all.len(), 4);
    assert_eq!(all[0], CharsetEncoding::Unicode);
    assert_eq!(all[1], CharsetEncoding::Tcvn3);
    assert_eq!(all[2], CharsetEncoding::VniWindows);
    assert_eq!(all[3], CharsetEncoding::Viqr);
}
