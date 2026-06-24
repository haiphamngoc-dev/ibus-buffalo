//! Buffalo UI Configuration Tool.
//!
//! This crate implements a graphical configuration interface for the IBus Buffalo
//! Vietnamese input method engine. It uses Relm4 and GTK4 to present a clean,
//! modern preference dialog, allowing users to customize typing layouts,
//! charsets, and advanced spelling and tone placement options.

use buffalo_core::{CharsetEncoding, EAUTO_CORRECT_ENABLED, EFREE_TONE_MARKING, ESTD_TONE_STYLE};
use ibus_buffalo::{Config, load_config, save_config};
use relm4::gtk::gdk;
use relm4::gtk::prelude::*;
use relm4::prelude::*;
use std::str::FromStr;

/// The main application state holding the active configuration.
struct App {
    /// The application configuration loaded from and saved to disk.
    config: Config,
}

/// Messages representing user interaction events in the UI.
#[derive(Debug)]
enum Msg {
    /// Triggered when the user selects a different charset encoding from the combo box.
    CharsetChanged(u32),
    /// Triggered when the user selects a different input method (e.g., Telex, VNI) from the combo box.
    InputMethodChanged(u32),
    /// Triggered when a checkbox for an engine flag is toggled.
    ToggleFlag(u32, bool),
    /// Closes the configuration window and exits successfully.
    Close,
    /// Closes the window and restarts the IBus daemon to apply all changes globally.
    Quit,
    /// Restores all configuration fields to their default values and restarts IBus.
    RestoreDefaults,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("IBus Buffalo"),
            set_default_size: (400, -1),
            set_resizable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 0,

                // Top section containing basic settings and control buttons
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 16,
                    set_margin_all: 20,

                    // Left column: dropdown settings
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 12,
                        set_hexpand: true,

                        // Charset selection dropdown row
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 12,

                            gtk::Label {
                                set_text: "Bảng mã:",
                                set_halign: gtk::Align::Start,
                                set_width_chars: 10,
                                add_css_class: "field-label",
                            },

                            #[name = "charset_combo"]
                            gtk::ComboBoxText {
                                set_hexpand: true,
                                append_text: "Unicode",
                                append_text: "TCVN3 (ABC)",
                                append_text: "VNI Windows",
                                append_text: "VIQR",

                                connect_changed[sender] => move |combo| {
                                    let idx = combo.active().unwrap_or(0);
                                    sender.input(Msg::CharsetChanged(idx));
                                }
                            },
                        },

                        // Input method layout selection dropdown row
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 12,

                            gtk::Label {
                                set_text: "Kiểu gõ:",
                                set_halign: gtk::Align::Start,
                                set_width_chars: 10,
                                add_css_class: "field-label",
                            },

                            #[name = "im_combo"]
                            gtk::ComboBoxText {
                                set_hexpand: true,
                                append_text: "Telex",
                                append_text: "VNI",

                                connect_changed[sender] => move |combo| {
                                    let idx = combo.active().unwrap_or(0);
                                    sender.input(Msg::InputMethodChanged(idx));
                                }
                            },
                        },
                    },

                    // Right column: action control buttons
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,

                        gtk::Button {
                            set_label: "Đóng",
                            add_css_class: "accent-btn",
                            set_width_request: 110,
                            connect_clicked => Msg::Close,
                        },

                        gtk::Button {
                            set_label: "Kết thúc",
                            set_width_request: 110,
                            connect_clicked => Msg::Quit,
                        },

                        gtk::Button {
                            set_label: "Mặc định",
                            set_width_request: 110,
                            connect_clicked => Msg::RestoreDefaults,
                        },
                    },
                },

                // Advanced options section enclosed in a frame
                gtk::Frame {
                    set_label: Some("Tùy chọn nâng cao"),
                    set_margin_start: 20,
                    set_margin_end: 20,
                    set_margin_bottom: 20,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        set_margin_all: 12,

                        // Checkbox to toggle automatic Vietnamese spelling correction
                        #[name = "spell_check"]
                        gtk::CheckButton {
                            set_label: Some("Tự động sửa lỗi chính tả"),
                            connect_toggled[sender] => move |btn| {
                                sender.input(Msg::ToggleFlag(EAUTO_CORRECT_ENABLED, btn.is_active()));
                            }
                        },

                        // Checkbox to toggle modern free tone placement rules
                        #[name = "free_tone"]
                        gtk::CheckButton {
                            set_label: Some("Đặt dấu tự do (free tone marking)"),
                            connect_toggled[sender] => move |btn| {
                                sender.input(Msg::ToggleFlag(EFREE_TONE_MARKING, btn.is_active()));
                            }
                        },

                        // Checkbox to toggle standard tone styles (e.g., hòa vs hoà)
                        #[name = "std_tone"]
                        gtk::CheckButton {
                            set_label: Some("Đặt dấu kiểu mới (hòa, khỏe,...)"),
                            connect_toggled[sender] => move |btn| {
                                sender.input(Msg::ToggleFlag(ESTD_TONE_STYLE, btn.is_active()));
                            }
                        },
                    }
                },

                gtk::Separator {
                    set_orientation: gtk::Orientation::Horizontal,
                },

                // Footer section containing version details and repository links
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_start: 20,
                    set_margin_end: 20,
                    set_margin_top: 10,
                    set_margin_bottom: 10,

                    gtk::Label {
                        set_text: "IBus Buffalo v0.1.0",
                        set_halign: gtk::Align::Start,
                        set_hexpand: true,
                        add_css_class: "status-label",
                    },

                    gtk::Label {
                        set_markup: "<a href=\"https://github.com/haiphamngoc-dev/ibus-buffalo\">github.com/haiphamngoc-dev/ibus-buffalo</a>",
                        set_halign: gtk::Align::End,
                        add_css_class: "status-label",
                    },
                },
            },
        }
    }

    /// Initializes the component model and loads custom styles and widget values.
    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = load_config();
        let model = App { config };

        // Define premium custom styling rules using GTK CSS
        let provider = gtk::CssProvider::new();
        provider.load_from_data(
            "
            window {
                background-color: #f0f0f0;
            }
            .field-label {
                font-size: 13px;
                font-weight: 500;
                color: #333333;
            }
            .status-label {
                font-size: 11px;
                color: #888888;
            }
            button {
                padding: 6px 12px;
                border-radius: 4px;
                font-size: 13px;
            }
            .accent-btn {
                background-color: #2563eb;
                color: #ffffff;
                font-weight: 600;
            }
            .accent-btn:hover {
                background-color: #1d4ed8;
            }
            combobox button {
                padding: 4px 8px;
            }
            frame {
                border: 1px solid #d1d5db;
                border-radius: 6px;
                background-color: #ffffff;
            }
            frame > label {
                font-weight: 600;
                font-size: 12px;
                color: #4b5563;
                margin-bottom: 4px;
            }
        ",
        );

        // Apply style provider to the application display
        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let widgets = view_output!();

        // Initialize charset dropdown selection based on config
        let charset_idx = match CharsetEncoding::from_str(&model.config.charset) {
            Ok(CharsetEncoding::Unicode) => 0,
            Ok(CharsetEncoding::Tcvn3) => 1,
            Ok(CharsetEncoding::VniWindows) => 2,
            Ok(CharsetEncoding::Viqr) => 3,
            _ => 0,
        };
        widgets.charset_combo.set_active(Some(charset_idx));

        // Initialize input method dropdown selection based on config
        let im_idx = match model.config.input_method.as_str() {
            "Telex" => 0u32,
            "VNI" => 1,
            "English" => match model.config.vietnamese_layout.as_str() {
                "VNI" => 1,
                _ => 0,
            },
            _ => 0,
        };
        widgets.im_combo.set_active(Some(im_idx));

        // Initialize checkboxes based on config flags bitmask
        widgets
            .spell_check
            .set_active((model.config.flags & EAUTO_CORRECT_ENABLED) != 0);
        widgets
            .free_tone
            .set_active((model.config.flags & EFREE_TONE_MARKING) != 0);
        widgets
            .std_tone
            .set_active((model.config.flags & ESTD_TONE_STYLE) != 0);

        ComponentParts { model, widgets }
    }

    /// Handles application state transitions and saves configuration updates.
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Msg::CharsetChanged(idx) => {
                let charsets = CharsetEncoding::all();
                if let Some(cs) = charsets.get(idx as usize) {
                    self.config.charset = cs.to_string();
                    let _ = save_config(&self.config);
                }
            }
            Msg::InputMethodChanged(idx) => {
                let im = match idx {
                    0 => "Telex",
                    1 => "VNI",
                    _ => "Telex",
                };
                self.config.input_method = im.to_string();
                self.config.vietnamese_layout = im.to_string();
                let _ = save_config(&self.config);
            }
            Msg::ToggleFlag(flag, active) => {
                if active {
                    self.config.flags |= flag;
                } else {
                    self.config.flags &= !flag;
                }
                let _ = save_config(&self.config);
            }
            Msg::Close => {
                let _ = save_config(&self.config);
                std::process::exit(0);
            }
            Msg::Quit => {
                let _ = save_config(&self.config);
                let _ = std::process::Command::new("ibus").args(["restart"]).spawn();
                std::process::exit(0);
            }
            Msg::RestoreDefaults => {
                self.config = Config::default();
                let _ = save_config(&self.config);
                let _ = std::process::Command::new("ibus").args(["restart"]).spawn();
                std::process::exit(0);
            }
        }
    }
}

/// The main entrypoint for the configuration UI application.
fn main() {
    let app = RelmApp::new("org.freedesktop.IBus.buffalo.config");
    app.run::<App>(());
}
