use super::*;
use crate::features::spaces::pages::actions::components::{
    ActionEditFooter, ActionEditSaveBus, ActionEditTopbar,
};

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

    ActionEditSaveBus::provide();
    let current_page = use_signal(|| 0usize);

    rsx! {
        div { class: "arena",
            ActionEditTopbar {
                space_name: space.title.clone(),
                action_type_label: tr.type_badge_label.to_string(),
                action_type_key: "discussion",
                title,
                on_title_change: move |_v: String| {},
                editable_title: false,
                right_actions: rsx! {
                    button {
                        class: "btn btn--secondary",
                        r#type: "button",
                        "data-testid": "discussion-import-overview",
                        onclick: move |_| {
                            let mut import_request = ctx.import_request;
                            import_request.set(import_request() + 1);
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line {
                                x1: "12",
                                y1: "15",
                                x2: "12",
                                y2: "3",
                            }
                        }
                        "{tr.import_from_overview}"
                    }
                },
                on_back: move |_| {
                    nav.go_back();
                },
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
            main { class: "pager",
                div {
                    class: "pager__track",
                    style: "transform: translateX(-{current_page() * 100}%);",
                    ContentCard {}
                    ConfigCard {}
                }
            }
            ActionEditFooter {
                current_page,
                total_pages: 2,
                action_type_key: "discussion",
            }
        }
    }
}
