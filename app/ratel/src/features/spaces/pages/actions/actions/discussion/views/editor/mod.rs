use super::*;
use crate::features::spaces::pages::actions::components::ActionEditTopbar;

mod i18n;
pub use i18n::DiscussionEditorTranslate;

mod content_card;
mod config_card;
use config_card::ConfigCard;
use content_card::ContentCard;

#[component]
pub fn DiscussionActionEditorPage(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
) -> Element {
    let nav = use_navigator();
    let space_id_signal: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let discussion_id_signal: ReadSignal<SpacePostEntityType> =
        use_signal(|| discussion_id.clone()).into();
    let ctx = Context::init(space_id_signal, discussion_id_signal)?;

    let tr: DiscussionEditorTranslate = use_translate();
    let space = crate::features::spaces::space_common::hooks::use_space()();

    let initial_title = ctx.discussion().post.title.clone();
    let title = use_signal(|| initial_title);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "arena",
            ActionEditTopbar {
                space_name: space.title.clone(),
                action_type_label: tr.type_badge_label.to_string(),
                action_type_key: "discussion".to_string(),
                title,
                on_title_change: move |_v: String| {},
                editable_title: false,
                on_back: move |_| {
                    nav.go_back();
                },
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
            main { class: "pager",
                ContentCard {}
                ConfigCard {}
            }
        }
    }
}
