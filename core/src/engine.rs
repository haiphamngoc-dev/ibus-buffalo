//! Buffalo Core Vietnamese Typing Engine implementation.
//!
//! This module implements the main state machine and decision-making logic (`Engine`)
//! that processes keystrokes, builds up a composition of transformations,
//! checks spelling, handles tone and diacritic placement rules, and produces the finalized
//! output string.

use crate::charset::{
    add_mark_to_char, add_tone_to_char, can_process_key, find_tone_from_char, is_alpha, is_space,
    is_vowel,
};
use crate::rules::{EffectType, InputMethod, Mark, Rule, Tone};
use crate::spelling::is_valid_cvc;
use std::collections::HashMap;

// Modes

/// Flag indicating that Vietnamese typing mode is active.
pub const VIETNAMESE_MODE: u32 = 1 << 0;
/// Flag indicating that English typing mode is active (bypass Vietnamese processing).
pub const ENGLISH_MODE: u32 = 1 << 1;
/// Flag indicating that tone marks should be omitted from the output.
pub const TONE_LESS: u32 = 1 << 2;
/// Flag indicating that diacritic marks should be omitted from the output.
pub const MARK_LESS: u32 = 1 << 3;
/// Flag indicating that the processed string should be converted to lower case.
pub const LOWER_CASE: u32 = 1 << 4;
/// Flag indicating that the output should represent the full/entire text buffer.
pub const FULL_TEXT: u32 = 1 << 5;
/// Flag indicating that punctuation marks mode is active.
pub const PUNCTUATION_MODE: u32 = 1 << 6;
/// Flag indicating that keys are processed/inserted in reverse order.
pub const IN_REVERSE_ORDER: u32 = 1 << 7;

// Flags

/// Flag enabling modern/free tone marking placement (rules for placing tone mark on appropriate vowels).
pub const EFREE_TONE_MARKING: u32 = 1 << 0;
/// Flag enabling standard Vietnamese tone style (e.g. `hòa` vs `hoà`).
pub const ESTD_TONE_STYLE: u32 = 1 << 1;
/// Flag enabling automatic spell-correction checks on the input sequence.
pub const EAUTO_CORRECT_ENABLED: u32 = 1 << 2;
/// Standard typing flags preset (enables free tone marking, standard tone style, and auto-correct).
pub const ESTD_FLAGS: u32 = EFREE_TONE_MARKING | ESTD_TONE_STYLE | EAUTO_CORRECT_ENABLED;

/// Represents a single character transformation step in the engine's composition history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transformation {
    /// The rule applied for this transformation.
    pub rule: Rule,
    /// Absolute index in composition Vec that this transformation targets (if any).
    pub target: Option<usize>,
    /// Specifies if the resulting character should be upper case.
    pub is_upper_case: bool,
}

/// The core typing engine managing composition history, input method configuration, and settings.
pub struct Engine {
    /// History of all key transformations in the current composition buffer.
    pub composition: Vec<Transformation>,
    /// Configured input method rules and layouts (e.g. Telex, VNI).
    pub input_method: InputMethod,
    /// Configuration flags (e.g. `EFREE_TONE_MARKING`, `ESTD_TONE_STYLE`).
    pub flags: u32,
}

impl Engine {
    /// Creates a new `Engine` instance with the specified input method and configuration flags.
    pub fn new(input_method: InputMethod, flags: u32) -> Self {
        Self {
            composition: Vec::new(),
            input_method,
            flags,
        }
    }

    /// Resets the engine composition history, clearing the current buffer.
    pub fn reset(&mut self) {
        self.composition.clear();
    }

    /// Returns a reference to the active `InputMethod` layout.
    pub fn get_input_method(&self) -> &InputMethod {
        &self.input_method
    }

    /// Updates the engine configuration flags.
    pub fn set_flag(&mut self, flag: u32) {
        self.flags = flag;
    }

    /// Checks if a lowercase key is categorized as a "super key" (modifies multiple vowels simultaneously, e.g. Telex `w`).
    pub fn is_super_key(&self, lower_key: char) -> bool {
        self.input_method.super_keys.contains(&lower_key)
    }

    /// Checks if a key is a designated tone modification key in the current layout.
    pub fn is_tone_key(&self, key: char) -> bool {
        self.input_method.tone_keys.contains(&key)
    }

    /// Checks if a key triggers any rule within the current input method layout.
    pub fn is_effective_key(&self, key: char) -> bool {
        self.input_method.keys.contains(&key)
    }

    /// Validates if the current spelling of the last word in composition is correct.
    ///
    /// * `input_is_full_complete` - Strict check toggle passed to spelling rule validator.
    pub fn is_valid(&self, input_is_full_complete: bool) -> bool {
        let last_word_start = extract_last_word_start(&self.composition, &self.input_method.keys);
        is_valid_slice(&self.composition, last_word_start, input_is_full_complete)
    }

    /// Flattens the composition buffer and returns the processed output string based on the given mode.
    pub fn get_processed_string(&self, mode: u32) -> String {
        if (mode & FULL_TEXT) != 0 {
            flatten(&self.composition, mode)
        } else if (mode & PUNCTUATION_MODE) != 0 {
            let start = extract_last_word_with_punctuation_marks_start(
                &self.composition,
                &self.input_method.keys,
            );
            flatten(&self.composition[start..], mode & !PUNCTUATION_MODE)
        } else {
            let start = extract_last_word_start(&self.composition, &self.input_method.keys);
            flatten(&self.composition[start..], mode)
        }
    }

    /// Checks if the engine can process the given key under the active input method configuration.
    pub fn can_process_key(&self, key: char) -> bool {
        can_process_key(key, &self.input_method.keys)
    }

    /// Processes a single keystroke, applying rules and generating new transformations.
    ///
    /// * `key` - The incoming key character.
    /// * `mode` - The active mode flags.
    pub fn process_key(&mut self, key: char, mode: u32) {
        let lower_key = key.to_ascii_lowercase();
        let is_upper_case = key.is_ascii_uppercase();

        if (mode & ENGLISH_MODE) != 0 || !self.can_process_key(lower_key) {
            let trans = new_appending_trans(lower_key, is_upper_case);
            if (mode & IN_REVERSE_ORDER) != 0 {
                self.composition.insert(0, trans);
            } else {
                self.composition.push(trans);
            }
            return;
        }

        // Just process the key stroke on the last syllable
        let last_syllable_start =
            extract_last_syllable_start(&self.composition, &self.input_method.keys);

        let applicable_rules = self.get_applicable_rules(lower_key);
        let transformations = self.generate_transformations(
            last_syllable_start,
            applicable_rules,
            lower_key,
            is_upper_case,
        );

        self.composition.extend(transformations);
    }

    /// Removes the last character from the composition buffer, correcting tone placement if requested.
    ///
    /// * `refresh_last_tone` - If true, re-calculates the correct tone placement on the remaining vowels.
    pub fn remove_last_char(&mut self, refresh_last_tone: bool) {
        let last_appending_idx = find_last_appending_trans_idx(&self.composition);
        if last_appending_idx.is_none() {
            return;
        }
        let last_appending_idx = last_appending_idx.unwrap();
        let last_appending = &self.composition[last_appending_idx];

        if !self.can_process_key(last_appending.rule.key) {
            self.composition.pop();
            return;
        }

        let start = extract_last_word_start(&self.composition, &self.input_method.keys);

        // Retain only those transformations in the last word that do not target or match the last appending
        let mut new_composition = self.composition[..start].to_vec();
        for i in start..self.composition.len() {
            let t = &self.composition[i];
            if t.target == Some(last_appending_idx) || i == last_appending_idx {
                continue;
            }
            new_composition.push(t.clone());
        }

        if refresh_last_tone {
            let tone_trans = refresh_last_tone_target(
                &new_composition,
                start,
                (self.flags & ESTD_TONE_STYLE) != 0,
            );
            new_composition.extend(tone_trans);
        }

        self.composition = new_composition;
    }

    /// Restores or converts the last word in the composition history to/from raw text format.
    ///
    /// * `to_vietnamese` - If true, processes raw keys again with Vietnamese typing rules.
    pub fn restore_last_word(&mut self, to_vietnamese: bool) {
        let start = extract_last_word_start(&self.composition, &self.input_method.keys);
        if start == self.composition.len() {
            return;
        }

        let last_comb = self.composition[start..].to_vec();
        self.composition.truncate(start);

        if !to_vietnamese {
            for trans in last_comb {
                if trans.rule.key != '\0' {
                    self.composition
                        .push(new_appending_trans(trans.rule.key, trans.is_upper_case));
                }
            }
        } else {
            for trans in last_comb {
                self.process_key(trans.rule.key, VIETNAMESE_MODE);
            }
        }
    }

    /// Helper to retrieve rules matching the given trigger key.
    fn get_applicable_rules(&self, key: char) -> Vec<Rule> {
        self.input_method
            .rules
            .iter()
            .filter(|r| r.key == key)
            .cloned()
            .collect()
    }

    /// Computes and generates transformations needed to apply a key stroke to the current syllable.
    fn generate_transformations(
        &self,
        start: usize,
        applicable_rules: Vec<Rule>,
        lower_key: char,
        is_upper_case: bool,
    ) -> Vec<Transformation> {
        let mut transformations = Vec::new();

        // Double typing an effect key undoes it
        if self.composition.len() > start {
            let last_trans_idx = self.composition.len() - 1;
            let rule = &self.composition[last_trans_idx].rule;
            if rule.effect_type == EffectType::Appending
                && rule.key == lower_key
                && rule.key != rule.result
            {
                transformations.push(Transformation {
                    rule: Rule {
                        key: '\0',
                        effect: Mark::Raw as u8,
                        effect_type: EffectType::MarkTransformation,
                        effect_on: '\0',
                        result: '\0',
                        appended_rules: Vec::new(),
                    },
                    target: Some(last_trans_idx),
                    is_upper_case,
                });
                return transformations;
            }
        }

        let (target, applicable_rule) = self.find_target(start, &applicable_rules);
        if let (Some(tgt_idx), Some(rule)) = (target, applicable_rule) {
            transformations.push(Transformation {
                rule: rule.clone(),
                target: Some(tgt_idx),
                is_upper_case,
            });

            if rule.effect_type != EffectType::MarkTransformation {
                return transformations;
            }

            let mut temp_composition = self.composition.clone();
            temp_composition.extend(transformations.clone());

            if is_valid_slice(&temp_composition, start, true) {
                return transformations;
            }

            // Implement uow shortcut
            let (target_v, virtual_rule) =
                self.find_target_in_slice(&temp_composition, start, &applicable_rules);
            if let (Some(tgt_v_idx), Some(mut v_rule)) = (target_v, virtual_rule) {
                v_rule.key = '\0';
                transformations.push(Transformation {
                    rule: v_rule,
                    target: Some(tgt_v_idx),
                    is_upper_case: false,
                });
                return transformations;
            }
        } else {
            // Implement ươ/ưo + o -> uô
            let lower_flat = flatten(
                &self.composition[start..],
                VIETNAMESE_MODE | TONE_LESS | LOWER_CASE,
            );
            let re_uh_o = regex::Regex::new(r"(ưo|ươ)").unwrap();
            if re_uh_o.is_match(&lower_flat) {
                let vowels = filter_appending_composition(
                    &self.composition,
                    get_right_most_vowels_start(&self.composition, start),
                );
                if !vowels.is_empty() {
                    let trans = Transformation {
                        target: Some(vowels[0]),
                        rule: Rule {
                            key: '\0',
                            effect_type: EffectType::MarkTransformation,
                            effect: Mark::None as u8,
                            effect_on: '\0',
                            result: '\0',
                            appended_rules: Vec::new(),
                        },
                        is_upper_case: false,
                    };

                    let mut temp_composition = self.composition.clone();
                    temp_composition.push(trans.clone());

                    let (target_alt, rule_alt) =
                        self.find_target_in_slice(&temp_composition, start, &applicable_rules);
                    if let (Some(tgt_alt_idx), Some(r_alt)) = (target_alt, rule_alt) {
                        if tgt_alt_idx != vowels[0] {
                            transformations.push(trans);
                            transformations.push(Transformation {
                                rule: r_alt,
                                target: Some(tgt_alt_idx),
                                is_upper_case,
                            });
                            return transformations;
                        }
                    }
                }
            }

            let undo_trans = self.generate_undo_transformations(start, &applicable_rules);
            if !undo_trans.is_empty() {
                transformations.extend(undo_trans);
                transformations.push(new_appending_trans(lower_key, is_upper_case));
            }
        }

        if transformations.is_empty() {
            // Fallback transformations
            transformations.extend(self.generate_fallback_transformations(
                &applicable_rules,
                lower_key,
                is_upper_case,
            ));

            let mut temp_composition = self.composition.clone();
            temp_composition.extend(transformations.clone());

            if let Some(virtual_trans) = self.apply_uow_shortcut(&temp_composition, start) {
                transformations.push(virtual_trans);
            }
        }

        // Adjust tone placement if necessary
        let mut temp_composition = self.composition.clone();
        temp_composition.extend(transformations.clone());
        let tone_adjust = refresh_last_tone_target(
            &temp_composition,
            start,
            (self.flags & ESTD_TONE_STYLE) != 0,
        );
        transformations.extend(tone_adjust);

        transformations
    }

    /// Finds the index and applicable rule mapping to the active target in composition.
    fn find_target(
        &self,
        start: usize,
        applicable_rules: &[Rule],
    ) -> (Option<usize>, Option<Rule>) {
        self.find_target_in_slice(&self.composition, start, applicable_rules)
    }

    /// Scans a specific slice of composition history for rule application target.
    fn find_target_in_slice(
        &self,
        composition: &[Transformation],
        start: usize,
        applicable_rules: &[Rule],
    ) -> (Option<usize>, Option<Rule>) {
        let str_flat = flatten(&composition[start..], VIETNAMESE_MODE);

        for rule in applicable_rules {
            if rule.effect_type != EffectType::ToneTransformation {
                continue;
            }

            let mut target = None;
            if (self.flags & EFREE_TONE_MARKING) != 0 {
                if has_valid_tone(composition, start, Tone::from_u8(rule.effect)) {
                    target =
                        find_tone_target(composition, start, (self.flags & ESTD_TONE_STYLE) != 0);
                }
            } else if let Some(last_appending) =
                find_last_appending_trans_idx(&composition[start..])
            {
                let abs_idx = start + last_appending;
                if is_vowel(composition[abs_idx].rule.effect_on) {
                    target = Some(abs_idx);
                }
            }

            if let Some(tgt_idx) = target {
                let mut temp = composition.to_vec();
                temp.push(Transformation {
                    target: Some(tgt_idx),
                    rule: rule.clone(),
                    is_upper_case: false,
                });
                if str_flat == flatten(&temp[start..], VIETNAMESE_MODE) {
                    continue;
                }
                if Tone::from_u8(rule.effect) == Tone::None
                    && is_free(composition, start, tgt_idx, EffectType::ToneTransformation)
                    && find_tone_from_char(composition[tgt_idx].rule.result) == 0
                {
                    target = None;
                }
            }

            return (target, Some(rule.clone()));
        }

        self.find_mark_target(composition, start, applicable_rules)
    }

    /// Scans composition history for target matching a diacritic mark rule.
    fn find_mark_target(
        &self,
        composition: &[Transformation],
        start: usize,
        applicable_rules: &[Rule],
    ) -> (Option<usize>, Option<Rule>) {
        let str_flat = flatten(&composition[start..], VIETNAMESE_MODE);

        for i in (start..composition.len()).rev() {
            let trans = &composition[i];
            for rule in applicable_rules {
                if rule.effect_type != EffectType::MarkTransformation {
                    continue;
                }

                if trans.rule.result == rule.effect_on && rule.effect > 0 {
                    let target = find_root_target(composition, i);
                    let mut temp = composition.to_vec();
                    temp.push(Transformation {
                        target: Some(target),
                        rule: rule.clone(),
                        is_upper_case: false,
                    });
                    if str_flat == flatten(&temp[start..], VIETNAMESE_MODE) {
                        continue;
                    }

                    if is_valid_slice(&temp, start, false) {
                        return (Some(target), Some(rule.clone()));
                    }
                }
            }
        }
        (None, None)
    }

    /// Generates transformations necessary to undo tone or diacritic changes if typing layout triggers it.
    fn generate_undo_transformations(
        &self,
        start: usize,
        applicable_rules: &[Rule],
    ) -> Vec<Transformation> {
        let mut transformations = Vec::new();
        let str_flat = flatten(
            &self.composition[start..],
            VIETNAMESE_MODE | TONE_LESS | LOWER_CASE,
        );

        for rule in applicable_rules {
            if rule.effect_type == EffectType::ToneTransformation {
                let mut target = None;
                if (self.flags & EFREE_TONE_MARKING) != 0 {
                    if has_valid_tone(&self.composition, start, Tone::from_u8(rule.effect)) {
                        target = find_tone_target(
                            &self.composition,
                            start,
                            (self.flags & ESTD_TONE_STYLE) != 0,
                        );
                    }
                } else if let Some(last_appending) =
                    find_last_appending_trans_idx(&self.composition[start..])
                {
                    let abs_idx = start + last_appending;
                    if is_vowel(self.composition[abs_idx].rule.effect_on) {
                        target = Some(abs_idx);
                    }
                }

                if let Some(tgt_idx) = target {
                    transformations.push(Transformation {
                        target: Some(tgt_idx),
                        rule: Rule {
                            key: '\0',
                            effect: 0,
                            effect_type: EffectType::ToneTransformation,
                            effect_on: '\0',
                            result: '\0',
                            appended_rules: Vec::new(),
                        },
                        is_upper_case: false,
                    });
                }
            } else if rule.effect_type == EffectType::MarkTransformation {
                for i in (start..self.composition.len()).rev() {
                    let trans = &self.composition[i];
                    if trans.rule.result == rule.effect_on {
                        let target = find_root_target(&self.composition, i);
                        let undo = Transformation {
                            target: Some(target),
                            rule: Rule {
                                key: '\0',
                                effect: 0,
                                effect_type: EffectType::MarkTransformation,
                                effect_on: '\0',
                                result: '\0',
                                appended_rules: Vec::new(),
                            },
                            is_upper_case: false,
                        };

                        let mut temp = self.composition.clone();
                        temp.push(undo.clone());
                        if str_flat
                            == flatten(&temp[start..], VIETNAMESE_MODE | TONE_LESS | LOWER_CASE)
                        {
                            continue;
                        }
                        transformations.push(undo);
                    }
                }
            }
        }
        transformations
    }

    /// Fallback logic to append characters when no transformation matches the existing composition target.
    fn generate_fallback_transformations(
        &self,
        applicable_rules: &[Rule],
        lower_key: char,
        is_upper_case: bool,
    ) -> Vec<Transformation> {
        let mut transformations = Vec::new();
        let mut appended_rules = Vec::new();
        let mut primary_rule = None;

        for rule in applicable_rules {
            if rule.key == lower_key && rule.effect_type == EffectType::Appending {
                let mut r = rule.clone();
                let is_upper = is_upper_case || r.effect_on.is_ascii_uppercase();
                r.effect_on = r.effect_on.to_ascii_lowercase();
                r.result = r.effect_on;
                primary_rule = Some((r, is_upper));
                break;
            }
        }

        let trans = if let Some((r, is_upper)) = primary_rule {
            appended_rules = r.appended_rules.clone();
            Transformation {
                rule: r,
                target: None,
                is_upper_case: is_upper,
            }
        } else {
            new_appending_trans(lower_key, is_upper_case)
        };

        transformations.push(trans);

        for mut rule in appended_rules {
            let is_upper = is_upper_case || rule.effect_on.is_ascii_uppercase();
            rule.key = '\0';
            rule.effect_on = rule.effect_on.to_ascii_lowercase();
            rule.result = rule.effect_on;
            transformations.push(Transformation {
                rule,
                target: None,
                is_upper_case: is_upper,
            });
        }

        transformations
    }

    /// Shortcut logic for typing `uow` sequences.
    fn apply_uow_shortcut(
        &self,
        syllable: &[Transformation],
        start: usize,
    ) -> Option<Transformation> {
        let flat = flatten(&syllable[start..], TONE_LESS | LOWER_CASE);
        let re_uoh_tail = regex::Regex::new(r"(uơ|ưo)\p{L}+").unwrap();

        if !self.input_method.super_keys.is_empty() && re_uoh_tail.is_match(&flat) {
            let super_key = self.input_method.super_keys[0];
            let applicable_rules = self.get_applicable_rules(super_key);
            let (target, missing_rule) =
                self.find_target_in_slice(syllable, start, &applicable_rules);
            if let (Some(tgt_idx), Some(mut r)) = (target, missing_rule) {
                r.key = '\0';
                return Some(Transformation {
                    rule: r,
                    target: Some(tgt_idx),
                    is_upper_case: false,
                });
            }
        }
        None
    }
}

// Global helper functions

/// Evaluates composition transformations and flattens them to a final String buffer.
pub fn flatten(composition: &[Transformation], mode: u32) -> String {
    let canvas = get_canvas(composition, mode);
    canvas.into_iter().collect()
}

/// Evaluates diacritics and casing, generating a sequential list of characters (`Vec<char>`).
pub fn get_canvas(composition: &[Transformation], mode: u32) -> Vec<char> {
    let mut canvas = Vec::new();
    let mut appending_map: HashMap<usize, Vec<Transformation>> = HashMap::new();
    let mut appending_list = Vec::new();
    let mut index_map = HashMap::new();

    for (i, trans) in composition.iter().enumerate() {
        if (mode & ENGLISH_MODE) != 0 {
            if trans.rule.key == '\0' {
                continue;
            }
            appending_list.push(trans.clone());
        } else if trans.rule.effect_type == EffectType::Appending {
            if trans.rule.key == '\0' {
                continue;
            }
            appending_list.push(trans.clone());
            index_map.insert(i, appending_list.len() - 1);
        } else if let Some(target_idx) = trans.target {
            appending_map
                .entry(target_idx)
                .or_default()
                .push(trans.clone());
        }
    }

    for (i, appending_trans) in composition.iter().enumerate() {
        if appending_trans.rule.effect_type != EffectType::Appending
            || appending_trans.rule.key == '\0'
        {
            continue;
        }

        let mut chr;
        if (mode & ENGLISH_MODE) != 0 {
            chr = appending_trans.rule.key;
        } else {
            chr = appending_trans.rule.effect_on;
            if let Some(trans_list) = appending_map.get(&i) {
                for trans in trans_list {
                    match trans.rule.effect_type {
                        EffectType::MarkTransformation => {
                            if trans.rule.effect == Mark::Raw as u8 {
                                chr = appending_trans.rule.key;
                            } else {
                                chr = add_mark_to_char(chr, trans.rule.effect);
                            }
                        }
                        EffectType::ToneTransformation => {
                            chr = add_tone_to_char(chr, trans.rule.effect);
                        }
                        _ => {}
                    }
                }
            }
        }

        if (mode & TONE_LESS) != 0 {
            chr = add_tone_to_char(chr, 0);
        }
        if (mode & MARK_LESS) != 0 {
            chr = add_mark_to_char(chr, 0);
        }

        if (mode & LOWER_CASE) != 0 {
            chr = chr.to_ascii_lowercase();
        } else if appending_trans.is_upper_case {
            chr = chr.to_ascii_uppercase();
        }

        canvas.push(chr);
    }

    canvas
}

/// Finds the index of the last character appending transformation step in composition.
pub fn find_last_appending_trans_idx(composition: &[Transformation]) -> Option<usize> {
    composition
        .iter()
        .rposition(|t| t.rule.effect_type == EffectType::Appending)
}

/// Creates a new character appending transformation struct.
pub fn new_appending_trans(key: char, is_upper_case: bool) -> Transformation {
    Transformation {
        is_upper_case,
        target: None,
        rule: Rule {
            key,
            effect_on: key,
            effect_type: EffectType::Appending,
            result: key,
            effect: 0,
            appended_rules: Vec::new(),
        },
    }
}

/// Traverses composition targets recursively to find the original character appending index.
pub fn find_root_target(composition: &[Transformation], index: usize) -> usize {
    if let Some(target_idx) = composition[index].target {
        find_root_target(composition, target_idx)
    } else {
        index
    }
}

/// Checks if a target index has not been modified by a specific effect type.
pub fn is_free(
    composition: &[Transformation],
    start: usize,
    target: usize,
    effect_type: EffectType,
) -> bool {
    for i in start..composition.len() {
        let t = &composition[i];
        if t.target == Some(target) && t.rule.effect_type == effect_type {
            return false;
        }
    }
    true
}

/// Filters indices, keeping only those representing active Appending transformations in composition.
pub fn filter_appending_composition(
    composition: &[Transformation],
    indices: Vec<usize>,
) -> Vec<usize> {
    indices
        .into_iter()
        .filter(|&idx| {
            idx < composition.len() && composition[idx].rule.effect_type == EffectType::Appending
        })
        .collect()
}

/// Retrieves composition indices corresponding to vowel transformations in the last word.
pub fn get_right_most_vowels_start(composition: &[Transformation], start: usize) -> Vec<usize> {
    let (_, vo, _) = extract_cvc_trans_indices(composition, start);
    vo
}

/// Determines the vowel transformation index that should receive the tone mark under standard/modern rules.
pub fn find_tone_target(
    composition: &[Transformation],
    start: usize,
    std_style: bool,
) -> Option<usize> {
    if composition.len() <= start {
        return None;
    }
    let (_, vo, lc) = extract_cvc_trans_indices(composition, start);
    let vowels = filter_appending_composition(composition, vo.clone());
    if vowels.is_empty() {
        return None;
    }

    let mut target = None;
    if vowels.len() == 1 {
        target = Some(vowels[0]);
    } else if vowels.len() == 2 && std_style {
        for &idx in &vo {
            let res = composition[idx].rule.result;
            if res == 'ơ' || res == 'ê' {
                if composition[idx].target.is_some() {
                    target = composition[idx].target;
                } else {
                    target = Some(idx);
                }
            }
        }
        if target.is_none() {
            if !lc.is_empty() {
                target = Some(vowels[1]);
            } else {
                target = Some(vowels[0]);
            }
        }
    } else if vowels.len() == 2 {
        if !lc.is_empty() {
            target = Some(vowels[1]);
        } else {
            let sub: Vec<Transformation> = vowels.iter().map(|&i| composition[i].clone()).collect();
            let flat = flatten(&sub, ENGLISH_MODE | LOWER_CASE | TONE_LESS | MARK_LESS);
            if flat == "oa" || flat == "oe" || flat == "uy" || flat == "ue" || flat == "uo" {
                target = Some(vowels[1]);
            } else {
                target = Some(vowels[0]);
            }
        }
    } else if vowels.len() == 3 {
        let sub: Vec<Transformation> = vowels.iter().map(|&i| composition[i].clone()).collect();
        let flat = flatten(&sub, ENGLISH_MODE | LOWER_CASE | TONE_LESS | MARK_LESS);
        if flat == "uye" {
            target = Some(vowels[2]);
        } else {
            target = Some(vowels[1]);
        }
    }

    target
}

/// Checks if the tone is valid for the current vowel/consonant combination.
pub fn has_valid_tone(composition: &[Transformation], start: usize, tone: Tone) -> bool {
    if tone == Tone::None || tone == Tone::Acute || tone == Tone::Dot {
        return true;
    }
    let (_, _, lc) = extract_cvc_trans_indices(composition, start);
    if lc.is_empty() {
        return true;
    }
    let sub: Vec<Transformation> = lc.iter().map(|&i| composition[i].clone()).collect();
    let last_consonants = flatten(&sub, ENGLISH_MODE | LOWER_CASE);

    let dot_with_consonants = vec!["c", "k", "p", "t", "ch"];
    !dot_with_consonants.contains(&last_consonants.as_str())
}

/// Validates spelling constraints of a composition slice starting from a specified index.
pub fn is_valid_slice(
    composition: &[Transformation],
    start: usize,
    input_is_full_complete: bool,
) -> bool {
    if composition.len() - start <= 1 {
        return true;
    }

    let mut last_tone = None;
    for i in (start..composition.len()).rev() {
        if composition[i].rule.effect_type == EffectType::ToneTransformation {
            last_tone = Some(Tone::from_u8(composition[i].rule.effect));
            break;
        }
    }

    if let Some(tone) = last_tone {
        if !has_valid_tone(composition, start, tone) {
            return false;
        }
    }

    let (fc, vo, lc) = extract_cvc_trans_indices(composition, start);
    let sub_fc: Vec<Transformation> = fc.iter().map(|&i| composition[i].clone()).collect();
    let sub_vo: Vec<Transformation> = vo.iter().map(|&i| composition[i].clone()).collect();
    let sub_lc: Vec<Transformation> = lc.iter().map(|&i| composition[i].clone()).collect();

    let flatten_mode = VIETNAMESE_MODE | LOWER_CASE | TONE_LESS;
    is_valid_cvc(
        &flatten(&sub_fc, flatten_mode),
        &flatten(&sub_vo, flatten_mode),
        &flatten(&sub_lc, flatten_mode),
        input_is_full_complete,
    )
}

/// Internal helper to extract segment transformations recursively.
fn extract_atomic_trans(
    composition: &[Transformation],
    indices: &[usize],
    last: &[usize],
    last_is_vowel: bool,
) -> (Vec<usize>, Vec<usize>) {
    if indices.is_empty() {
        return (Vec::new(), last.to_vec());
    }
    let last_idx = indices[indices.len() - 1];
    let tmp = &composition[last_idx];
    if tmp.target.is_none() && last_is_vowel != is_vowel(tmp.rule.result) {
        return (indices.to_vec(), last.to_vec());
    }
    let mut next_last = vec![last_idx];
    next_last.extend_from_slice(last);
    extract_atomic_trans(
        composition,
        &indices[..indices.len() - 1],
        &next_last,
        last_is_vowel,
    )
}

/// Splits composition indices into First Consonant, Vowel, and Last Consonant lists of raw characters.
pub fn extract_cvc_appending_trans(
    composition: &[Transformation],
    indices: &[usize],
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let (head, last_consonant) = extract_atomic_trans(composition, indices, &[], false);
    let (mut first_consonant, mut vowel) = extract_atomic_trans(composition, &head, &[], true);

    if last_consonant.is_empty() && vowel.is_empty() && first_consonant.is_empty() {
        return (Vec::new(), Vec::new(), Vec::new());
    }

    if !last_consonant.is_empty() && vowel.is_empty() && first_consonant.is_empty() {
        first_consonant = last_consonant;
        return (first_consonant, Vec::new(), Vec::new());
    }

    // 'gi' and 'qu' qualified consonants exceptions
    if first_consonant.len() == 1 && !vowel.is_empty() {
        let fc_idx = first_consonant[0];
        let v0_idx = vowel[0];
        let fc_char = composition[fc_idx].rule.result;
        let v0_char = composition[v0_idx].rule.result;

        let cond_g = fc_char == 'g'
            && v0_char == 'i'
            && vowel.len() > 1
            && !(vowel[1] < composition.len()
                && composition[vowel[1]].rule.result == 'e'
                && !last_consonant.is_empty());
        let cond_q = fc_char == 'q' && v0_char == 'u';

        if cond_g || cond_q {
            first_consonant.push(vowel[0]);
            vowel.remove(0);
        }
    }

    (first_consonant, vowel, last_consonant)
}

/// Splits and groups composition transformations into First Consonant, Vowel, and Last Consonant index lists.
pub fn extract_cvc_trans_indices(
    composition: &[Transformation],
    start: usize,
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut trans_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut appending_list = Vec::new();

    for i in start..composition.len() {
        let trans = &composition[i];
        if trans.target.is_none() {
            appending_list.push(i);
        } else if let Some(tgt) = trans.target {
            trans_map.entry(tgt).or_default().push(i);
        }
    }

    let (fc, vo, lc) = extract_cvc_appending_trans(composition, &appending_list);

    let mut full_fc = Vec::new();
    for &t in &fc {
        full_fc.push(t);
        if let Some(list) = trans_map.get(&t) {
            full_fc.extend(list);
        }
    }

    let mut full_vo = Vec::new();
    for &t in &vo {
        full_vo.push(t);
        if let Some(list) = trans_map.get(&t) {
            full_vo.extend(list);
        }
    }

    let mut full_lc = Vec::new();
    for &t in &lc {
        full_lc.push(t);
        if let Some(list) = trans_map.get(&t) {
            full_lc.extend(list);
        }
    }

    (full_fc, full_vo, full_lc)
}

/// Locates the beginning index of the last word in composition, treating punctuation marks as word boundary markers.
pub fn extract_last_word_with_punctuation_marks_start(
    composition: &[Transformation],
    _effect_keys: &[char],
) -> usize {
    for i in (0..composition.len()).rev() {
        let sub = &composition[i..];
        let canvas = get_canvas(sub, ENGLISH_MODE);
        if canvas.is_empty() {
            continue;
        }
        let c = canvas[0];
        if is_space(c) {
            return i + 1;
        }
    }
    0
}

/// Locates the beginning index of the last word in the composition history.
pub fn extract_last_word_start(composition: &[Transformation], effect_keys: &[char]) -> usize {
    for i in (0..composition.len()).rev() {
        let sub = &composition[i..];
        let canvas = get_canvas(sub, VIETNAMESE_MODE | LOWER_CASE | TONE_LESS | MARK_LESS);
        if canvas.is_empty() {
            continue;
        }
        let c = canvas[0];
        if !is_alpha(c) && !effect_keys.contains(&c) {
            return i + 1;
        }
    }
    0
}

/// Locates the beginning index of the last syllable in the composition history.
pub fn extract_last_syllable_start(composition: &[Transformation], effect_keys: &[char]) -> usize {
    let start = extract_last_word_start(composition, effect_keys);
    let last = &composition[start..];

    let mut anchor = 0;
    for i in 0..last.len() {
        if !is_valid_slice(&last[anchor..=i], 0, false) {
            anchor = i;
        }
    }
    start + anchor
}

/// Retrieves the composition index of the last tone modification step.
pub fn get_last_tone_transformation_idx(
    composition: &[Transformation],
    start: usize,
) -> Option<usize> {
    composition[start..]
        .iter()
        .rposition(|t| t.rule.effect_type == EffectType::ToneTransformation && t.target.is_some())
        .map(|idx| start + idx)
}

/// Recalculates and adjusts the target vowel index receiving the tone diacritic mark in the last word.
pub fn refresh_last_tone_target(
    composition: &[Transformation],
    start: usize,
    std_style: bool,
) -> Vec<Transformation> {
    let mut transformations = Vec::new();
    let rightmost_vowels = get_right_most_vowels_start(composition, start);
    let last_tone_idx = get_last_tone_transformation_idx(composition, start);

    if rightmost_vowels.is_empty() || last_tone_idx.is_none() {
        return Vec::new();
    }
    let last_tone_idx = last_tone_idx.unwrap();
    let last_tone_trans = &composition[last_tone_idx];

    let new_tone_target = find_tone_target(composition, start, std_style);
    if last_tone_trans.target != new_tone_target {
        transformations.push(Transformation {
            target: last_tone_trans.target,
            rule: Rule {
                key: '\0',
                effect_type: EffectType::ToneTransformation,
                effect: Tone::None as u8,
                effect_on: '\0',
                result: '\0',
                appended_rules: Vec::new(),
            },
            is_upper_case: false,
        });

        let mut override_rule = last_tone_trans.rule.clone();
        override_rule.key = '\0';
        transformations.push(Transformation {
            target: new_tone_target,
            rule: override_rule,
            is_upper_case: false,
        });
    }

    transformations
}

// Tone mapping extension
impl Tone {
    /// Maps a numerical value (0-5) to its corresponding `Tone` variant.
    pub fn from_u8(val: u8) -> Self {
        match val {
            1 => Tone::Grave,
            2 => Tone::Acute,
            3 => Tone::Hook,
            4 => Tone::Tilde,
            5 => Tone::Dot,
            _ => Tone::None,
        }
    }
}
