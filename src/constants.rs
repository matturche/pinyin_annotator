// use std::collections::HashSet;
// use std::sync::LazyLock;

pub const LOCALES: &[&str] = &["en", "fr"];

pub const HSK_LEVELS: &[&str] = &[
    "None", "HSK1", "HSK2", "HSK3", "HSK4", "HSK5", "HSK6", "HSK7-9",
];

// // These are the lists of entries that will never get pinyin, as their pronounciation is too
// // unstable to rely on automatic pinyin annotation
// pub static EXCLUDED_ENTRIES: LazyLock<HashSet<&str>> = LazyLock::new(|| {
//     let mut s = HashSet::new();
//     s.insert("了");
//     s.insert("得");
//     s.insert("地");
//     s.insert("一");
//     s.insert("不");
//     s
// });
