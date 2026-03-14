use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HskDictEntry {
    pub pinyin: String,
    pub level: u32,
}

#[derive(Clone, Debug)]
pub struct HskWord {
    pub hanzi: String,
    pub pinyin: String,
    pub level: u32,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum HskVersion {
    #[default]
    Hsk20,
    Hsk30,
}

impl ToString for HskVersion {
    fn to_string(&self) -> String {
        match self {
            HskVersion::Hsk20 => String::from("HSK2.0"),
            HskVersion::Hsk30 => String::from("HSK3.0"),
        }
    }
}
impl From<&str> for HskVersion {
    fn from(value: &str) -> Self {
        match value {
            "HSK2.0" => Self::Hsk20,
            "HSK3.0" => Self::Hsk30,
            _ => Self::default(),
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum HskLevel {
    #[default]
    None,
    Hsk1,
    Hsk2,
    Hsk3,
    Hsk4,
    Hsk5,
    Hsk6,
    Hsk7to9,
}

impl ToString for HskLevel {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("NONE"),
            Self::Hsk1 => String::from("HSK1"),
            Self::Hsk2 => String::from("HSK2"),
            Self::Hsk3 => String::from("HSK3"),
            Self::Hsk4 => String::from("HSK4"),
            Self::Hsk5 => String::from("HSK5"),
            Self::Hsk6 => String::from("HSK6"),
            Self::Hsk7to9 => String::from("HSK7-9"),
        }
    }
}

impl From<&str> for HskLevel {
    fn from(value: &str) -> Self {
        match value {
            "None" => Self::None,
            "HSK1" => Self::Hsk1,
            "HSK2" => Self::Hsk2,
            "HSK3" => Self::Hsk3,
            "HSK4" => Self::Hsk4,
            "HSK5" => Self::Hsk5,
            "HSK6" => Self::Hsk6,
            "HSK7-9" => Self::Hsk7to9,
            _ => Self::None,
        }
    }
}

impl Into<u32> for HskLevel {
    fn into(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Hsk1 => 1,
            Self::Hsk2 => 2,
            Self::Hsk3 => 3,
            Self::Hsk4 => 4,
            Self::Hsk5 => 5,
            Self::Hsk6 => 6,
            Self::Hsk7to9 => 7,
        }
    }
}

// #[derive(Clone, Debug)]
// pub struct AnalyzeOptions {
//     pub hsk_level: HskLevel,
// }

pub fn word_is_chinese_word(word: &str) -> bool {
    let mut result = true;
    for character in word.chars() {
        if !char_is_hanzi(&character) {
            result = false;
        }
    }
    result
}

// Not exhaustive but contains a test for all unified ideographs and extensions A-H
pub fn char_is_hanzi(character: &char) -> bool {
    ('\u{4E00}' <= *character && *character <= '\u{9FFF}')
        || ('\u{3400}' <= *character && *character <= '\u{4DBF}')
        || ('\u{20000}' <= *character && *character <= '\u{2A6DF}')
        || ('\u{2A700}' <= *character && *character <= '\u{2B739}')
        || ('\u{2B740}' <= *character && *character <= '\u{2B81D}')
        || ('\u{2B820}' <= *character && *character <= '\u{2CEA1}')
        || ('\u{2CEB0}' <= *character && *character <= '\u{2EBE0}')
        || ('\u{30000}' <= *character && *character <= '\u{3134A}')
        || ('\u{31350}' <= *character && *character <= '\u{323AF}')
}

#[allow(dead_code)]
pub fn get_length_of_chinese_string(text: &str) -> usize {
    let mut length = 0;
    for idx in 0..text.len() {
        if text.is_char_boundary(idx) {
            length += 1;
        }
    }
    length
}

pub fn get_hanzi_sub_combinations_patterns(word: &str) -> Vec<Vec<String>> {
    let mut sub_combinations_patterns: Vec<Vec<String>> = vec![];
    let mut sub_combinations: Vec<String> = vec![];
    let chars = word.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    if chars.len() > 1 {
        // Add all individual hanzi as a pattern
        sub_combinations_patterns.push(chars.clone());
    }
    if chars.len() == 3 {
        sub_combinations.push(chars[0..2].join(""));
        sub_combinations.push(chars.get(2).unwrap().clone());
        sub_combinations_patterns.push(sub_combinations);
        sub_combinations = vec![];
        sub_combinations.push(chars.get(0).unwrap().clone());
        sub_combinations.push(chars[1..3].join(""));
        sub_combinations_patterns.push(sub_combinations);
    } else if chars.len() == 4 {
        sub_combinations.push(chars[0..2].join(""));
        sub_combinations.push(chars[2..4].join(""));
        sub_combinations_patterns.push(sub_combinations);
        sub_combinations = vec![];
        sub_combinations.push(chars.get(0).unwrap().clone());
        sub_combinations.push(chars.get(1).unwrap().clone());
        sub_combinations.push(chars[2..4].join(""));
        sub_combinations_patterns.push(sub_combinations);
        sub_combinations = vec![];
        sub_combinations.push(chars[0..2].join(""));
        sub_combinations.push(chars.get(2).unwrap().clone());
        sub_combinations.push(chars.get(3).unwrap().clone());
        sub_combinations_patterns.push(sub_combinations);
    }
    sub_combinations_patterns
}

pub fn pattern_is_in_dict(dict: &HashMap<String, HskDictEntry>, pattern: &Vec<String>) -> bool {
    let mut is = true;
    for word in pattern {
        let entry = dict.get(word);
        if let None = entry {
            is = false;
            break;
        }
    }
    is
}

pub fn try_find_word_in_dict(
    dict: &HashMap<String, HskDictEntry>,
    word: &str,
) -> Option<Vec<HskWord>> {
    let mut entries: Option<Vec<HskWord>> = None;
    let dict_entry = dict.get(word);
    if let Some(dict_entry) = dict_entry {
        let hsk_word = HskWord {
            hanzi: word.to_string(),
            pinyin: dict_entry.pinyin.clone(),
            level: dict_entry.level,
        };
        if let Some(ref mut entries) = entries {
            entries.push(hsk_word);
        } else {
            entries = Some(vec![hsk_word]);
        }
    } else {
        let patterns = get_hanzi_sub_combinations_patterns(word);
        for p in patterns {
            if pattern_is_in_dict(&dict, &p) {
                for w in p {
                    let dict_entry = dict.get(&w).unwrap();
                    let hsk_word = HskWord {
                        hanzi: w.to_string(),
                        pinyin: dict_entry.pinyin.clone(),
                        level: dict_entry.level,
                    };

                    if let Some(ref mut entries) = entries {
                        entries.push(hsk_word);
                    } else {
                        entries = Some(vec![hsk_word]);
                    }
                }
                break;
            }
        }
    }

    entries
}
