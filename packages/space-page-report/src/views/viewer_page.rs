use super::*;

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let tr: ReportTranslate = use_translate();
    let mut content = use_signal(String::new);
    let editable = use_signal(|| false);
    let mut did_load = use_signal(|| false);
    let editor_key = use_signal(|| 0u32);

    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);
        let space_pk = space_id.clone();
        let mut content = content.clone();
        let mut editor_key = editor_key.clone();
        spawn(async move {
            if let Ok(resp) = crate::views::get_ai_report(space_pk).await {
                if let Some(html) = resp.html_contents {
                    if !html.trim().is_empty() {
                        content.set(html);
                        editor_key.set(editor_key() + 1);
                    }
                }
            }
        });
    });

    rsx! {
        div { class: "flex flex-col gap-4 w-full items-center",
            div { class: "flex items-center justify-between w-full max-w-5xl",
                Typo { variant: Variant::H1, weight: Weight::Extrabold, "{tr.title_editor}" }
            }
            div { class: "w-full max-w-5xl rounded-lg bg-card p-6 flex flex-col",
                div { class: "flex items-center justify-end flex-shrink-0",
                    div { class: "flex items-center gap-3" }
                }
                div { class: "flex flex-col w-full min-h-0 flex-1 overflow-hidden",
                    TiptapEditor {
                        key: "{editor_key()}",
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
