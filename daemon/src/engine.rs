use buffalo_core::{
    CharsetEncoding, Engine, VIETNAMESE_MODE, encode, get_input_method, is_word_break_symbol,
};
use std::str::FromStr;
use std::sync::Mutex;
use zbus::interface;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{OwnedValue, Value};

use crate::config::{load_config, save_config};
use crate::dbus_types::{get_prop_list, new_ibus_text};
use crate::utils::{
    BACKSPACE_FORWARDING_IM, FORWARD_AS_COMMIT_IM, IBUS_BACKSPACE, IBUS_CONTROL_MASK, IBUS_ESCAPE,
    IBUS_HYPER_MASK, IBUS_LEFT, IBUS_LOCK_MASK, IBUS_META_MASK, IBUS_MOD1_MASK, IBUS_MOD4_MASK,
    IBUS_RELEASE_MASK, IBUS_RETURN, IBUS_SHIFT_MASK, IBUS_SUPER_MASK, IBUS_TAB, PREEDIT_IM,
    SHIFT_LEFT_FORWARDING_IM, SURROUNDING_TEXT_IM, XTEST_FAKE_KEY_EVENT_IM, get_focus_window_class,
    get_offset_runes, get_ui_executable_path, is_backspace_mode, is_modifier_key,
};

/// The main IBus Engine instance managing state, input method engine, active window tracking,
/// and preedit buffer tracking.
pub struct IBusEngine {
    /// The core buffalo typing engine.
    pub engine: Mutex<Engine>,
    /// The class name of the currently focused window.
    pub wm_class: Mutex<String>,
    /// The last committed text, used for offset calculations.
    pub last_text: Mutex<String>,
}

impl IBusEngine {
    /// Creates a new `IBusEngine` instance.
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Mutex::new(engine),
            wm_class: Mutex::new(String::new()),
            last_text: Mutex::new(String::new()),
        }
    }
}

#[interface(name = "org.freedesktop.IBus.Engine")]
impl IBusEngine {
    /// D-Bus method invoked by IBus to process key events.
    ///
    /// * `signal_emitter` - Object used to emit D-Bus signals.
    /// * `keyval` - Key symbol value.
    /// * `keycode` - Hardware keycode.
    /// * `state` - Key modifier state mask.
    ///
    /// Returns `true` if the engine handles the key event, or `false` to let the system handle it.
    async fn process_key_event(
        &self,
        #[zbus(signal_emitter)] signal_emitter: SignalEmitter<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> bool {
        debug_println!(
            "--> [D-Bus] ProcessKeyEvent: keyval={:#x}, keycode={}, state={:#x}",
            keyval,
            keycode,
            state
        );
        let handled = self
            .handle_key(&signal_emitter, keyval, keycode, state)
            .await;
        debug_println!("--> [D-Bus] ProcessKeyEvent Result: {}", handled);
        handled
    }

    /// D-Bus method invoked when the input method engine gains focus.
    /// Updates the tracked window class and registers engine properties.
    async fn focus_in(&self, #[zbus(signal_emitter)] signal_emitter: SignalEmitter<'_>) {
        let wm = get_focus_window_class().await;
        println!("FocusIn: window class = {}", wm);
        {
            let mut current_wm = self.wm_class.lock().unwrap();
            if *current_wm != wm {
                *current_wm = wm;
                let mut engine = self.engine.lock().unwrap();
                engine.reset();
                let mut lt = self.last_text.lock().unwrap();
                lt.clear();
            }
        }

        let config = load_config();
        let props = get_prop_list(&config);
        let props_val = OwnedValue::try_from(props).unwrap();
        let _ = Self::register_properties(&signal_emitter, Value::from(props_val)).await;
    }

    /// D-Bus method invoked when the input method engine loses focus.
    async fn focus_out(&self) {}

    /// D-Bus method invoked to reset the engine state.
    async fn reset(&self, #[zbus(signal_emitter)] signal_emitter: SignalEmitter<'_>) {
        self.commit_and_reset(&signal_emitter, PREEDIT_IM).await;
    }

    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn enable(&self) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn disable(&self) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn set_cursor_location(&self, _x: i32, _y: i32, _w: i32, _h: i32) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn set_capabilities(&self, _cap: u32) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn set_surrounding_text(&self, _text: Value<'_>, _cursor_index: u32, _anchor_pos: u32) {}

    /// D-Bus method invoked when an IBus menu property is clicked or toggled.
    async fn property_activate(
        &self,
        #[zbus(signal_emitter)] signal_emitter: SignalEmitter<'_>,
        prop_name: String,
        prop_state: u32,
    ) {
        println!(
            "PropertyActivate: name = {}, state = {}",
            prop_name, prop_state
        );

        if prop_name == "about" {
            let ui_path = get_ui_executable_path();
            let _ = std::process::Command::new(ui_path).spawn();
            return;
        }

        let mut config = load_config();

        if prop_name.starts_with("InputMethod::") {
            if prop_state == 1 {
                let im = &prop_name["InputMethod::".len()..];
                config.input_method = im.to_string();
                let _ = save_config(&config);
            }
        } else if prop_name.starts_with("Charset::") {
            if prop_state == 1 {
                let cs = &prop_name["Charset::".len()..];
                config.charset = cs.to_string();
                let _ = save_config(&config);
            }
        }

        let active_layout = &config.input_method;
        if let Some(im_def) = get_input_method(active_layout) {
            let mut engine = self.engine.lock().unwrap();
            *engine = Engine::new(im_def, config.flags);
        }

        let props = get_prop_list(&config);
        let props_val = OwnedValue::try_from(props).unwrap();
        let _ = Self::register_properties(&signal_emitter, Value::from(props_val)).await;
    }

    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn property_show(&self, _prop_name: String) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn property_hide(&self, _prop_name: String) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn page_up(&self) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn page_down(&self) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn cursor_up(&self) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn cursor_down(&self) {}
    /// D-Bus methods required by IBus Engine interface but not actively implemented.
    async fn candidate_clicked(&self, _index: u32, _button: u32, _state: u32) {}

    /// Signal emitted to commit completed text to the application.
    #[zbus(signal)]
    pub async fn commit_text(
        signal_emitter: &SignalEmitter<'_>,
        text: Value<'_>,
    ) -> zbus::Result<()>;

    /// Signal emitted to update the inline preedit composition text.
    #[zbus(signal)]
    pub async fn update_preedit_text(
        signal_emitter: &SignalEmitter<'_>,
        text: Value<'_>,
        cursor_pos: u32,
        visible: bool,
        mode: u32,
    ) -> zbus::Result<()>;

    /// Signal emitted to hide the inline preedit composition.
    #[zbus(signal)]
    pub async fn hide_preedit_text(signal_emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    /// Signal emitted to forward key events back to the client application.
    #[zbus(signal)]
    pub async fn forward_key_event(
        signal_emitter: &SignalEmitter<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> zbus::Result<()>;

    /// Signal emitted to delete surrounding text relative to the cursor.
    #[zbus(signal)]
    pub async fn delete_surrounding_text(
        signal_emitter: &SignalEmitter<'_>,
        offset: i32,
        n_chars: u32,
    ) -> zbus::Result<()>;

    /// Signal emitted to register configuration properties with IBus.
    #[zbus(signal)]
    pub async fn register_properties(
        signal_emitter: &SignalEmitter<'_>,
        props: Value<'_>,
    ) -> zbus::Result<()>;

    /// Signal emitted to update specific configuration property states.
    #[zbus(signal)]
    pub async fn update_property(
        signal_emitter: &SignalEmitter<'_>,
        prop: Value<'_>,
    ) -> zbus::Result<()>;
}

impl IBusEngine {
    /// Internal key handler managing layout processing, modifiers check, and keystroke dispatching.
    pub async fn handle_key(
        &self,
        signal_emitter: &SignalEmitter<'_>,
        keyval: u32,
        _keycode: u32,
        state: u32,
    ) -> bool {
        debug_println!(
            "--> handle_key: keyval={:#x}, keycode={}, state={:#x}",
            keyval,
            _keycode,
            state
        );
        let _config = load_config();
        let is_release = (state & IBUS_RELEASE_MASK) != 0;

        if is_release {
            debug_println!("--> handle_key ignored (release event)");
            return false;
        }

        if is_modifier_key(keyval) {
            debug_println!(
                "--> handle_key ignored (modifier key itself: keyval={:#x})",
                keyval
            );
            return false;
        }

        let input_mode = PREEDIT_IM;
        let charset =
            CharsetEncoding::from_str(&_config.charset).unwrap_or(CharsetEncoding::Unicode);
        debug_println!(
            "--> handle_key: input_mode={}, config.input_method='{}', charset='{}'",
            input_mode,
            _config.input_method,
            _config.charset
        );

        let has_modifiers = (state
            & (IBUS_CONTROL_MASK
                | IBUS_LOCK_MASK
                | IBUS_MOD1_MASK
                | IBUS_MOD4_MASK
                | IBUS_SUPER_MASK
                | IBUS_HYPER_MASK
                | IBUS_META_MASK))
            != 0;

        if has_modifiers {
            debug_println!("--> handle_key ignored (has modifiers: state={:#x})", state);
            self.commit_and_reset(signal_emitter, input_mode).await;
            return false;
        }

        if keyval == IBUS_BACKSPACE {
            debug_println!("--> handle_key: Backspace");
            return self.handle_backspace(signal_emitter, input_mode).await;
        }
        if keyval == IBUS_RETURN || keyval == IBUS_ESCAPE {
            debug_println!("--> handle_key: Return/Escape");
            self.commit_and_reset(signal_emitter, input_mode).await;
            return false;
        }
        if keyval == IBUS_TAB {
            debug_println!("--> handle_key: Tab");
            self.commit_and_reset(signal_emitter, input_mode).await;
            return false;
        }

        // X11 keysyms in the range 0xff00 to 0xffff are special function keys (e.g. arrows, F1-F12, Home, End)
        // and should not be treated as printable characters.
        if keyval < 0xff00 {
            if let Some(c) = char::from_u32(keyval) {
                if c.is_control() {
                    debug_println!("--> handle_key: control character '{:?}'", c);
                    self.commit_and_reset(signal_emitter, input_mode).await;
                    return false;
                }

                let (can_process, old_text, new_text) = {
                    let mut engine = self.engine.lock().unwrap();
                    if engine.can_process_key(c) {
                        let old = engine.get_processed_string(VIETNAMESE_MODE);
                        engine.process_key(c, VIETNAMESE_MODE);
                        let new = engine.get_processed_string(VIETNAMESE_MODE);
                        debug_println!("--> engine processed key '{}': '{}' -> '{}'", c, old, new);
                        (true, old, new)
                    } else {
                        debug_println!("--> engine cannot process key '{}'", c);
                        (false, String::new(), String::new())
                    }
                };

                if can_process {
                    let encoded_new = encode(&charset, &new_text);
                    if is_backspace_mode(input_mode) {
                        let encoded_old = encode(&charset, &old_text);
                        let (suffix, n_bs) = get_offset_runes(&encoded_new, &encoded_old);
                        debug_println!("--> is_backspace_mode: suffix='{}', n_bs={}", suffix, n_bs);
                        Self::send_backspace(signal_emitter, input_mode, n_bs).await;
                        if !suffix.is_empty() {
                            let ibus_text = new_ibus_text(&suffix);
                            debug_println!("--> committing suffix: '{}'", suffix);
                            let _ = Self::commit_text(signal_emitter, Value::from(ibus_text)).await;
                        }
                        let mut lt = self.last_text.lock().unwrap();
                        *lt = encoded_new;
                    } else {
                        debug_println!("--> updating preedit: '{}'", encoded_new);
                        let ibus_text = new_ibus_text(&encoded_new);
                        let _ = Self::update_preedit_text(
                            signal_emitter,
                            Value::from(ibus_text),
                            encoded_new.chars().count() as u32,
                            true,
                            1,
                        )
                        .await;
                    }
                    return true;
                } else if is_word_break_symbol(c) {
                    debug_println!("--> is_word_break_symbol: '{}'", c);
                    let old_text = {
                        let mut engine = self.engine.lock().unwrap();
                        let old = engine.get_processed_string(VIETNAMESE_MODE);
                        engine.reset();
                        old
                    };

                    let encoded_old = encode(&charset, &old_text);
                    let committed = format!("{}{}", encoded_old, c);
                    if is_backspace_mode(input_mode) {
                        debug_println!("--> backspace_mode committing: '{}'", c);
                        let ibus_text = new_ibus_text(&c.to_string());
                        let _ = Self::commit_text(signal_emitter, Value::from(ibus_text)).await;
                        let mut lt = self.last_text.lock().unwrap();
                        lt.clear();
                    } else {
                        debug_println!("--> committing final word: '{}'", committed);
                        let ibus_text = new_ibus_text(&committed);
                        let _ = Self::commit_text(signal_emitter, Value::from(ibus_text)).await;
                        let _ = Self::hide_preedit_text(signal_emitter).await;
                    }
                    return true;
                }
            }
        }

        debug_println!("--> handle_key fallback: commit_and_reset and returning false");
        self.commit_and_reset(signal_emitter, input_mode).await;
        false
    }

    /// Internal handler to process the Backspace keystroke.
    /// Returns true if handled by removing characters from the preedit buffer.
    pub async fn handle_backspace(
        &self,
        signal_emitter: &SignalEmitter<'_>,
        input_mode: i32,
    ) -> bool {
        let (preedit_is_empty, new_text) = {
            let mut engine = self.engine.lock().unwrap();
            let preedit = engine.get_processed_string(VIETNAMESE_MODE);
            if preedit.is_empty() {
                (true, String::new())
            } else {
                engine.remove_last_char(true);
                let new_t = engine.get_processed_string(VIETNAMESE_MODE);
                (false, new_t)
            }
        };

        if preedit_is_empty {
            return false;
        }

        let config = load_config();
        let charset =
            CharsetEncoding::from_str(&config.charset).unwrap_or(CharsetEncoding::Unicode);
        let encoded_text = encode(&charset, &new_text);

        if is_backspace_mode(input_mode) {
            let mut lt = self.last_text.lock().unwrap();
            *lt = encoded_text;
            false
        } else {
            if encoded_text.is_empty() {
                let _ = Self::hide_preedit_text(signal_emitter).await;
            } else {
                let ibus_text = new_ibus_text(&encoded_text);
                let _ = Self::update_preedit_text(
                    signal_emitter,
                    Value::from(ibus_text),
                    encoded_text.chars().count() as u32,
                    true,
                    1,
                )
                .await;
            }
            true
        }
    }

    /// Commits any active preedit buffer to the client application and resets the engine.
    pub async fn commit_and_reset(&self, signal_emitter: &SignalEmitter<'_>, input_mode: i32) {
        let preedit = {
            let mut engine = self.engine.lock().unwrap();
            let preedit = engine.get_processed_string(VIETNAMESE_MODE);
            engine.reset();
            preedit
        };

        if !preedit.is_empty() {
            if is_backspace_mode(input_mode) {
                let mut lt = self.last_text.lock().unwrap();
                lt.clear();
            } else {
                let config = load_config();
                let charset =
                    CharsetEncoding::from_str(&config.charset).unwrap_or(CharsetEncoding::Unicode);
                let encoded = encode(&charset, &preedit);
                let ibus_text = new_ibus_text(&encoded);
                let _ = Self::commit_text(signal_emitter, Value::from(ibus_text)).await;
                let _ = Self::hide_preedit_text(signal_emitter).await;
            }
        }
    }

    /// Simulates sending Backspace commands depending on the active input mode.
    pub async fn send_backspace(signal_emitter: &SignalEmitter<'_>, im_mode: i32, n: usize) {
        if n == 0 {
            return;
        }
        debug_println!("--> send_backspace: mode={}, n={}", im_mode, n);
        match im_mode {
            SURROUNDING_TEXT_IM => {
                let _ = Self::delete_surrounding_text(signal_emitter, -(n as i32), n as u32).await;
            }
            BACKSPACE_FORWARDING_IM | FORWARD_AS_COMMIT_IM => {
                for _ in 0..n {
                    let _ = Self::forward_key_event(signal_emitter, IBUS_BACKSPACE, 22, 0).await;
                    let _ = Self::forward_key_event(
                        signal_emitter,
                        IBUS_BACKSPACE,
                        22,
                        IBUS_RELEASE_MASK,
                    )
                    .await;
                }
            }
            SHIFT_LEFT_FORWARDING_IM => {
                for _ in 0..n {
                    let _ =
                        Self::forward_key_event(signal_emitter, IBUS_LEFT, 113, IBUS_SHIFT_MASK)
                            .await;
                    let _ =
                        Self::forward_key_event(signal_emitter, IBUS_LEFT, 113, IBUS_RELEASE_MASK)
                            .await;
                }
            }
            XTEST_FAKE_KEY_EVENT_IM => {
                crate::x11_helper::x11_send_backspace(n, 10);
            }
            _ => {}
        }
    }
}
