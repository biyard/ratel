use crate::features::spaces::pages::actions::*;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

#[component]
pub fn PrerequisiteTile(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    initial_prerequisite: bool,
    #[props(default)] on_changed: EventHandler<bool>,
) -> Element {
    let tr: PrerequisiteTileTranslate = use_translate();
    let mut toast = use_toast();
    let mut prereq = use_signal(|| initial_prerequisite);

    let toggle = move |_| {
        let new_val = !prereq();
        prereq.set(new_val);
        spawn(async move {
            let req = UpdateSpaceActionRequest::Prerequisite {
                prerequisite: new_val,
            };
            match update_space_action(space_id(), action_id(), req).await {
                Ok(_) => on_changed.call(new_val),
                Err(err) => {
                    toast.error(err);
                    prereq.set(!new_val);
                }
            }
        });
    };

    rsx! {
        div { class: "tile", "data-testid": "tile-prereq",
            span { class: "tile__label", "{tr.title}" }
            div { class: "tile__row",
                span { style: "font-size:13px;color:var(--qc-text-muted)",
                    "{tr.description}"
                }
                crate::common::components::Switch {
                    active: prereq(),
                    on_toggle: toggle,
                    label: tr.title.to_string(),
                }
            }
        }
    }
}

translate! {
    PrerequisiteTileTranslate;

    title: {
        en: "Space Entry Requirement",
        ko: "스페이스 참여 조건",
    },
    description: {
        en: "Make this action required for joining the space",
        ko: "이 액션을 스페이스 참여 필수 조건으로 설정",
    },
}
