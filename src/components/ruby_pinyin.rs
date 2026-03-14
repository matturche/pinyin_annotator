use leptos::prelude::*;

#[component]
pub fn RubyPinyin(word: String, pinyin: String) -> impl IntoView {
    view! { <ruby>{word}<rt class="font-serif">{pinyin}</rt></ruby> }.into_any()
}
