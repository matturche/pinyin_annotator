use std::collections::HashMap;

use leptos::html;
use leptos::prelude::*;

use crate::components::analyzed_text::AnalyzedText;
use crate::components::language_controller::LanguageController;
use crate::components::theme_controller::ThemeController;
use crate::constants::HSK_LEVELS;
use crate::i18n::*;
use crate::utils::{HskLevel, HskVersion};
use crate::{api::fetch_hsk_dict_and_extension, utils::HskDictEntry};
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let i18n = use_i18n();
    let (hsk_dict, set_hsk_dict) = signal::<HashMap<String, HskDictEntry>>(HashMap::new());
    let (text, set_text) = signal(String::new());
    let (hsk_level, set_hsk_level) = signal(HskLevel::default());
    let fetched_hsk_dict = LocalResource::new(async move || {
        fetch_hsk_dict_and_extension(HskVersion::Hsk20, true).await
    });

    let textarea_node: NodeRef<html::Textarea> = NodeRef::new();
    let hsk_level_node: NodeRef<html::Select> = NodeRef::new();

    let set_text_to_annotate = move || {
        let text_from_textarea = textarea_node
            .get()
            .expect("<textarea> input should be mounted")
            .value();
        set_text(text_from_textarea);
        let level = hsk_level_node
            .get()
            .expect("<select> should be mounted for hsk level.")
            .value()
            .to_uppercase();
        set_hsk_level(HskLevel::from(level.as_str()));
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <Suspense fallback=move || {
                view! {
                    <div class="min-h-screen flex justify-center items-center">
                        <span class="loading loading-spinner text-primary"></span>
                    </div>
                }
            }>
                <div class="bg-base-200 min-h-screen">
                    <div class="flex justify-end p-2">
                        <ThemeController />
                        <LanguageController />
                    </div>
                    <div class="flex flex-col justify-center py-4 text-center">
                        <h1 class="text-2xl">{t!(i18n, title)}</h1>
                        <p class="text-lg">{t!(i18n, subtitle)}</p>
                    </div>

                    {move || {
                        Suspend::new(async move {
                            set_hsk_dict(fetched_hsk_dict.await);
                        })
                    }}
                    <div class="flex justify-center">
                        <div>
                            <fieldset class="fieldset">
                                <legend class="fieldset-legend">
                                    {t!(i18n, hsk_level_legend)}
                                </legend>

                                <div class="flex items-center justify-center object-fill">
                                    <select
                                        class="select select-primary w-fit"
                                        node_ref=hsk_level_node
                                        id="hsk-level"
                                    >
                                        {move || {
                                            HSK_LEVELS
                                                .iter()
                                                .map(|level| {
                                                    view! { <option>{level.to_string()}</option> }.into_any()
                                                })
                                                .collect_view()
                                        }}
                                    </select>

                                </div>
                            </fieldset>
                        </div>

                    </div>
                    <div class="flex justify-center items-center px-4 py-4">
                        <div class="w-full max-w-[1000px]">
                            <fieldset class="fieldset">
                                <legend class="fieldset-legend justify-center">
                                    {t!(i18n, textarea_legend)}
                                </legend>

                                <div class="flex items-center justify-center object-fill">
                                    <textarea
                                        class="textarea h-48 w-full p-4 font-serif text-xl"
                                        node_ref=textarea_node
                                        placeholder=t_string!(i18n, textarea_placeholder)
                                    />
                                </div>
                            </fieldset>
                        </div>
                    </div>
                    <div class="flex justify-center p-2">
                        <button
                            class="btn btn-accent text-white rounded-xl"
                            on:click=move |_evt| { set_text_to_annotate() }
                        >
                            {t!(i18n, annotate_button)}
                        </button>
                    </div>
                    <div class="flex justify-center">
                        <div class="px-4 py-4 pb-64 prose sm:prose-xl md:prose-2xl text-base-content text-justify text-pretty font-kai">
                            <AnalyzedText text hsk_dict hsk_level />
                        </div>
                    </div>

                </div>
            </Suspense>
        </ErrorBoundary>
    }
    .into_any()
}
