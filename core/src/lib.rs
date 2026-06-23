pub mod charset;
pub mod engine;
pub mod rules;
pub mod spelling;

// Re-export key components for library users
pub use engine::{
    Engine, Transformation, ENGLISH_MODE, FULL_TEXT, IN_REVERSE_ORDER, LOWER_CASE, MARK_LESS,
    PUNCTUATION_MODE, TONE_LESS, VIETNAMESE_MODE, EFREE_TONE_MARKING, ESTD_TONE_STYLE,
    EAUTO_CORRECT_ENABLED, ESTD_FLAGS,
};
pub use rules::{get_input_method, EffectType, InputMethod, Mark, Rule, Tone};
pub use charset::{
    has_any_vietnamese_rune, has_any_vietnamese_vowel, is_vietnamese_rune, is_vowel,
};
