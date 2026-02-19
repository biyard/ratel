use super::*;
use common::{
    components::{Button, ButtonStyle, TiptapEditor, Typo, Variant, Weight},
    icons::{edit::Edit1, other_devices::Save},
};
use dioxus::prelude::spawn;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    let tr: ReportTranslate = use_translate();
    let mut content = use_signal(String::new);
    let mut editable = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    let mut error = use_signal(|| None as Option<String>);

    rsx! {
        div { class: "flex flex-col gap-4 w-full items-center",
            div { class: "flex items-center justify-between w-full max-w-5xl gap-2",
                Typo { variant: Variant::H1, weight: Weight::Extrabold, "{tr.title_editor}" }
                Button {
                    style: ButtonStyle::Secondary,
                    onclick: move |_| {
                        if is_loading() {
                            return;
                        }
                        is_loading.set(true);
                        error.set(None);
                        let space_pk = space_id.clone();
                        let mut content = content.clone();
                        let mut is_loading = is_loading.clone();
                        let mut error = error.clone();
                        spawn(async move {
                            match crate::views::request_ai_report(space_pk).await {
                                Ok(resp) => content.set(resp.html_contents),
                                Err(err) => error.set(Some(err.to_string())),
                            }
                            is_loading.set(false);
                        });
                    },
                    if is_loading() {
                        "{tr.generating}"
                    } else {
                        "{tr.btn_generate_report}"
                    }
                }
            }
            if let Some(message) = error() {
                div { class: "text-red-500 w-full max-w-5xl", "{tr.generate_failed}: {message}" }
            }
            div { class: "w-full max-w-5xl rounded-lg bg-card p-6 flex flex-col min-h-0 overflow-hidden",
                div { class: "flex items-center justify-end flex-shrink-0",
                    div { class: "flex items-center gap-3",
                        if !editable() {
                            button {
                                class: "cursor-pointer w-5 h-5 [&>path]:stroke-1",
                                aria_label: tr.btn_edit,
                                onclick: move |_| editable.set(true),
                                Edit1 { class: "w-5 h-5 [&>path]:stroke-1 [&>path]:stroke-white" }
                            }
                        } else {
                            button {
                                class: "cursor-pointer w-5 h-5 [&>path]:stroke-1",
                                aria_label: tr.btn_save,
                                onclick: move |_| editable.set(false),
                                Save { class: "w-5 h-5 [&>path]:stroke-1 [&>path]:stroke-white" }
                            }
                        }
                    }
                }
                div { class: "flex flex-col w-full min-h-0 flex-1 overflow-hidden",
                    TiptapEditor {
                        class: "w-full h-fit",
                        content: content(),
                        editable: editable(),
                        placeholder: tr.placeholder,
                        on_content_change: move |html: String| {
                            content.set(html);
                        },
                    }
                }
            }
        }
    }
}
