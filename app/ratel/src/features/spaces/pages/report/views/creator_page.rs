use super::*;
use crate::common::{
    components::{editor::Editor as RichEditor, Button, ButtonStyle},
    icons::{edit::Edit1, other_devices::Save},
};
use dioxus::prelude::spawn;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    let tr: ReportTranslate = use_translate();
    let space_id_load = space_id.clone();
    let space_id_generate = space_id.clone();
    let space_id_save = space_id.clone();
    let mut content = use_signal(String::new);
    let mut editable = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    let mut is_saving = use_signal(|| false);
    let mut error = use_signal(|| None as Option<String>);
    let mut did_load = use_signal(|| false);

    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);
        let space_pk = space_id_load.clone();
        let mut content = content.clone();
        let mut error = error.clone();
        spawn(async move {
            match crate::features::spaces::pages::report::controllers::get_analyze(space_pk).await {
                Ok(resp) => {
                    if let Some(html) = resp.html_contents {
                        if !html.trim().is_empty() {
                            content.set(html);
                        }
                    }
                }
                Err(err) => error.set(Some(err.to_string())),
            }
        });
    });

    rsx! {
        div { class: "flex flex-col gap-4 items-center w-full",
            div { class: "flex gap-2 justify-between items-center w-full max-w-5xl",
                h1 { "{tr.title_editor}" }
                Button {
                    style: ButtonStyle::Secondary,
                    onclick: move |_| {
                        if is_loading() {
                            return;
                        }
                        is_loading.set(true);
                        error.set(None);
                        let space_pk = space_id_generate.clone();
                        let mut content = content.clone();
                        let mut is_loading = is_loading.clone();
                        let mut error = error.clone();
                        spawn(async move {
                            match crate::features::spaces::pages::report::controllers::create_ai_report(
                                    space_pk,
                                )
                                .await
                            {
                                Ok(resp) => {
                                    content.set(resp.html_contents);
                                }
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
                div { class: "w-full max-w-5xl text-red-500", "{tr.generate_failed}: {message}" }
            }
            div { class: "flex flex-col p-6 w-full max-w-5xl rounded-lg bg-card",
                div { class: "flex flex-shrink-0 justify-end items-center",
                    div { class: "flex gap-3 items-center",
                        if !editable() {
                            button {
                                class: "w-5 h-5 cursor-pointer [&>path]:stroke-1",
                                aria_label: tr.btn_edit,
                                onclick: move |_| {
                                    if is_loading() || is_saving() {
                                        return;
                                    }
                                    editable.set(true);
                                },
                                Edit1 { class: "w-5 h-5 [&>path]:stroke-1 [&>path]:stroke-white" }
                            }
                        } else {
                            button {
                                class: "w-5 h-5 cursor-pointer [&>path]:stroke-1",
                                aria_label: tr.btn_save,
                                onclick: move |_| {
                                    if is_loading() || is_saving() {
                                        return;
                                    }
                                    is_saving.set(true);
                                    error.set(None);
                                    let space_pk = space_id_save.clone();
                                    let html = content();
                                    let mut is_saving = is_saving.clone();
                                    let mut error = error.clone();
                                    let mut editable = editable.clone();
                                    spawn(async move {
                                        match crate::features::spaces::pages::report::controllers::update_analyze(
                                                space_pk,
                                                crate::features::spaces::pages::report::controllers::UpdateAnalyzeHtmlRequest {
                                                    html_contents: html,
                                                },
                                            )
                                            .await
                                        {
                                            Ok(_) => editable.set(false),
                                            Err(err) => error.set(Some(err.to_string())),
                                        }
                                        is_saving.set(false);
                                    });
                                },
                                Save { class: "w-5 h-5 [&>path]:stroke-1 [&>path]:stroke-white" }
                            }
                        }
                    }
                }
                div { class: "flex overflow-hidden flex-col flex-1 w-full min-h-0",
                    RichEditor {
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
