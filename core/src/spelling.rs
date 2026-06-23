//! Vietnamese Spelling Rule Engine.
//!
//! This module implements grammatical/spelling checks for Vietnamese words.
//! It splits words into (First Consonant, Vowel, Last Consonant) components
//! and validates their combinations using transition/validity matrices.

use crate::charset::add_mark_to_toneless_char;

lazy_static::lazy_static! {
    /// Categorized sequences of initial consonants in Vietnamese spelling.
    pub static ref FIRST_CONSONANT_SEQS: Vec<&'static str> = vec![
        "b d đ g gh m n nh p ph r s t tr v z",
        "c h k kh qu th",
        "ch gi l ng ngh x",
        "đ l",
        "h",
    ];

    /// Categorized sequences of vowels in Vietnamese spelling.
    pub static ref VOWEL_SEQS: Vec<&'static str> = vec![
        "ê i ua uê uy y",
        "a iê oa uyê yê",
        "â ă e o oo ô ơ oe u ư uâ uô ươ",
        "oă",
        "uơ",
        "ai ao au âu ay ây eo êu ia iêu iu oai oao oay oeo oi ôi ơi ưa uây ui ưi uôi ươi ươu ưu uya uyu yêu",
        "ă",
        "i",
    ];

    /// Categorized sequences of final consonants in Vietnamese spelling.
    pub static ref LAST_CONSONANT_SEQS: Vec<&'static str> = vec![
        "ch nh",
        "c ng",
        "m n p t",
        "k",
        "c",
    ];

    /// Matrix defining valid transitions between first consonants and vowels.
    /// Indexes map to rows in `FIRST_CONSONANT_SEQS` and `VOWEL_SEQS`.
    pub static ref CV_MATRIX: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 5],
        vec![0, 1, 2, 3, 4, 5],
        vec![0, 1, 2, 3, 5],
        vec![6],
        vec![7],
    ];

    /// Matrix defining valid transitions between vowels and final consonants.
    /// Indexes map to rows in `VOWEL_SEQS` and `LAST_CONSONANT_SEQS`.
    pub static ref VC_MATRIX: Vec<Vec<usize>> = vec![
        vec![0, 2],
        vec![0, 1, 2],
        vec![1, 2],
        vec![1, 2],
        vec![],
        vec![],
        vec![3],
        vec![4],
    ];
}

/// Performs a lookup of an input segment in spelling sequence rules.
///
/// * `seq` - The spelling reference sequences (consonants or vowels).
/// * `input` - The input string segment to look up.
/// * `input_is_full` - If true, requires the candidate sequence word to match the input length exactly.
/// * `input_is_complete` - If true, checks characters with exact matches; if false, matches base/toneless equivalents.
pub fn lookup(
    seq: &[&str],
    input: &str,
    input_is_full: bool,
    input_is_complete: bool,
) -> Vec<usize> {
    let mut ret = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();
    let input_len = input_chars.len();

    for (index, &row) in seq.iter().enumerate() {
        for word in row.split_whitespace() {
            let canvas: Vec<char> = word.chars().collect();
            if canvas.len() < input_len || (input_is_full && canvas.len() > input_len) {
                continue;
            }
            let mut is_match = true;
            for (k, &ic) in input_chars.iter().enumerate() {
                let canvas_char = canvas[k];
                if ic != canvas_char
                    && (input_is_complete || add_mark_to_toneless_char(canvas_char, 0) != ic)
                {
                    is_match = false;
                    break;
                }
            }
            if is_match {
                ret.push(index);
                break;
            }
        }
    }
    ret
}

/// Validates whether a combination of first consonant, vowel, and last consonant is phonologically valid in Vietnamese.
///
/// * `fc` - The first/initial consonant segment.
/// * `vo` - The vowel/nucleus segment.
/// * `lc` - The last/final consonant segment.
/// * `input_is_full_complete` - Strict check toggle indicating whether the syllable components are fully complete.
pub fn is_valid_cvc(fc: &str, vo: &str, lc: &str, input_is_full_complete: bool) -> bool {
    let mut fc_indexes = None;
    if !fc.is_empty() {
        let lookup_res = lookup(
            &FIRST_CONSONANT_SEQS,
            fc,
            input_is_full_complete || !vo.is_empty(),
            true,
        );
        if lookup_res.is_empty() {
            return false;
        }
        fc_indexes = Some(lookup_res);
    }

    let mut vo_indexes = None;
    if !vo.is_empty() {
        let lookup_res = lookup(
            &VOWEL_SEQS,
            vo,
            input_is_full_complete || !lc.is_empty(),
            input_is_full_complete,
        );
        if lookup_res.is_empty() {
            return false;
        }
        vo_indexes = Some(lookup_res);
    }

    let mut lc_indexes = None;
    if !lc.is_empty() {
        let lookup_res = lookup(&LAST_CONSONANT_SEQS, lc, input_is_full_complete, true);
        if lookup_res.is_empty() {
            return false;
        }
        lc_indexes = Some(lookup_res);
    }

    let vo_indexes_val = match vo_indexes {
        None => return fc_indexes.is_some(),
        Some(v) => v,
    };

    if let Some(fc_idx) = fc_indexes {
        let cv_valid = is_valid_cv(&fc_idx, &vo_indexes_val);
        if !cv_valid || lc_indexes.is_none() {
            return cv_valid;
        }
    }

    if let Some(lc_idx) = lc_indexes {
        is_valid_vc(&vo_indexes_val, &lc_idx)
    } else {
        true
    }
}

/// Helper function to validate combination of first consonant indices and vowel indices.
fn is_valid_cv(fc_indexes: &[usize], vo_indexes: &[usize]) -> bool {
    for &fc in fc_indexes {
        if fc < CV_MATRIX.len() {
            for &c in &CV_MATRIX[fc] {
                for &vo in vo_indexes {
                    if c == vo {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Helper function to validate combination of vowel indices and last consonant indices.
fn is_valid_vc(vo_indexes: &[usize], lc_indexes: &[usize]) -> bool {
    for &vo in vo_indexes {
        if vo < VC_MATRIX.len() {
            for &c in &VC_MATRIX[vo] {
                for &lc in lc_indexes {
                    if c == lc {
                        return true;
                    }
                }
            }
        }
    }
    false
}
