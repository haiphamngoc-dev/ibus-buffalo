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
    let telex = get_input_method("Telex 2").expect("Failed to load Telex 2 layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    assert_eq!(type_word(&mut engine, "dd"), "đ");
    assert_eq!(type_word(&mut engine, "aa"), "â");
    assert_eq!(type_word(&mut engine, "aw"), "ă");
    assert_eq!(type_word(&mut engine, "w"), "ư");
    assert_eq!(type_word(&mut engine, "oo"), "ô");
    assert_eq!(type_word(&mut engine, "ow"), "ơ");
}

#[test]
fn test_telex_words() {
    let telex = get_input_method("Telex 2").expect("Failed to load Telex 2 layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    assert_eq!(type_word(&mut engine, "hoangs"), "hoáng");
    assert_eq!(type_word(&mut engine, "tuyeejt"), "tuyệt");
    assert_eq!(type_word(&mut engine, "chuyeenr"), "chuyển");
    assert_eq!(type_word(&mut engine, "dduongwf"), "đường");
    assert_eq!(type_word(&mut engine, "vieetj"), "việt");
}

#[test]
fn test_telex_free_tone() {
    let telex = get_input_method("Telex 2").expect("Failed to load Telex 2 layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    // Free tone mark placement (typing accent later or in the middle)
    assert_eq!(type_word(&mut engine, "hoasng"), "hoáng");
    assert_eq!(type_word(&mut engine, "truyeenj"), "truyện");
}

#[test]
fn test_telex_undo() {
    let telex = get_input_method("Telex 2").expect("Failed to load Telex 2 layout");
    let mut engine = Engine::new(telex, ESTD_FLAGS);

    // Typing dd once yields đ, typing d again undoes to dd
    assert_eq!(type_word(&mut engine, "ddd"), "dd");
    assert_eq!(type_word(&mut engine, "aas"), "ấ");
}
