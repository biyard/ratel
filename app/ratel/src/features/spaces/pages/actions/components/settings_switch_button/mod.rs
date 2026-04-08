use crate::common::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};

/// Context signal provided by each `*ActionPage` dispatcher. When set
/// to `true`, creators are shown the creator (configuration) view even
/// if the action has already started. Default is `false` — creators
/// start on the participant view so they moderate alongside everyone.
#[derive(Clone, Copy)]
pub struct ActionEditMode(pub Signal<bool>);

pub fn use_action_edit_mode() -> ActionEditMode {
    use_context::<ActionEditMode>()
}

translate! {
    SettingsSwitchButtonTranslate;
    settings: {
        en: "Settings",
        ko: "설정하기",
    },
    back_to_participant: {
        en: "Back",
        ko: "참여 화면",
    },
}

/// Top-right floating button that lets a creator toggle between the
/// participant view and the creator (configuration) view on an action
/// page.
///
/// Renders nothing for non-creators. When `edit_mode` is `false` it
/// shows a gear + "설정하기" label that flips the mode to `true`.
/// When `edit_mode` is `true` it shows a back arrow + label that
/// returns the creator to the participant view.
#[component]
pub fn SettingsSwitchButton() -> Element {
    let tr: SettingsSwitchButtonTranslate = use_translate();
    let role = use_space_role()();
    let edit_mode_ctx = use_action_edit_mode();
    let mut edit_mode = edit_mode_ctx.0;

    if role != SpaceUserRole::Creator {
        return rsx! {};
    }

    // Once the space has ended (Processing / Finished), creators also
    // lose the settings entry point — they can still browse the page
    // but no reconfiguration is meaningful anymore.
    let space = use_space()();
    if matches!(
        space.status,
        Some(SpaceStatus::Processing | SpaceStatus::Finished)
    ) {
        return rsx! {};
    }

    let is_editing = edit_mode();

    rsx! {
        div { class: "flex justify-end items-center w-full mb-2",
            button {
                r#type: "button",
                class: "inline-flex gap-2 items-center px-3 py-2 text-sm font-semibold rounded-full border transition-colors cursor-pointer bg-card-bg border-card-border text-text-primary hover:bg-hover",
                "data-testid": "action-settings-switch",
                onclick: move |_| {
                    edit_mode.set(!is_editing);
                },
                if is_editing {
                    icons::arrows::LineArrowLeft {
                        width: "16",
                        height: "16",
                        class: "[&>path]:stroke-current",
                    }
                    span { {tr.back_to_participant} }
                } else {
                    icons::settings::Settings2 {
                        width: "16",
                        height: "16",
                        class: "[&>path]:stroke-current [&>circle]:stroke-current [&>path]:fill-none [&>circle]:fill-none",
                    }
                    span { {tr.settings} }
                }
            }
        }
    }
}
