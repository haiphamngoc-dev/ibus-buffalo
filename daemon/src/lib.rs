//! IBus Buffalo Input Method Daemon Library.
//!
//! This library provides the core component logic for the IBus Buffalo input method daemon,
//! including configurations, zbus D-Bus interface implementations, and event loop logic.

#[macro_use]
pub mod macros;

pub mod config;
pub mod dbus_types;
pub mod engine;
pub mod factory;
pub mod server;
pub mod utils;
pub mod x11_helper;

// Re-export configuration interface
pub use config::{Config, load_config, load_macro_table, save_config};

// Re-export D-Bus IBus compatible types
pub use dbus_types::{
    IBusAttrList, IBusAttribute, IBusPropList, IBusProperty, IBusText, get_prop_list,
    new_ibus_property, new_ibus_text,
};

// Re-export engine and factory zbus interfaces
pub use engine::IBusEngine;
pub use factory::IBusFactory;

// Re-export daemon entrypoint
pub use server::run_daemon;

// Re-export utility constants and helpers
pub use utils::{
    IBUS_BACKSPACE, IBUS_CONTROL_MASK, IBUS_ESCAPE, IBUS_HYPER_MASK, IBUS_LEFT, IBUS_LOCK_MASK,
    IBUS_META_MASK, IBUS_MOD1_MASK, IBUS_MOD4_MASK, IBUS_RELEASE_MASK, IBUS_RETURN,
    IBUS_SHIFT_MASK, IBUS_SUPER_MASK, IBUS_TAB, get_ibus_address, get_local_machine_id,
    get_ui_executable_path, get_user_config_dir, is_modifier_key,
};
