//! Vietnamese Typing Rules and Input Method Layout Parser.
//!
//! This module defines the core data structures (like `Rule`, `Mark`, `Tone`, `EffectType`)
//! and parser logic for compiling input method layouts (such as Telex, VNI, VIQR) into
//! evaluation rules used by the typing engine.

use crate::charset::{add_tone_to_char, get_mark_family, is_vowel};
use std::collections::HashMap;

/// The type of transformation effect applied by a keystroke.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    /// Appends specific characters directly (e.g. Telex 'w' appending 'ư').
    Appending,
    /// Adds, modifies, or removes a base diacritic mark (e.g. Telex 'a' -> 'â').
    MarkTransformation,
    /// Adds, modifies, or removes a tone mark (e.g. Telex 's' -> acute tone).
    ToneTransformation,
    /// Replaces a character or sequence with a designated result.
    Replacing,
}

/// Representation of Vietnamese base diacritic marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    /// No diacritic mark.
    None = 0,
    /// Circumflex mark (dấu mũ - `â`, `ê`, `ô`).
    Hat = 1,
    /// Breve mark (dấu trăng - `ă`).
    Breve = 2,
    /// Horn mark (dấu móc - `ơ`, `ư`).
    Horn = 3,
    /// Strikethrough/dash (dấu gạch ngang - `đ`).
    Dash = 4,
    /// Untransformed/raw indicator.
    Raw = 5,
}

/// Representation of Vietnamese tone marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    /// No tone (thanh ngang).
    None = 0,
    /// Grave accent (thanh huyền).
    Grave = 1,
    /// Acute accent (thanh sắc).
    Acute = 2,
    /// Hook above (thanh hỏi).
    Hook = 3,
    /// Tilde (thanh ngã).
    Tilde = 4,
    /// Underdot (thanh nặng).
    Dot = 5,
}

/// A specific rule mapping a trigger key to its transformation effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// The trigger key (e.g. 's', 'a', '6').
    pub key: char,
    /// The numerical value of the applied `Mark` or `Tone`.
    pub effect: u8,
    /// The type of transformation.
    pub effect_type: EffectType,
    /// The character that must be matched in the buffer for this rule to trigger.
    pub effect_on: char,
    /// The resulting character after transformation.
    pub result: char,
    /// Secondary rules appended alongside this rule.
    pub appended_rules: Vec<Rule>,
}

/// Representation of an entire input method layout configuration.
#[derive(Debug, Clone)]
pub struct InputMethod {
    /// The name of the input method (e.g., "Telex", "VNI").
    pub name: String,
    /// The compiled rules.
    pub rules: Vec<Rule>,
    /// Keys that can apply transformation to multiple characters simultaneously (e.g., 'w').
    pub super_keys: Vec<char>,
    /// Keys used to apply tones.
    pub tone_keys: Vec<char>,
    /// Keys used to trigger appending rules.
    pub appending_keys: Vec<char>,
    /// All key triggers defined in this input method.
    pub keys: Vec<char>,
}

lazy_static::lazy_static! {
    /// Maps string identifiers from configurations to their respective `Tone` representation.
    pub static ref TONES_MAP: HashMap<&'static str, Tone> = {
        let mut m = HashMap::new();
        m.insert("XoaDauThanh", Tone::None);
        m.insert("DauSac", Tone::Acute);
        m.insert("DauHuyen", Tone::Grave);
        m.insert("DauNga", Tone::Tilde);
        m.insert("DauNang", Tone::Dot);
        m.insert("DauHoi", Tone::Hook);
        m
    };

    /// Maps Vietnamese characters containing a diacritic to their respective `Mark` representation.
    pub static ref MARKS_BY_CHAR: HashMap<char, Mark> = {
        let mut m = HashMap::new();
        m.insert('â', Mark::Hat);
        m.insert('ê', Mark::Hat);
        m.insert('ô', Mark::Hat);
        m.insert('ă', Mark::Breve);
        m.insert('ơ', Mark::Horn);
        m.insert('ư', Mark::Horn);
        m.insert('đ', Mark::Dash);
        m
    };

    pub static ref INPUT_METHOD_DEFINITIONS: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut m = HashMap::new();

        // Telex Layout
        let mut telex = HashMap::new();
        telex.insert("z", "XoaDauThanh");
        telex.insert("s", "DauSac");
        telex.insert("f", "DauHuyen");
        telex.insert("r", "DauHoi");
        telex.insert("x", "DauNga");
        telex.insert("j", "DauNang");
        telex.insert("a", "A_Â");
        telex.insert("e", "E_Ê");
        telex.insert("o", "O_Ô");
        telex.insert("w", "UOA_ƯƠĂ__Ư");
        telex.insert("d", "D_Đ");
        telex.insert("]", "__ư");
        telex.insert("[", "__ơ");
        telex.insert("}", "__Ư");
        telex.insert("{", "__Ơ");
        m.insert("Telex", telex);

        // VNI Layout
        let mut vni = HashMap::new();
        vni.insert("0", "XoaDauThanh");
        vni.insert("1", "DauSac");
        vni.insert("2", "DauHuyen");
        vni.insert("3", "DauHoi");
        vni.insert("4", "DauNga");
        vni.insert("5", "DauNang");
        vni.insert("6", "AEO_ÂÊÔ");
        vni.insert("7", "UO_ƯƠ");
        vni.insert("8", "A_Ă");
        vni.insert("9", "D_Đ");
        m.insert("VNI", vni);

        m
    };
}

/// Resolves the `Mark` of a given Vietnamese character.
pub fn find_mark_from_char(chr: char) -> Option<Mark> {
    MARKS_BY_CHAR.get(&chr).copied()
}

/// Builds and compiles an `InputMethod` object from its layout name.
pub fn get_input_method(name: &str) -> Option<InputMethod> {
    let defs = &*INPUT_METHOD_DEFINITIONS;
    let im_def = defs.get(name)?;

    let mut im = InputMethod {
        name: name.to_string(),
        rules: Vec::new(),
        super_keys: Vec::new(),
        tone_keys: Vec::new(),
        appending_keys: Vec::new(),
        keys: Vec::new(),
    };

    for (&key_str, &line) in im_def {
        let key = key_str.chars().next()?;
        im.keys.push(key);

        let parsed_rules = parse_rules(key, line);
        for rule in &parsed_rules {
            if rule.effect_type == EffectType::Appending {
                im.appending_keys.push(rule.key);
            }
            if rule.effect_type == EffectType::ToneTransformation {
                im.tone_keys.push(rule.key);
            }
        }

        if line.to_lowercase().contains("uo") {
            im.super_keys.push(key);
        }

        im.rules.extend(parsed_rules);
    }

    Some(im)
}

/// Parses standard definitions/DSL strings into a set of typing rules.
pub fn parse_rules(key: char, line: &str) -> Vec<Rule> {
    if let Some(&tone) = TONES_MAP.get(line) {
        let rule = Rule {
            key,
            effect: tone as u8,
            effect_type: EffectType::ToneTransformation,
            effect_on: '\0',
            result: '\0',
            appended_rules: Vec::new(),
        };
        vec![rule]
    } else {
        parse_toneless_rules(key, line)
    }
}

/// Parses rules that are not tone-related (e.g. diacritic marks, character appends).
pub fn parse_toneless_rules(key: char, line: &str) -> Vec<Rule> {
    let line_lower = line.to_lowercase();
    let re_dsl = regex::Regex::new(r"([a-z]+)_([a-z\p{L}]+)([a-z\p{L}_]*)").unwrap();

    let mut rules = Vec::new();
    if let Some(caps) = re_dsl.captures(&line_lower) {
        let effective_ons: Vec<char> = caps.get(1).unwrap().as_str().chars().collect();
        let results: Vec<char> = caps.get(2).unwrap().as_str().chars().collect();
        let suffix = caps.get(3).unwrap().as_str();

        for i in 0..effective_ons.len() {
            if i < results.len() {
                if let Some(effect) = find_mark_from_char(results[i]) {
                    rules.extend(parse_toneless_rule(
                        key,
                        effective_ons[i],
                        results[i],
                        effect,
                    ));
                }
            }
        }

        if let Some(rule) = get_appending_rule(key, suffix) {
            rules.push(rule);
        }
    } else if let Some(rule) = get_appending_rule(key, line) {
        rules.push(rule);
    }
    rules
}

/// Compiles a base toneless conversion rule across all tone variations.
pub fn parse_toneless_rule(key: char, effective_on: char, result: char, effect: Mark) -> Vec<Rule> {
    let mut rules = Vec::new();
    let tones_list = vec![
        Tone::None,
        Tone::Dot,
        Tone::Acute,
        Tone::Grave,
        Tone::Hook,
        Tone::Tilde,
    ];

    for chr in get_mark_family(effective_on) {
        if chr == result {
            rules.push(Rule {
                key,
                effect_type: EffectType::MarkTransformation,
                effect: 0,
                effect_on: result,
                result: effective_on,
                appended_rules: Vec::new(),
            });
        } else if is_vowel(chr) {
            for &tone in &tones_list {
                let effect_on_tone = add_tone_to_char(chr, tone as u8);
                let result_tone = add_tone_to_char(result, tone as u8);
                rules.push(Rule {
                    key,
                    effect_type: EffectType::MarkTransformation,
                    effect_on: effect_on_tone,
                    effect: effect as u8,
                    result: result_tone,
                    appended_rules: Vec::new(),
                });
            }
        } else {
            rules.push(Rule {
                key,
                effect_type: EffectType::MarkTransformation,
                effect_on: chr,
                effect: effect as u8,
                result,
                appended_rules: Vec::new(),
            });
        }
    }
    rules
}

/// Parses and compiles character-appending rules from DSL format (e.g. `_ư` or `__ơ`).
pub fn get_appending_rule(key: char, value: &str) -> Option<Rule> {
    let re_dsl_appending = regex::Regex::new(r"(_?)_([a-z\p{L}]+)").unwrap();
    if let Some(caps) = re_dsl_appending.captures(value) {
        let chars: Vec<char> = caps.get(2).unwrap().as_str().chars().collect();
        if chars.is_empty() {
            return None;
        }
        let mut rule = Rule {
            key,
            effect_type: EffectType::Appending,
            effect_on: chars[0],
            result: chars[0],
            effect: 0,
            appended_rules: Vec::new(),
        };
        if chars.len() > 1 {
            for &chr in &chars[1..] {
                rule.appended_rules.push(Rule {
                    key,
                    effect_type: EffectType::Appending,
                    effect_on: chr,
                    result: chr,
                    effect: 0,
                    appended_rules: Vec::new(),
                });
            }
        }
        Some(rule)
    } else {
        None
    }
}
