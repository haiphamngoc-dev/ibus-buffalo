//! Buffalo UI Configuration Tool.
//!
//! This crate implements a graphical configuration interface for the IBus Buffalo
//! Vietnamese input method engine. It uses Relm4 and GTK4 to present a clean,
//! modern preference dialog, allowing users to customize typing layouts,
//! charsets, and advanced spelling and tone placement options.

pub mod app;
pub mod style;

use app::App;
use relm4::prelude::*;

/// The main entrypoint for the configuration UI application.
fn main() {
    let app = RelmApp::new("org.freedesktop.IBus.buffalo.config");
    app.run::<App>(());
}
