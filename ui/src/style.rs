/// Returns the custom premium CSS styling rules for the configuration UI.
pub fn get_custom_css() -> &'static str {
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
    "
}
