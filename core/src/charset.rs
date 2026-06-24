//! Vietnamese Charset Definitions and Helper Functions.
//!
//! This module provides static mappings, collections, and utility functions for identifying
//! and manipulating Vietnamese characters, vowels, tones, diacritics (marks), and punctuation.

use std::collections::HashMap;

lazy_static::lazy_static! {
    /// A list of all Vietnamese vowels, including their variations with different tones and marks.
    /// Order matters here as it is used to determine and modify the tone of a vowel.
    pub static ref VOWELS: Vec<char> = "aàáảãạăằắẳẵặâầấẩẫậeèéẻẽẹêềếểễệiìíỉĩịoòóỏõọôồốổỗộơờớởỡợuùúủũụưừứửữựyỳýỷỹỵ"
        .chars()
        .collect();

    /// A list of common punctuation marks and symbols that do not form part of words.
    pub static ref PUNCTUATION_MARKS: Vec<char> = vec![
        ',', ';', ':', '.', '"', '\'', '!', '?', ' ',
        '<', '>', '=', '+', '-', '*', '/', '\\',
        '_', '~', '`', '@', '#', '$', '%', '^', '&', '(', ')', '{', '}', '[', ']',
        '|',
    ];

    /// Mapping of base characters to their corresponding mark families.
    /// Each entry maps a character to a 5-character string representation of its diacritic variations.
    pub static ref MARKS_MAPS: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('a', "aâă__");
        m.insert('â', "aâă__");
        m.insert('ă', "aâă__");
        m.insert('e', "eê___");
        m.insert('ê', "eê___");
        m.insert('o', "oô_ơ_");
        m.insert('ô', "oô_ơ_");
        m.insert('ơ', "oô_ơ_");
        m.insert('u', "u__ư_");
        m.insert('ư', "u__ư_");
        m.insert('d', "d___đ");
        m.insert('đ', "d___đ");
        m
    };
}

/// Checks if a key represents a space character.
pub fn is_space(key: char) -> bool {
    key == ' '
}

/// Checks if a character is a punctuation mark or standard symbol.
pub fn is_punctuation_mark(key: char) -> bool {
    PUNCTUATION_MARKS.contains(&key)
}

/// Checks if a character should act as a word break symbol.
///
/// Returns `true` if the character is a punctuation mark, a space, or a number.
pub fn is_word_break_symbol(key: char) -> bool {
    is_punctuation_mark(key) || (key >= '0' && key <= '9')
}

/// Checks if a character is a Vietnamese vowel (regardless of tone or diacritic).
pub fn is_vowel(chr: char) -> bool {
    VOWELS.contains(&chr)
}

/// Finds the index position of a vowel in the static `VOWELS` vector.
///
/// Returns `Some(index)` if found, or `None` otherwise.
pub fn find_vowel_position(chr: char) -> Option<usize> {
    VOWELS.iter().position(|&v| v == chr)
}

/// Returns a list of characters belonging to the mark family of the given character.
///
/// Useful for identifying variations of a character with different diacritics (e.g. `a` -> `['a', 'â', 'ă']`).
pub fn get_mark_family(chr: char) -> Vec<char> {
    if let Some(&s) = MARKS_MAPS.get(&chr) {
        s.chars().filter(|&c| c != '_').collect()
    } else {
        Vec::new()
    }
}

/// Finds the position/index of a character within its mark family mapping.
pub fn find_mark_position(chr: char) -> Option<usize> {
    if let Some(&s) = MARKS_MAPS.get(&chr) {
        s.chars().position(|v| v == chr)
    } else {
        None
    }
}

/// Adds a diacritic mark to a toneless character based on index position.
///
/// * `chr` - The base character.
/// * `mark` - The index of the mark in the character's mapping sequence.
pub fn add_mark_to_toneless_char(chr: char, mark: u8) -> char {
    if let Some(&s) = MARKS_MAPS.get(&chr) {
        let marks: Vec<char> = s.chars().collect();
        if (mark as usize) < marks.len() && marks[mark as usize] != '_' {
            return marks[mark as usize];
        }
    }
    chr
}

/// Adds a diacritic mark to a character while preserving its current tone.
///
/// * `chr` - The character to modify.
/// * `mark` - The index of the mark to apply.
pub fn add_mark_to_char(chr: char, mark: u8) -> char {
    let tone = find_tone_from_char(chr);
    let toneless_chr = add_tone_to_char(chr, 0);
    let marked_toneless = add_mark_to_toneless_char(toneless_chr, mark);
    add_tone_to_char(marked_toneless, tone)
}

/// Checks if a character is an ASCII alphabetic letter (a-z, A-Z).
pub fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// Finds the tone index (0 to 5) from a Vietnamese vowel.
///
/// Returns 0 for non-vowels or vowels with no tone (ToneNone).
pub fn find_tone_from_char(chr: char) -> u8 {
    if let Some(pos) = find_vowel_position(chr) {
        (pos % 6) as u8
    } else {
        0 // ToneNone
    }
}

/// Applies a tone index (0 to 5) to a Vietnamese vowel while keeping its base mark.
///
/// * `chr` - The vowel to modify.
/// * `tone` - The tone index (0 for normal/none, 1 for acute/sắc, 2 for grave/huyền, 3 for ask/hỏi, 4 for tilde/ngã, 5 for heavy/nặng).
pub fn add_tone_to_char(chr: char, tone: u8) -> char {
    if let Some(pos) = find_vowel_position(chr) {
        let current_tone = pos % 6;
        let offset = tone as isize - current_tone as isize;
        let new_pos = (pos as isize + offset) as usize;
        if new_pos < VOWELS.len() {
            return VOWELS[new_pos];
        }
    }
    chr
}

/// Determines whether a key/character can be processed by the typing engine.
///
/// Returns `true` if the key is alphabetic, is an active modification key, or is a Vietnamese rune.
pub fn can_process_key(lower_key: char, effect_keys: &[char]) -> bool {
    if is_alpha(lower_key) || effect_keys.contains(&lower_key) {
        return true;
    }
    if is_word_break_symbol(lower_key) {
        return false;
    }
    is_vietnamese_rune(lower_key)
}

/// Checks if a character is a Vietnamese rune (meaning it has a tone or a base Vietnamese mark).
pub fn is_vietnamese_rune(lower_key: char) -> bool {
    if find_tone_from_char(lower_key) != 0 {
        return true;
    }
    lower_key != add_mark_to_toneless_char(lower_key, 0)
}

/// Checks if a string contains any Vietnamese runes.
pub fn has_any_vietnamese_rune(word: &str) -> bool {
    word.chars()
        .any(|c| is_vietnamese_rune(c.to_lowercase().next().unwrap_or(c)))
}

/// Checks if a string contains any Vietnamese vowels.
pub fn has_any_vietnamese_vowel(word: &str) -> bool {
    word.chars()
        .any(|c| is_vowel(c.to_lowercase().next().unwrap_or(c)))
}
