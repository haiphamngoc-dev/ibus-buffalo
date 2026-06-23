//! Vietnamese Charset Encoder.
//!
//! This module provides charset encoding support for converting Unicode Vietnamese text
//! to various legacy encoding formats. The engine internally processes everything in Unicode,
//! and this module performs post-processing to convert the output to the target charset.
//!
//! Supported charsets:
//! - **Unicode**: No conversion (identity)
//! - **TCVN3 (ABC)**: Legacy encoding used with .VnTimes and similar fonts
//! - **VNI Windows**: Legacy VNI encoding for Windows
//! - **VIQR**: ASCII-based representation of Vietnamese diacritics
//!

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Represents the available output charset encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharsetEncoding {
    /// Standard Unicode (UTF-8). No conversion needed.
    Unicode,
    /// TCVN3 (ABC) legacy encoding.
    Tcvn3,
    /// VNI Windows legacy encoding.
    VniWindows,
    /// VIQR ASCII-based encoding.
    Viqr,
}

impl CharsetEncoding {
    /// Returns the human-readable display name for the charset.
    pub fn display_name(&self) -> &str {
        match self {
            CharsetEncoding::Unicode => "Unicode",
            CharsetEncoding::Tcvn3 => "TCVN3 (ABC)",
            CharsetEncoding::VniWindows => "VNI Windows",
            CharsetEncoding::Viqr => "VIQR",
        }
    }

    /// Returns a list of all available charset encodings.
    pub fn all() -> Vec<CharsetEncoding> {
        vec![
            CharsetEncoding::Unicode,
            CharsetEncoding::Tcvn3,
            CharsetEncoding::VniWindows,
            CharsetEncoding::Viqr,
        ]
    }
}

impl fmt::Display for CharsetEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharsetEncoding::Unicode => write!(f, "Unicode"),
            CharsetEncoding::Tcvn3 => write!(f, "TCVN3"),
            CharsetEncoding::VniWindows => write!(f, "VNI"),
            CharsetEncoding::Viqr => write!(f, "VIQR"),
        }
    }
}

impl FromStr for CharsetEncoding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unicode" => Ok(CharsetEncoding::Unicode),
            "TCVN3" => Ok(CharsetEncoding::Tcvn3),
            "VNI" => Ok(CharsetEncoding::VniWindows),
            "VIQR" => Ok(CharsetEncoding::Viqr),
            _ => Err(format!("Unknown charset: {}", s)),
        }
    }
}

lazy_static::lazy_static! {
    /// TCVN3 (ABC) charset mapping from Unicode characters to their TCVN3 encoded strings.
    static ref TCVN3_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        // Lowercase
        m.insert('đ', "\u{00AE}");
        m.insert('â', "\u{00A9}");
        m.insert('ă', "\u{00A8}");
        m.insert('ê', "\u{00AA}");
        m.insert('ô', "\u{00AB}");
        m.insert('ơ', "\u{00AC}");
        m.insert('ư', "\u{00AD}");
        m.insert('á', "\u{00B8}");
        m.insert('à', "\u{00B5}");
        m.insert('ả', "\u{00B6}");
        m.insert('ã', "\u{00B7}");
        m.insert('ạ', "\u{00B9}");
        m.insert('ấ', "\u{00CA}");
        m.insert('ầ', "\u{00C7}");
        m.insert('ẩ', "\u{00C8}");
        m.insert('ẫ', "\u{00C9}");
        m.insert('ậ', "\u{00CB}");
        m.insert('ắ', "\u{00BE}");
        m.insert('ằ', "\u{00BB}");
        m.insert('ẳ', "\u{00BC}");
        m.insert('ẵ', "\u{00BD}");
        m.insert('ặ', "\u{00C6}");
        m.insert('é', "\u{00D0}");
        m.insert('è', "\u{00CC}");
        m.insert('ẻ', "\u{00CE}");
        m.insert('ẽ', "\u{00CF}");
        m.insert('ẹ', "\u{00D1}");
        m.insert('ế', "\u{00D5}");
        m.insert('ề', "\u{00D2}");
        m.insert('ể', "\u{00D3}");
        m.insert('ễ', "\u{00D4}");
        m.insert('ệ', "\u{00D6}");
        m.insert('í', "\u{00DD}");
        m.insert('ì', "\u{00D7}");
        m.insert('ỉ', "\u{00D8}");
        m.insert('ĩ', "\u{00DC}");
        m.insert('ị', "\u{00DE}");
        m.insert('ó', "\u{00E3}");
        m.insert('ò', "\u{00DF}");
        m.insert('ỏ', "\u{00E1}");
        m.insert('õ', "\u{00E2}");
        m.insert('ọ', "\u{00E4}");
        m.insert('ố', "\u{00E8}");
        m.insert('ồ', "\u{00E5}");
        m.insert('ổ', "\u{00E6}");
        m.insert('ỗ', "\u{00E7}");
        m.insert('ộ', "\u{00E9}");
        m.insert('ớ', "\u{00ED}");
        m.insert('ờ', "\u{00EA}");
        m.insert('ở', "\u{00EB}");
        m.insert('ỡ', "\u{00EC}");
        m.insert('ợ', "\u{00EE}");
        m.insert('ú', "\u{00F3}");
        m.insert('ù', "\u{00EF}");
        m.insert('ủ', "\u{00F1}");
        m.insert('ũ', "\u{00F2}");
        m.insert('ụ', "\u{00F4}");
        m.insert('ứ', "\u{00F8}");
        m.insert('ừ', "\u{00F5}");
        m.insert('ử', "\u{00F6}");
        m.insert('ữ', "\u{00F7}");
        m.insert('ự', "\u{00F9}");
        m.insert('ý', "\u{00FD}");
        m.insert('ỳ', "\u{00FA}");
        m.insert('ỷ', "\u{00FB}");
        m.insert('ỹ', "\u{00FC}");
        m.insert('ỵ', "\u{00FE}");
        // Uppercase
        m.insert('Đ', "\u{00A7}");
        m.insert('Â', "\u{00A2}");
        m.insert('Ă', "\u{00A1}");
        m.insert('Ê', "\u{00A3}");
        m.insert('Ô', "\u{00A4}");
        m.insert('Ơ', "\u{00A5}");
        m.insert('Ư', "\u{00A6}");
        m.insert('Á', "\u{00B8}");
        m.insert('À', "\u{00B5}");
        m.insert('Ả', "\u{00B6}");
        m.insert('Ã', "\u{00B7}");
        m.insert('Ạ', "\u{00B9}");
        m.insert('Ấ', "\u{00CA}");
        m.insert('Ầ', "\u{00C7}");
        m.insert('Ẩ', "\u{00C8}");
        m.insert('Ẫ', "\u{00C9}");
        m.insert('Ậ', "\u{00CB}");
        m.insert('Ắ', "\u{00BE}");
        m.insert('Ằ', "\u{00BB}");
        m.insert('Ẳ', "\u{00BC}");
        m.insert('Ẵ', "\u{00BD}");
        m.insert('Ặ', "\u{00C6}");
        m.insert('É', "\u{00D0}");
        m.insert('È', "\u{00CC}");
        m.insert('Ẻ', "\u{00CE}");
        m.insert('Ẽ', "\u{00CF}");
        m.insert('Ẹ', "\u{00D1}");
        m.insert('Ế', "\u{00D5}");
        m.insert('Ề', "\u{00D2}");
        m.insert('Ể', "\u{00D3}");
        m.insert('Ễ', "\u{00D4}");
        m.insert('Ệ', "\u{00D6}");
        m.insert('Í', "\u{00DD}");
        m.insert('Ì', "\u{00D7}");
        m.insert('Ỉ', "\u{00D8}");
        m.insert('Ĩ', "\u{00DC}");
        m.insert('Ị', "\u{00DE}");
        m.insert('Ó', "\u{00E3}");
        m.insert('Ò', "\u{00DF}");
        m.insert('Ỏ', "\u{00E1}");
        m.insert('Õ', "\u{00E2}");
        m.insert('Ọ', "\u{00E4}");
        m.insert('Ố', "\u{00E8}");
        m.insert('Ồ', "\u{00E5}");
        m.insert('Ổ', "\u{00E6}");
        m.insert('Ỗ', "\u{00E7}");
        m.insert('Ộ', "\u{00E9}");
        m.insert('Ớ', "\u{00ED}");
        m.insert('Ờ', "\u{00EA}");
        m.insert('Ở', "\u{00EB}");
        m.insert('Ỡ', "\u{00EC}");
        m.insert('Ợ', "\u{00EE}");
        m.insert('Ú', "\u{00F3}");
        m.insert('Ù', "\u{00EF}");
        m.insert('Ủ', "\u{00F1}");
        m.insert('Ũ', "\u{00F2}");
        m.insert('Ụ', "\u{00F4}");
        m.insert('Ứ', "\u{00F8}");
        m.insert('Ừ', "\u{00F5}");
        m.insert('Ử', "\u{00F6}");
        m.insert('Ữ', "\u{00F7}");
        m.insert('Ự', "\u{00F9}");
        m.insert('Ý', "\u{00FD}");
        m.insert('Ỳ', "\u{00FA}");
        m.insert('Ỷ', "\u{00FB}");
        m.insert('Ỹ', "\u{00FC}");
        m.insert('Ỵ', "\u{00FE}");
        m
    };

    /// VNI Windows charset mapping from Unicode characters to their VNI encoded strings.
    static ref VNI_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        // Lowercase
        m.insert('đ', "\u{00F1}");
        m.insert('â', "a\u{00E2}");
        m.insert('ă', "a\u{00EA}");
        m.insert('ê', "e\u{00E2}");
        m.insert('ô', "o\u{00E2}");
        m.insert('ơ', "\u{00F4}");
        m.insert('ư', "\u{00F6}");
        m.insert('á', "a\u{00F9}");
        m.insert('à', "a\u{00F8}");
        m.insert('ả', "a\u{00FB}");
        m.insert('ã', "a\u{00F5}");
        m.insert('ạ', "a\u{00EF}");
        m.insert('ấ', "a\u{00E1}");
        m.insert('ầ', "a\u{00E0}");
        m.insert('ẩ', "a\u{00E5}");
        m.insert('ẫ', "a\u{00E3}");
        m.insert('ậ', "a\u{00E4}");
        m.insert('ắ', "a\u{00E9}");
        m.insert('ằ', "a\u{00E8}");
        m.insert('ẳ', "a\u{00FA}");
        m.insert('ẵ', "a\u{00FC}");
        m.insert('ặ', "a\u{00EB}");
        m.insert('é', "e\u{00F9}");
        m.insert('è', "e\u{00F8}");
        m.insert('ẻ', "e\u{00FB}");
        m.insert('ẽ', "e\u{00F5}");
        m.insert('ẹ', "e\u{00EF}");
        m.insert('ế', "e\u{00E1}");
        m.insert('ề', "e\u{00E0}");
        m.insert('ể', "e\u{00E5}");
        m.insert('ễ', "e\u{00E3}");
        m.insert('ệ', "e\u{00E4}");
        m.insert('í', "\u{00ED}");
        m.insert('ì', "\u{00EC}");
        m.insert('ỉ', "\u{00E6}");
        m.insert('ĩ', "\u{00F3}");
        m.insert('ị', "\u{00F2}");
        m.insert('ó', "o\u{00F9}");
        m.insert('ò', "o\u{00F8}");
        m.insert('ỏ', "o\u{00FB}");
        m.insert('õ', "o\u{00F5}");
        m.insert('ọ', "o\u{00EF}");
        m.insert('ố', "o\u{00E1}");
        m.insert('ồ', "o\u{00E0}");
        m.insert('ổ', "o\u{00E5}");
        m.insert('ỗ', "o\u{00E3}");
        m.insert('ộ', "o\u{00E4}");
        m.insert('ớ', "\u{00F4}\u{00F9}");
        m.insert('ờ', "\u{00F4}\u{00F8}");
        m.insert('ở', "\u{00F4}\u{00FB}");
        m.insert('ỡ', "\u{00F4}\u{00F5}");
        m.insert('ợ', "\u{00F4}\u{00EF}");
        m.insert('ú', "u\u{00F9}");
        m.insert('ù', "u\u{00F8}");
        m.insert('ủ', "u\u{00FB}");
        m.insert('ũ', "u\u{00F5}");
        m.insert('ụ', "u\u{00EF}");
        m.insert('ứ', "\u{00F6}\u{00F9}");
        m.insert('ừ', "\u{00F6}\u{00F8}");
        m.insert('ử', "\u{00F6}\u{00FB}");
        m.insert('ữ', "\u{00F6}\u{00F5}");
        m.insert('ự', "\u{00F6}\u{00EF}");
        m.insert('ý', "y\u{00F9}");
        m.insert('ỳ', "y\u{00F8}");
        m.insert('ỷ', "y\u{00FB}");
        m.insert('ỹ', "y\u{00F5}");
        m.insert('ỵ', "\u{00EE}");
        // Uppercase
        m.insert('Đ', "\u{00D1}");
        m.insert('Â', "A\u{00C2}");
        m.insert('Ă', "A\u{00CA}");
        m.insert('Ê', "E\u{00C2}");
        m.insert('Ô', "O\u{00C2}");
        m.insert('Ơ', "\u{00D4}");
        m.insert('Ư', "\u{00D6}");
        m.insert('Á', "A\u{00D9}");
        m.insert('À', "A\u{00D8}");
        m.insert('Ả', "A\u{00DB}");
        m.insert('Ã', "A\u{00D5}");
        m.insert('Ạ', "A\u{00CF}");
        m.insert('Ấ', "A\u{00C1}");
        m.insert('Ầ', "A\u{00C0}");
        m.insert('Ẩ', "A\u{00C5}");
        m.insert('Ẫ', "A\u{00C3}");
        m.insert('Ậ', "A\u{00C4}");
        m.insert('Ắ', "A\u{00C9}");
        m.insert('Ằ', "A\u{00C8}");
        m.insert('Ẳ', "A\u{00DA}");
        m.insert('Ẵ', "A\u{00DC}");
        m.insert('Ặ', "A\u{00CB}");
        m.insert('É', "E\u{00D9}");
        m.insert('È', "E\u{00D8}");
        m.insert('Ẻ', "E\u{00DB}");
        m.insert('Ẽ', "E\u{00D5}");
        m.insert('Ẹ', "E\u{00CF}");
        m.insert('Ế', "E\u{00C1}");
        m.insert('Ề', "E\u{00C0}");
        m.insert('Ể', "E\u{00C5}");
        m.insert('Ễ', "E\u{00C3}");
        m.insert('Ệ', "E\u{00C4}");
        m.insert('Í', "\u{00CD}");
        m.insert('Ì', "\u{00CC}");
        m.insert('Ỉ', "\u{00C6}");
        m.insert('Ĩ', "\u{00D3}");
        m.insert('Ị', "\u{00D2}");
        m.insert('Ó', "O\u{00D9}");
        m.insert('Ò', "O\u{00D8}");
        m.insert('Ỏ', "O\u{00DB}");
        m.insert('Õ', "O\u{00D5}");
        m.insert('Ọ', "O\u{00CF}");
        m.insert('Ố', "O\u{00C1}");
        m.insert('Ồ', "O\u{00C0}");
        m.insert('Ổ', "O\u{00C5}");
        m.insert('Ỗ', "O\u{00C3}");
        m.insert('Ộ', "O\u{00C4}");
        m.insert('Ớ', "\u{00D4}\u{00D9}");
        m.insert('Ờ', "\u{00D4}\u{00D8}");
        m.insert('Ở', "\u{00D4}\u{00DB}");
        m.insert('Ỡ', "\u{00D4}\u{00D5}");
        m.insert('Ợ', "\u{00D4}\u{00CF}");
        m.insert('Ú', "U\u{00D9}");
        m.insert('Ù', "U\u{00D8}");
        m.insert('Ủ', "U\u{00DB}");
        m.insert('Ũ', "U\u{00D5}");
        m.insert('Ụ', "U\u{00CF}");
        m.insert('Ứ', "\u{00D6}\u{00D9}");
        m.insert('Ừ', "\u{00D6}\u{00D8}");
        m.insert('Ử', "\u{00D6}\u{00DB}");
        m.insert('Ữ', "\u{00D6}\u{00D5}");
        m.insert('Ự', "\u{00D6}\u{00CF}");
        m.insert('Ý', "Y\u{00D9}");
        m.insert('Ỳ', "Y\u{00D8}");
        m.insert('Ỷ', "Y\u{00DB}");
        m.insert('Ỹ', "Y\u{00D5}");
        m.insert('Ỵ', "\u{00CE}");
        m
    };

    /// VIQR charset mapping from Unicode characters to their VIQR ASCII-based strings.
    static ref VIQR_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        // Lowercase
        m.insert('đ', "dd");
        m.insert('â', "a^");
        m.insert('ă', "a(");
        m.insert('ê', "e^");
        m.insert('ô', "o^");
        m.insert('ơ', "o+");
        m.insert('ư', "u+");
        m.insert('á', "a'");
        m.insert('à', "a`");
        m.insert('ả', "a?");
        m.insert('ã', "a~");
        m.insert('ạ', "a.");
        m.insert('ấ', "a^'");
        m.insert('ầ', "a^`");
        m.insert('ẩ', "a^?");
        m.insert('ẫ', "a^~");
        m.insert('ậ', "a^.");
        m.insert('ắ', "a('");
        m.insert('ằ', "a(`");
        m.insert('ẳ', "a(?");
        m.insert('ẵ', "a(~");
        m.insert('ặ', "a(.");
        m.insert('é', "e'");
        m.insert('è', "e`");
        m.insert('ẻ', "e?");
        m.insert('ẽ', "e~");
        m.insert('ẹ', "e.");
        m.insert('ế', "e^'");
        m.insert('ề', "e^`");
        m.insert('ể', "e^?");
        m.insert('ễ', "e^~");
        m.insert('ệ', "e^.");
        m.insert('í', "i'");
        m.insert('ì', "i`");
        m.insert('ỉ', "i?");
        m.insert('ĩ', "i~");
        m.insert('ị', "i.");
        m.insert('ó', "o'");
        m.insert('ò', "o`");
        m.insert('ỏ', "o?");
        m.insert('õ', "o~");
        m.insert('ọ', "o.");
        m.insert('ố', "o^'");
        m.insert('ồ', "o^`");
        m.insert('ổ', "o^?");
        m.insert('ỗ', "o^~");
        m.insert('ộ', "o^.");
        m.insert('ớ', "o+'");
        m.insert('ờ', "o+`");
        m.insert('ở', "o+?");
        m.insert('ỡ', "o+~");
        m.insert('ợ', "o+.");
        m.insert('ú', "u'");
        m.insert('ù', "u`");
        m.insert('ủ', "u?");
        m.insert('ũ', "u~");
        m.insert('ụ', "u.");
        m.insert('ứ', "u+'");
        m.insert('ừ', "u+`");
        m.insert('ử', "u+?");
        m.insert('ữ', "u+~");
        m.insert('ự', "u+.");
        m.insert('ý', "y'");
        m.insert('ỳ', "y`");
        m.insert('ỷ', "y?");
        m.insert('ỹ', "y~");
        m.insert('ỵ', "y.");
        // Uppercase
        m.insert('Đ', "DD");
        m.insert('Â', "A^");
        m.insert('Ă', "A(");
        m.insert('Ê', "E^");
        m.insert('Ô', "O^");
        m.insert('Ơ', "O+");
        m.insert('Ư', "U+");
        m.insert('Á', "A'");
        m.insert('À', "A`");
        m.insert('Ả', "A?");
        m.insert('Ã', "A~");
        m.insert('Ạ', "A.");
        m.insert('Ấ', "A^'");
        m.insert('Ầ', "A^`");
        m.insert('Ẩ', "A^?");
        m.insert('Ẫ', "A^~");
        m.insert('Ậ', "A^.");
        m.insert('Ắ', "A('");
        m.insert('Ằ', "A(`");
        m.insert('Ẳ', "A(?");
        m.insert('Ẵ', "A(~");
        m.insert('Ặ', "A(.");
        m.insert('É', "E'");
        m.insert('È', "E`");
        m.insert('Ẻ', "E?");
        m.insert('Ẽ', "E~");
        m.insert('Ẹ', "E.");
        m.insert('Ế', "E^'");
        m.insert('Ề', "E^`");
        m.insert('Ể', "E^?");
        m.insert('Ễ', "E^~");
        m.insert('Ệ', "E^.");
        m.insert('Í', "I'");
        m.insert('Ì', "I`");
        m.insert('Ỉ', "I?");
        m.insert('Ĩ', "I~");
        m.insert('Ị', "I.");
        m.insert('Ó', "O'");
        m.insert('Ò', "O`");
        m.insert('Ỏ', "O?");
        m.insert('Õ', "O~");
        m.insert('Ọ', "O.");
        m.insert('Ố', "O^'");
        m.insert('Ồ', "O^`");
        m.insert('Ổ', "O^?");
        m.insert('Ỗ', "O^~");
        m.insert('Ộ', "O^.");
        m.insert('Ớ', "O+'");
        m.insert('Ờ', "O+`");
        m.insert('Ở', "O+?");
        m.insert('Ỡ', "O+~");
        m.insert('Ợ', "O+.");
        m.insert('Ú', "U'");
        m.insert('Ù', "U`");
        m.insert('Ủ', "U?");
        m.insert('Ũ', "U~");
        m.insert('Ụ', "U.");
        m.insert('Ứ', "U+'");
        m.insert('Ừ', "U+`");
        m.insert('Ử', "U+?");
        m.insert('Ữ', "U+~");
        m.insert('Ự', "U+.");
        m.insert('Ý', "Y'");
        m.insert('Ỳ', "Y`");
        m.insert('Ỷ', "Y?");
        m.insert('Ỹ', "Y~");
        m.insert('Ỵ', "Y.");
        m
    };
}

/// Encodes a Unicode Vietnamese string to the specified charset encoding.
///
/// For `CharsetEncoding::Unicode`, the input string is returned as-is (identity operation).
/// For other encodings, each Vietnamese character is looked up in the corresponding mapping table
/// and replaced with its encoded representation. Non-Vietnamese characters (ASCII, digits,
/// punctuation) are passed through unchanged.
///
/// # Arguments
/// * `charset` - The target charset encoding.
/// * `input` - The Unicode input string to encode.
///
/// # Returns
/// The encoded string in the target charset format.
pub fn encode(charset: &CharsetEncoding, input: &str) -> String {
    match charset {
        CharsetEncoding::Unicode => input.to_string(),
        CharsetEncoding::Tcvn3 => encode_with_map(input, &TCVN3_MAP),
        CharsetEncoding::VniWindows => encode_with_map(input, &VNI_MAP),
        CharsetEncoding::Viqr => encode_with_map(input, &VIQR_MAP),
    }
}

/// Internal helper that encodes a Unicode string using the provided character mapping table.
fn encode_with_map(input: &str, map: &HashMap<char, &str>) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        if let Some(&encoded) = map.get(&c) {
            output.push_str(encoded);
        } else {
            output.push(c);
        }
    }
    output
}
