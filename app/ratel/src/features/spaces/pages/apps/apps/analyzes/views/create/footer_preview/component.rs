//! Page-level fixed footer shown in PREVIEW mode (2 / 2 · ← 이전 ·
//! 보고서 생성). Mock confirm just navigates to the existing detail
//! mock report — Phase 2 doesn't persist anything.

use crate::features::spaces::pages::apps::apps::analyzes::views::create::*;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[component]
pub fn FooterPreview(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_analyze_create(space_id)?;
    let mut handle_submit = ctrl.handle_submit;

    let mode = ctrl.mode.read().clone();
    let visible = matches!(mode, CreateMode::Preview);

    rsx! {
        footer {
            class: "builder-actions builder-actions--preview",
            id: "footer-preview",
            hidden: !visible,
            div { class: "builder-actions__step", "{tr.create_footer_step_label_preview}" }
            div { class: "builder-actions__group",
                button {
                    r#type: "button",
                    class: "btn btn--ghost",
                    id: "preview-back",
                    "data-testid": "preview-back",
                    onclick: move |_| ctrl.back_to_create(),
                    "{tr.create_footer_back}"
                }
                button {
                    r#type: "button",
                    class: "btn btn--primary",
                    id: "preview-confirm",
                    "data-testid": "preview-confirm",
                    onclick: move |_| handle_submit.call(),
                    "{tr.create_footer_confirm}"
                }
            }
        }
    }
}
