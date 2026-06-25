//! X11 Helper Utilities for IBus Buffalo.
//!
//! This module provides functions for interacting with the X Window System (X11) via the `xlib`
//! library. It includes utilities to ignore X11 errors and retrieve properties of the active
//! focused window (such as window class or GTK application ID).

use std::ffi::CString;
use std::os::raw::{c_int, c_uchar, c_uint, c_ulong};
use std::ptr;
use x11::xlib;

/// A custom error handler that ignores X11 errors.
///
/// This avoids daemon crashes when querying windows that might be closed or destroyed mid-query.
unsafe extern "C" fn ignore_x_error(
    _display: *mut xlib::Display,
    _error: *mut xlib::XErrorEvent,
) -> c_int {
    0
}

/// Configures Xlib to use a custom error handler that silently ignores X11 errors.
pub fn set_x_ignore_error_handler() {
    unsafe {
        xlib::XSetErrorHandler(Some(ignore_x_error));
    }
}

/// Retrieves a string property from a given X11 window.
///
/// * `display` - Pointer to the X11 Display structure.
/// * `window` - The target X11 Window.
/// * `prop_name` - The name of the property to query (e.g., "WM_CLASS").
///
/// Returns `Some(String)` containing the property value if successful, or `None` otherwise.
unsafe fn get_string_property(
    display: *mut xlib::Display,
    window: xlib::Window,
    prop_name: &str,
) -> Option<String> {
    let prop_name_c = CString::new(prop_name).ok()?;
    let filter_atom = unsafe { xlib::XInternAtom(display, prop_name_c.as_ptr(), xlib::True) };
    if filter_atom == 0 {
        return None;
    }

    let mut actual_type: xlib::Atom = 0;
    let mut actual_format: c_int = 0;
    let mut n_items: c_ulong = 0;
    let mut bytes_after: c_ulong = 0;
    let mut prop_value: *mut c_uchar = ptr::null_mut();

    let status = unsafe {
        xlib::XGetWindowProperty(
            display,
            window,
            filter_atom,
            0,
            128, // MaxPropertyLen
            xlib::False,
            xlib::AnyPropertyType as xlib::Atom,
            &mut actual_type,
            &mut actual_format,
            &mut n_items,
            &mut bytes_after,
            &mut prop_value,
        )
    };

    if status == xlib::Success as c_int && !prop_value.is_null() {
        let slice = unsafe { std::slice::from_raw_parts(prop_value, n_items as usize) };
        let mut vec = slice.to_vec();
        for i in 0..vec.len() {
            if vec[i] == 0 && i + 1 < vec.len() {
                vec[i] = b':';
            }
        }
        if vec.last() == Some(&0) {
            vec.pop();
        }
        let res = String::from_utf8(vec).ok();
        unsafe { xlib::XFree(prop_value as *mut std::ffi::c_void) };
        res
    } else {
        None
    }
}

/// Traverses up the window tree from the currently focused window to find a window
/// that defines the requested property (e.g. `WM_CLASS`), skipping focus proxies.
///
/// * `display` - Pointer to the X11 Display structure.
/// * `prop_name` - The property name to lookup.
///
/// Returns `Some(String)` containing the value of the property, or `None` if not found.
unsafe fn get_focus_window_class_by_prop(
    display: *mut xlib::Display,
    prop_name: &str,
) -> Option<String> {
    let mut w: xlib::Window = 0;
    let mut revert_to: c_int = 0;
    unsafe { xlib::XGetInputFocus(display, &mut w, &mut revert_to) };

    for _ in 0..5 {
        // MaxWmClassesLen
        if w == 0 {
            break;
        }
        if let Some(str_class) = unsafe { get_string_property(display, w, prop_name) } {
            if !str_class.contains("FocusProxy") {
                return Some(str_class);
            }
        }

        let mut root_window: xlib::Window = 0;
        let mut parent_window: xlib::Window = 0;
        let mut children_windows: *mut xlib::Window = ptr::null_mut();
        let mut n_child: c_uint = 0;

        unsafe {
            xlib::XQueryTree(
                display,
                w,
                &mut root_window,
                &mut parent_window,
                &mut children_windows,
                &mut n_child,
            )
        };

        if !children_windows.is_null() {
            unsafe { xlib::XFree(children_windows as *mut std::ffi::c_void) };
        }
        if root_window == parent_window || parent_window == 0 {
            break;
        }
        w = parent_window;
    }
    None
}

/// Gets the window class name of the currently focused X11 window.
///
/// First tries querying `WM_CLASS`. If not found, falls back to `_GTK_APPLICATION_ID`.
/// Returns the sanitized class name string, or `None` if it cannot be retrieved.
pub fn x11_get_focus_window_class() -> Option<String> {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            return None;
        }
        let mut wm_class = get_focus_window_class_by_prop(display, "WM_CLASS");
        if wm_class.is_none() {
            wm_class = get_focus_window_class_by_prop(display, "_GTK_APPLICATION_ID");
        }
        xlib::XCloseDisplay(display);
        wm_class.map(|s| s.replace('"', ""))
    }
}
