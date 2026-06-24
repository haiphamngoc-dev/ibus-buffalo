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
    /// Opens the project help page in the browser.
    ShowHelp,
    /// Opens the project about page in the browser.
    ShowAbout,
    /// Closes the configuration window and exits successfully.
    Close,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("IBus Buffalo - Cấu hình"),
            set_resizable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 16,
                add_css_class: "main-container",

                // Group box "Điều khiển" using Grid Layout for perfect alignment
                gtk::Frame {
                    set_label: Some("Điều khiển"),
                    add_css_class: "control-frame",
                    set_hexpand: true,

                    gtk::Grid {
                        set_column_spacing: 12,
                        set_row_spacing: 12,
                        set_margin_all: 12,

                        // Row 0: Bảng mã
                        attach[0, 0, 1, 1] = &gtk::Label {
                            set_text: "Bảng mã:",
                            set_xalign: 0.0,
                            add_css_class: "field-label",
                        },

                        #[name = "charset_combo"]
                        attach[1, 0, 1, 1] = &gtk::ComboBoxText {
                            set_hexpand: true,
                            append_text: "Unicode",
                            append_text: "TCVN3 (ABC)",
                            append_text: "VNI Windows",
                            append_text: "VIQR",

                            set_active: Some(match CharsetEncoding::from_str(&model.config.charset) {
                                Ok(CharsetEncoding::Unicode) => 0,
                                Ok(CharsetEncoding::Tcvn3) => 1,
                                Ok(CharsetEncoding::VniWindows) => 2,
                                Ok(CharsetEncoding::Viqr) => 3,
                                _ => 0,
                            }),

                            connect_changed[sender] => move |combo| {
                                let idx = combo.active().unwrap_or(0);
                                sender.input(Msg::CharsetChanged(idx));
                            }
                        },

                        // Row 1: Kiểu gõ
                        attach[0, 1, 1, 1] = &gtk::Label {
                            set_text: "Kiểu gõ:",
                            set_xalign: 0.0,
                            add_css_class: "field-label",
                        },

                        #[name = "im_combo"]
                        attach[1, 1, 1, 1] = &gtk::ComboBoxText {
                            set_hexpand: true,
                            append_text: "Telex",
                            append_text: "VNI",

                            set_active: Some(match model.config.input_method.as_str() {
                                "Telex" => 0,
                                "VNI" => 1,
                                _ => 0,
                            }),

                            connect_changed[sender] => move |combo| {
                                let idx = combo.active().unwrap_or(0);
                                sender.input(Msg::InputMethodChanged(idx));
                            }
                        },
                    }
                },

                // Collapsible Advanced Options
                #[name = "advanced_frame"]
                gtk::Frame {
                    set_label: Some("Tùy chọn nâng cao"),
                    add_css_class: "control-frame",
                    set_hexpand: true,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        set_margin_all: 12,

                        #[name = "spell_check"]
                        gtk::CheckButton {
                            set_label: Some("Tự động sửa lỗi chính tả"),
                            set_active: (model.config.flags & EAUTO_CORRECT_ENABLED) != 0,
                            connect_toggled[sender] => move |btn| {
                                sender.input(Msg::ToggleFlag(EAUTO_CORRECT_ENABLED, btn.is_active()));
                            }
                        },

                        #[name = "free_tone"]
                        gtk::CheckButton {
                            set_label: Some("Đặt dấu tự do (free tone marking)"),
                            set_active: (model.config.flags & EFREE_TONE_MARKING) != 0,
                            connect_toggled[sender] => move |btn| {
                                sender.input(Msg::ToggleFlag(EFREE_TONE_MARKING, btn.is_active()));
                            }
                        },

                        #[name = "std_tone"]
                        gtk::CheckButton {
                            set_label: Some("Đặt dấu kiểu mới (hòa, khỏe,...)"),
                            set_active: (model.config.flags & ESTD_TONE_STYLE) != 0,
                            connect_toggled[sender] => move |btn| {
                                sender.input(Msg::ToggleFlag(ESTD_TONE_STYLE, btn.is_active()));
                            }
                        },
                    }
                },

                // Bottom row of action buttons: Mở rộng, Thông tin, Đóng
                // homogeneous: true ensures equal width distribution
                // hexpand: true makes the container fill the window width, aligning perfectly with the frames
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 12,
                    set_homogeneous: true,
                    set_hexpand: true,

                    gtk::Button {
                        set_label: "Hướng dẫn",
                        add_css_class: "btn-normal",
                        connect_clicked => Msg::ShowHelp,
                    },

                    gtk::Button {
                        set_label: "Thông tin",
                        add_css_class: "btn-normal",
                        connect_clicked => Msg::ShowAbout,
                    },

                    gtk::Button {
                        set_label: "Đóng",
                        add_css_class: "btn-close",
                        connect_clicked => Msg::Close,
                    }
                }
            }
        }
    }

    /// Initializes the component model and loads custom styles.
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
                background-color: #f3f4f6;
            }
            .main-container {
                padding: 4px;
            }
            .control-frame {
                border: 1px solid #d1d5db;
                border-radius: 6px;
                background-color: #ffffff;
            }
            .control-frame > label {
                font-weight: bold;
                font-size: 13px;
                color: #374151;
            }
            .field-label {
                font-size: 13px;
                font-weight: 500;
                color: #374151;
            }
            /* Style only our custom action buttons, not combobox buttons */
            .btn-close, .btn-normal {
                padding: 6px 12px;
                border-radius: 6px;
                font-size: 13px;
                font-weight: 500;
                transition: all 0.15s ease-in-out;
            }
            /* Prominent, high-contrast Close (Đóng) button */
            .btn-close {
                background-color: #2563eb;
                background-image: none;
                color: #ffffff;
                font-weight: bold;
                border: 1px solid #1d4ed8;
            }
            .btn-close:hover {
                background-color: #1d4ed8;
            }
            .btn-normal {
                background-color: #ffffff;
                background-image: none;
                color: #374151;
                border: 1px solid #d1d5db;
            }
            .btn-normal:hover {
                background-color: #f3f4f6;
                border-color: #9ca3af;
            }
            checkbutton {
                font-size: 13px;
                font-weight: 500;
                color: #374151;
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
            Msg::ShowHelp => {
                let _ = std::process::Command::new("xdg-open")
                    .arg("https://github.com/haiphamngoc-dev/ibus-buffalo#readme")
                    .spawn();
            }
            Msg::ShowAbout => {
                let _ = std::process::Command::new("xdg-open")
                    .arg("https://github.com/haiphamngoc-dev/ibus-buffalo")
                    .spawn();
            }
            Msg::Close => {
                let _ = save_config(&self.config);
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
