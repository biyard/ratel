use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionViewerTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;

#[component]
pub fn DiscussionAttachments(files: Vec<File>) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    if files.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "disc-files",
            span { class: "disc-files__label", "{tr.attachments}" }
            div { class: "disc-files__grid",
                for file in files.iter() {
                    a {
                        class: "file-card",
                        key: "{file.id}",
                        href: file.url.as_deref().unwrap_or("#"),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        div { class: "file-card__icon",
                            lucide_dioxus::FileText { class: "w-[18px] h-[18px]" }
                        }
                        div { class: "file-card__info",
                            div { class: "file-card__name", "{file.name}" }
                            div { class: "file-card__size", "{file.size}" }
                        }
                    }
                }
            }
        }
    }
}
