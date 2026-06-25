use ibus_buffalo::{is_modifier_key, load_config, load_macro_table, new_ibus_text, save_config};

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
fn test_new_ibus_text() {
    let text = new_ibus_text("test");
    assert_eq!(text.text, "test");
    assert_eq!(text.name, "IBusText");
}

#[test]
fn test_config_and_macro_loading() {
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

    // 1. Test configuration load/save
    let mut config = load_config();
    assert_eq!(config.input_method, "Telex");

    config.input_method = "VNI".to_string();
    save_config(&config).unwrap();

    let reloaded = load_config();
    assert_eq!(reloaded.input_method, "VNI");

    // 2. Test macro table loading
    let macro_dir = temp_dir.join("ibus-buffalo");
    std::fs::create_dir_all(&macro_dir).unwrap();
    let macro_file = macro_dir.join("macro.txt");

    let content = "
# Comment line
  # Another comment

vn : Việt Nam
  ad :  anh dũng
invalid_line_no_colon
empty_val:
:empty_key
";
    std::fs::write(&macro_file, content).unwrap();

    let table = load_macro_table();
    assert_eq!(table.len(), 2);
    assert_eq!(table.get("vn").unwrap(), "Việt Nam");
    assert_eq!(table.get("ad").unwrap(), "anh dũng");

    let _ = std::fs::remove_dir_all(&temp_dir);
}
