use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionViewerTranslate;
use crate::features::spaces::pages::apps::apps::file::components::FileCard;

#[component]
pub fn DiscussionAttachments(files: Vec<File>) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    if files.is_empty() {
        return rsx! {};
    }

    rsx! {
        section { class: "flex flex-col gap-2",
            h2 { class: "text-xs font-medium tracking-wide uppercase text-foreground-muted",
                "{tr.attachments}"
            }
            div { class: "grid grid-cols-1 gap-2.5 md:grid-cols-2 desktop:grid-cols-3",
                for file in files.iter() {
                    FileCard {
                        key: "{file.id}",
                        file: file.clone(),
                        editable: false,
                        on_delete: None,
                    }
                }
            }
        }
    }
}
