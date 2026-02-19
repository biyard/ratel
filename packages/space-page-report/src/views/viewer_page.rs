use super::*;

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let _ = space_id;
    let tr: ReportTranslate = use_translate();
    let mut content = use_signal(String::new);
    let editable = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-4 w-full items-center",
            div { class: "flex items-center justify-between w-full max-w-5xl",
                Typo { variant: Variant::H1, weight: Weight::Extrabold, "{tr.title_editor}" }
            }
            div { class: "w-full max-w-5xl rounded-lg bg-card p-6 flex flex-col min-h-0 overflow-hidden",
                div { class: "flex items-center justify-end flex-shrink-0",
                    div { class: "flex items-center gap-3" }
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
