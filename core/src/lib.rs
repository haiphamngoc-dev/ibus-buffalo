pub mod charset;
pub mod engine;
pub mod rules;
pub mod spelling;

// Re-export key components for library users
pub use charset::{
    has_any_vietnamese_rune, has_any_vietnamese_vowel, is_vietnamese_rune, is_vowel,
    is_word_break_symbol,
};
pub use engine::{
    EAUTO_CORRECT_ENABLED, EFREE_TONE_MARKING, ENGLISH_MODE, ESTD_FLAGS, ESTD_TONE_STYLE, Engine,
    FULL_TEXT, IN_REVERSE_ORDER, LOWER_CASE, MARK_LESS, PUNCTUATION_MODE, TONE_LESS,
    Transformation, VIETNAMESE_MODE,
};
pub use rules::{EffectType, InputMethod, Mark, Rule, Tone, get_input_method};
