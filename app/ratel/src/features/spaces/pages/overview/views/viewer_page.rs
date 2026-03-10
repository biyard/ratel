use super::*;
use crate::common::components::{SpaceCard, TiptapEditor};
use crate::features::spaces::pages::apps::apps::file::components::FileCard;
use crate::features::spaces::space_common::hooks::use_space_query;

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let tr: OverviewTranslate = use_translate();
    let space_loader = use_space_query(&space_id)?;
    let space = space_loader.read().clone();

    let header_image = space.urls.first().cloned();
    let content = space.content.clone();

    let mut files = use_signal(Vec::<File>::new);
    let mut did_load = use_signal(|| false);

    let space_id_for_load = space_id.clone();
    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);

        let space_id = space_id_for_load.clone();
        spawn(async move {
            if let Ok(loaded_files) =
                crate::features::spaces::pages::apps::apps::file::get_space_files(space_id).await
            {
                files.set(loaded_files);
            }
        });
    });

    rsx! {
        div { class: "flex flex-col gap-2.5 w-full",
            // Header image
            if let Some(url) = header_image {
                div { class: "overflow-hidden w-full rounded-lg aspect-video",
                    img { class: "object-cover w-full h-full", src: "{url}" }
                }
            }

            // Content (read-only)
            SpaceCard { class: "rounded-lg !p-6 !bg-transparent",
                TiptapEditor {
                    class: "w-full h-fit [&>div]:bg-transparent [&_[data-tiptap-toolbar]]:bg-transparent",
                    content,
                    editable: false,
                    placeholder: tr.placeholder,
                }
            }

            // File attachments
            if !files().is_empty() {
                div { class: "flex flex-col gap-2.5 w-full",
                    for file in files().iter() {
                        FileCard {
                            key: "{file.id}",
                            file: file.clone(),
                            editable: false,
                        }
                    }
                }
            }
        }
    }
}
