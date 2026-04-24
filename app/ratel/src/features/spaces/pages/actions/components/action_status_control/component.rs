use crate::features::spaces::pages::actions::*;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

#[component]
pub fn ActionStatusControl(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    initial_status: Option<SpaceActionStatus>,
    #[props(default)] on_changed: EventHandler<SpaceActionStatus>,
) -> Element {
    let tr: ActionStatusControlTranslate = use_translate();
    let mut toast = use_toast();
    let mut status = use_signal(|| initial_status.clone());
    let mut saving = use_signal(|| false);

    let current = status();

    let transition = use_callback(move |next: SpaceActionStatus| {
        if saving() {
            return;
        }
        saving.set(true);
        spawn(async move {
            let req = UpdateSpaceActionRequest::Status {
                status: next.clone(),
            };
            match update_space_action(space_id(), action_id(), req).await {
                Ok(_) => {
                    status.set(Some(next.clone()));
                    on_changed.call(next);
                }
                Err(e) => {
                    toast.error(e);
                }
            }
            saving.set(false);
        });
    });

    let badge_label = match &current {
        None => tr.status_legacy.to_string(),
        Some(SpaceActionStatus::Designing) => tr.status_designing.to_string(),
        Some(SpaceActionStatus::Ongoing) => tr.status_ongoing.to_string(),
        Some(SpaceActionStatus::Finish) => tr.status_finish.to_string(),
    };

    rsx! {
        div { class: "flex gap-3 justify-between items-center w-full",
            span { class: "font-medium text-[12px]/[16px] text-foreground-muted",
                "{badge_label}"
            }
            match current.clone() {
                None => rsx! {
                    Button {
                        size: ButtonSize::Small,
                        style: ButtonStyle::Primary,
                        disabled: saving(),
                        onclick: move |_| transition.call(SpaceActionStatus::Designing),
                        "{tr.activate}"
                    }
                },
                Some(SpaceActionStatus::Designing) => rsx! {
                    Button {
                        size: ButtonSize::Small,
                        style: ButtonStyle::Primary,
                        disabled: saving(),
                        "data-testid": "action-publish",
                        onclick: move |_| transition.call(SpaceActionStatus::Ongoing),
                        "{tr.publish}"
                    }
                },
                Some(SpaceActionStatus::Ongoing) => rsx! {
                    Button {
                        size: ButtonSize::Small,
                        style: ButtonStyle::Outline,
                        disabled: saving(),
                        "data-testid": "action-close",
                        onclick: move |_| transition.call(SpaceActionStatus::Finish),
                        "{tr.close}"
                    }
                },
                Some(SpaceActionStatus::Finish) => rsx! {
                    Button { size: ButtonSize::Small, style: ButtonStyle::Text, disabled: true, "{tr.closed}" }
                },
            }
        }
    }
}

translate! {
    ActionStatusControlTranslate;

    status_legacy: { en: "Legacy (not visible to participants)", ko: "이전 버전 (참가자에게 보이지 않음)" },
    status_designing: { en: "Designing — not published yet", ko: "설계중 — 아직 공개되지 않음" },
    status_ongoing: { en: "Ongoing — accepting responses", ko: "진행중 — 응답 수신 중" },
    status_finish: { en: "Finished — closed", ko: "종료됨" },
    activate: { en: "Activate", ko: "활성화" },
    publish: { en: "Publish", ko: "공개" },
    close: { en: "Close", ko: "종료" },
    closed: { en: "Closed", ko: "종료됨" },
}
