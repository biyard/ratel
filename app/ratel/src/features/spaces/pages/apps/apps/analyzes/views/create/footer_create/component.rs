//! Page-level fixed footer shown in CREATE mode (1 / 2 · 취소 / 다음 →).
//! Lives outside the wizard card so `position: fixed` anchors to the
//! viewport, not to a `backdrop-filter` containing block.

use crate::features::spaces::pages::apps::apps::analyzes::views::create::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn FooterCreate(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create()?;
    let nav = use_navigator();

    let mode = ctrl.mode.read().clone();
    let visible = matches!(mode, CreateMode::Create);

    rsx! {
        footer {
            class: "builder-actions builder-actions--create",
            id: "footer-create",
            hidden: !visible,
            div { class: "builder-actions__step", "{tr.create_footer_step_label_create}" }
            div { class: "builder-actions__group",
                button {
                    r#type: "button",
                    class: "btn btn--ghost",
                    id: "create-cancel",
                    "data-testid": "create-cancel",
                    onclick: move |_| {
                        ctrl.clear_filters();
                        nav.push(Route::SpaceAnalyzesAppPage {
                            space_id: space_id(),
                        });
                    },
                    "{tr.create_footer_cancel}"
                }
                button {
                    r#type: "button",
                    class: "btn btn--primary",
                    id: "create-next",
                    "data-testid": "create-next",
                    onclick: move |_| ctrl.goto_preview(),
                    "{tr.create_footer_next}"
                }
            }
        }
    }
}
