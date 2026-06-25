use crate::config::Config;
use crate::utils::{PREEDIT_IM, SURROUNDING_TEXT_IM};
use buffalo_core::CharsetEncoding;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::zvariant::OwnedValue;

/// Represents an IBus text attribute, defining text formatting like underline, color, etc.
#[derive(
    Serialize,
    Deserialize,
    zbus::zvariant::Type,
    zbus::zvariant::Value,
    zbus::zvariant::OwnedValue,
    Debug,
)]
pub struct IBusAttribute {
    /// Attribute name (always "IBusAttribute").
    pub name: String,
    /// Metadata attachments.
    pub attachments: HashMap<String, OwnedValue>,
    /// Type of attribute.
    pub attr_type: u32,
    /// Value associated with the attribute.
    pub value: u32,
    /// Starting character index in the text.
    pub start_index: u32,
    /// Ending character index in the text.
    pub end_index: u32,
}

/// Represents a list of text attributes in IBus.
#[derive(
    Serialize,
    Deserialize,
    zbus::zvariant::Type,
    zbus::zvariant::Value,
    zbus::zvariant::OwnedValue,
    Debug,
)]
pub struct IBusAttrList {
    /// Struct name (always "IBusAttrList").
    pub name: String,
    /// Metadata attachments.
    pub attachments: HashMap<String, OwnedValue>,
    /// List of serializable attributes.
    pub attributes: Vec<OwnedValue>,
}

/// Represents a piece of text along with its formatting attributes in IBus.
#[derive(
    Serialize,
    Deserialize,
    zbus::zvariant::Type,
    zbus::zvariant::Value,
    zbus::zvariant::OwnedValue,
    Debug,
)]
pub struct IBusText {
    /// Struct name (always "IBusText").
    pub name: String,
    /// Metadata attachments.
    pub attachments: HashMap<String, OwnedValue>,
    /// The actual text string.
    pub text: String,
    /// Attributes associated with the text.
    pub attr_list: OwnedValue,
}

/// Helper function to construct a new `IBusText` without any formatting attributes.
///
/// * `text` - The string slice to wrap.
pub fn new_ibus_text(text: &str) -> IBusText {
    let attr_list = IBusAttrList {
        name: "IBusAttrList".to_string(),
        attachments: HashMap::new(),
        attributes: Vec::new(),
    };
    let attr_list_value = OwnedValue::try_from(attr_list).unwrap();
    IBusText {
        name: "IBusText".to_string(),
        attachments: HashMap::new(),
        text: text.to_string(),
        attr_list: attr_list_value,
    }
}

/// Represents a property item in the IBus status menu/bar.
#[derive(
    Serialize,
    Deserialize,
    zbus::zvariant::Type,
    zbus::zvariant::Value,
    zbus::zvariant::OwnedValue,
    Debug,
)]
pub struct IBusProperty {
    /// Property name.
    pub name: String,
    /// Metadata attachments.
    pub attachments: HashMap<String, OwnedValue>,
    /// Unique key string.
    pub key: String,
    /// Type of property (e.g. menu, toggle).
    pub prop_type: u32,
    /// Property label.
    pub label: OwnedValue,
    /// Icon name.
    pub icon: String,
    /// Tooltip text.
    pub tooltip: OwnedValue,
    /// Sensitivity state.
    pub sensitive: bool,
    /// Visibility state.
    pub visible: bool,
    /// Selected/checked state.
    pub state: u32,
    /// Sub-properties (for menu nesting).
    pub sub_props: OwnedValue,
    /// Symbol string.
    pub symbol: OwnedValue,
}

/// List wrapper for multiple `IBusProperty` instances.
#[derive(
    Serialize,
    Deserialize,
    zbus::zvariant::Type,
    zbus::zvariant::Value,
    zbus::zvariant::OwnedValue,
    Debug,
)]
pub struct IBusPropList {
    /// Struct name (always "IBusPropList").
    pub name: String,
    /// Metadata attachments.
    pub attachments: HashMap<String, OwnedValue>,
    /// Serializable property items.
    pub properties: Vec<OwnedValue>,
}

/// Helper function to construct a new `IBusProperty` object.
///
/// * `key` - Unique key name.
/// * `prop_type` - Type of the property.
/// * `label` - Label text.
/// * `icon` - Icon name.
/// * `state` - Initial state of the property.
/// * `sub_props` - List of child sub-properties, if any.
pub fn new_ibus_property(
    key: &str,
    prop_type: u32,
    label: &str,
    icon: &str,
    state: u32,
    sub_props: Option<IBusPropList>,
) -> IBusProperty {
    let label_val = OwnedValue::try_from(new_ibus_text(label)).unwrap();
    let tooltip_val = OwnedValue::try_from(new_ibus_text("")).unwrap();
    let symbol_val = OwnedValue::try_from(new_ibus_text("")).unwrap();
    let sub_props_val = if let Some(sp) = sub_props {
        OwnedValue::try_from(sp).unwrap()
    } else {
        let empty_list = IBusPropList {
            name: "IBusPropList".to_string(),
            attachments: HashMap::new(),
            properties: Vec::new(),
        };
        OwnedValue::try_from(empty_list).unwrap()
    };

    IBusProperty {
        name: "IBusProperty".to_string(),
        attachments: HashMap::new(),
        key: key.to_string(),
        prop_type,
        label: label_val,
        icon: icon.to_string(),
        tooltip: tooltip_val,
        sensitive: true,
        visible: true,
        state,
        sub_props: sub_props_val,
        symbol: symbol_val,
    }
}

/// Generates the list of standard settings and options displayed in the IBus menu.
///
/// * `config` - Reference to the active configuration.
pub fn get_prop_list(config: &Config) -> IBusPropList {
    let mut properties = Vec::new();

    let about_label = format!("IBus Buffalo v{}", env!("CARGO_PKG_VERSION"));
    let about_prop = new_ibus_property("about", 0, &about_label, "gtk-about", 0, None);
    properties.push(OwnedValue::try_from(about_prop).unwrap());

    let mut im_subprops = Vec::new();
    let ims = vec!["Telex", "Simple Telex", "VNI"];
    for im in ims {
        let state = if config.input_method == im { 1 } else { 0 };
        let prop = new_ibus_property(&format!("InputMethod::{}", im), 2, im, "", state, None);
        im_subprops.push(OwnedValue::try_from(prop).unwrap());
    }
    let im_menu = new_ibus_property(
        "input_method_menu",
        3,
        "Kiểu gõ",
        "preferences-desktop",
        0,
        Some(IBusPropList {
            name: "IBusPropList".to_string(),
            attachments: HashMap::new(),
            properties: im_subprops,
        }),
    );
    properties.push(OwnedValue::try_from(im_menu).unwrap());

    // Input mode submenu (Cơ chế gõ)
    let mut mode_subprops = Vec::new();
    let modes = vec![
        (PREEDIT_IM, "Preedit"),
        (SURROUNDING_TEXT_IM, "Surrounding Text"),
    ];
    for (mode_val, mode_label) in modes {
        let state = if config.default_input_mode == mode_val {
            1
        } else {
            0
        };
        let prop = new_ibus_property(
            &format!("InputMode::{}", mode_val),
            2,
            mode_label,
            "",
            state,
            None,
        );
        mode_subprops.push(OwnedValue::try_from(prop).unwrap());
    }
    let mode_menu = new_ibus_property(
        "input_mode_menu",
        3,
        "Cơ chế gõ",
        "input-keyboard",
        0,
        Some(IBusPropList {
            name: "IBusPropList".to_string(),
            attachments: HashMap::new(),
            properties: mode_subprops,
        }),
    );
    properties.push(OwnedValue::try_from(mode_menu).unwrap());

    // Charset encoding submenu
    let mut charset_subprops = Vec::new();
    for cs in CharsetEncoding::all() {
        let cs_key = cs.to_string();
        let state = if config.charset == cs_key { 1 } else { 0 };
        let prop = new_ibus_property(
            &format!("Charset::{}", cs_key),
            2,
            cs.display_name(),
            "",
            state,
            None,
        );
        charset_subprops.push(OwnedValue::try_from(prop).unwrap());
    }
    let charset_menu = new_ibus_property(
        "charset_menu",
        3,
        "Bảng mã",
        "preferences-desktop-font",
        0,
        Some(IBusPropList {
            name: "IBusPropList".to_string(),
            attachments: HashMap::new(),
            properties: charset_subprops,
        }),
    );
    properties.push(OwnedValue::try_from(charset_menu).unwrap());

    IBusPropList {
        name: "IBusPropList".to_string(),
        attachments: HashMap::new(),
        properties,
    }
}
