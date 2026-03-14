use crate::components::ruby_pinyin::RubyPinyin;
use crate::utils::{try_find_word_in_dict, word_is_chinese_word, HskDictEntry, HskLevel};

use leptos::prelude::*;

use jieba_rs::Jieba;
use pinyin::ToPinyin;
use std::collections::HashMap;

/// A component that analyze and annotates in pinyin the text
#[component]
pub fn AnalyzedText(
    text: ReadSignal<String>,
    hsk_dict: ReadSignal<HashMap<String, HskDictEntry>>,
    hsk_level: ReadSignal<HskLevel>,
) -> impl IntoView {
    view! {
        {move || {
            let jieba = Jieba::new();
            let mut text = text();
            text.push('\n');
            let dict = hsk_dict();
            let level: u32 = hsk_level().into();
            let mut views = vec![];
            let mut current_line = String::new();
            for current_char in text.chars() {
                if current_char == '\n' {
                    views.push(view! { <br /> }.into_any());
                    let words = jieba.cut(&current_line, false);
                    for word in words {
                        if word_is_chinese_word(word) {
                            let mut pinyin: String;
                            if let Some(hsk_words) = try_find_word_in_dict(&dict, word) {
                                for hsk_word in hsk_words {
                                    if hsk_word.level > level {
                                        pinyin = hsk_word.pinyin.clone();
                                    } else {
                                        pinyin = String::new();
                                    }
                                    views
                                        .push(
                                            view! {
                                                <RubyPinyin
                                                    word=hsk_word.hanzi.clone()
                                                    pinyin=String::from(pinyin)
                                                />
                                            }
                                                .into_any(),
                                        );
                                }
                            } else {
                                let pinyin_vec = word.to_pinyin().flatten();
                                pinyin = pinyin_vec
                                    .into_iter()
                                    .map(|x| x.with_tone())
                                    .collect::<Vec<&str>>()
                                    .join("");
                                views
                                    .push(

                                        view! {
                                            <RubyPinyin
                                                word=String::from(word)
                                                pinyin=String::from(pinyin)
                                            />
                                        }
                                            .into_any(),
                                    );
                            }
                        } else {
                            views
                                .push(

                                    view! { <span class="font-serif">{word}</span> }
                                        .into_any(),
                                );
                        }
                    }
                    current_line = String::new();
                } else {
                    current_line.push(current_char);
                }
            }
            views.collect_view()
        }}
    }
}
