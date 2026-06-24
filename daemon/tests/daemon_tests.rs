use ibus_buffalo::{
    BACKSPACE_FORWARDING_IM, PREEDIT_IM, SURROUNDING_TEXT_IM, get_offset_runes, is_backspace_mode,
    is_modifier_key, load_config, new_ibus_text, save_config,
};

#[test]
fn test_is_modifier_key() {
    assert!(is_modifier_key(0xffe1)); // Shift_L
    assert!(is_modifier_key(0xffe3)); // Control_L
    assert!(is_modifier_key(0xffe9)); // Alt_L
    assert!(is_modifier_key(0xffeb)); // Super_L
    assert!(is_modifier_key(0xfe03)); // ISO_Level3_Shift
    assert!(is_modifier_key(0xff7e)); // Mode_switch

    assert!(!is_modifier_key(0x0020)); // Space
    assert!(!is_modifier_key(0x0061)); // a
    assert!(!is_modifier_key(0xff08)); // Backspace
    assert!(!is_modifier_key(0xff0d)); // Return
}

#[test]
fn test_is_backspace_mode() {
    assert!(!is_backspace_mode(PREEDIT_IM));
    assert!(is_backspace_mode(SURROUNDING_TEXT_IM));
    assert!(is_backspace_mode(BACKSPACE_FORWARDING_IM));
}

#[test]
fn test_get_offset_runes() {
    // Appending characters
    let (suffix, n_bs) = get_offset_runes("hello", "hell");
    assert_eq!(suffix, "o");
    assert_eq!(n_bs, 0);

    // Deleting characters
    let (suffix, n_bs) = get_offset_runes("he", "hello");
    assert_eq!(suffix, "");
    assert_eq!(n_bs, 3);

    // Modifying/replacing characters
    let (suffix, n_bs) = get_offset_runes("hallo", "hello");
    assert_eq!(suffix, "allo");
    assert_eq!(n_bs, 4);
}

#[test]
fn test_new_ibus_text() {
    let text = new_ibus_text("test");
    assert_eq!(text.text, "test");
    assert_eq!(text.name, "IBusText");
}

#[test]
fn test_config_load_save() {
    let temp_dir = std::env::temp_dir().join(format!(
        "ibus-buffalo-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Safety: set_var is unsafe in recent Rust versions due to multi-threaded environments,
    // but here we are in a controlled test environment.
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", &temp_dir);
    }

    let mut config = load_config();
    assert_eq!(config.input_method, "Telex");

    config.input_method = "VNI".to_string();
    save_config(&config).unwrap();

    let reloaded = load_config();
    assert_eq!(reloaded.input_method, "VNI");

    let _ = std::fs::remove_dir_all(&temp_dir);
}
