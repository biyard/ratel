use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParticipationTagKind {
    Warning,
    Success,
}

#[component]
pub fn ParticipationCard(
    space_id: SpacePartition,
    credential_path: Option<String>,
    on_login: EventHandler<()>,
) -> Element {
    let tr: ParticipationCardTranslate = use_translate();
    let navigator = use_navigator();
    let mut participate =
        use_action(crate::features::spaces::controllers::participate_space::participate_space);
    let participate_credential_path = credential_path.clone();
    let credential_button_path = credential_path.clone();
    let login_for_participate = on_login.clone();
    let login_for_credentials = on_login;

    let handle_participate = move |_| {
        let space_id = space_id.clone();

        if participate_credential_path.is_none() {
            login_for_participate.call(());
            return;
        }

        spawn(async move {
            let space_detail = crate::features::spaces::space_common::types::space_key(&space_id);
            participate.call(space_id).await;
            invalidate_query(&space_detail);
        });
    };

    let handle_open_credentials = move |_| {
        if let Some(path) = &credential_button_path {
            navigator.push(path.clone());
        } else {
            login_for_credentials.call(());
        }
    };

    rsx! {
        div { class: "px-4 w-full",
            div { class: "flex flex-col items-start gap-2.5 px-3 py-4 w-full rounded-[12px] border border-primary bg-primary/5",
                div { class: "flex flex-col items-start gap-2.5 w-full",
                    div { class: "flex flex-col items-start gap-1 w-full",
                        p { class: "w-full font-bold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-white",
                            {tr.title}
                        }
                        p { class: "w-full font-medium font-raleway text-[13px]/[20px] text-gray-300",
                            {tr.description}
                        }
                    }

                    div { class: "flex items-center gap-1",
                        ParticipationRequirementTag {
                            kind: ParticipationTagKind::Warning,
                            label: tr.age.to_string(),
                        }
                        ParticipationRequirementTag {
                            kind: ParticipationTagKind::Success,
                            label: tr.country.to_string(),
                        }
                    }
                }

                div { class: "flex flex-col items-start gap-2.5 w-full",
                    Button {
                        class: "w-full",
                        style: ButtonStyle::Primary,
                        size: ButtonSize::Small,
                        onclick: handle_participate,
                        {tr.participate}
                    }
                    Button {
                        class: "w-full",
                        style: ButtonStyle::Outline,
                        size: ButtonSize::Small,
                        onclick: handle_open_credentials,
                        {tr.see_my_credential}
                    }
                }
            }
        }
    }
}

#[component]
fn ParticipationRequirementTag(kind: ParticipationTagKind, label: String) -> Element {
    let (container_class, icon_class, text_class) = match kind {
        ParticipationTagKind::Warning => (
            "border-red-500 bg-red-500/5",
            "border-red-500 text-red-500",
            "text-white",
        ),
        ParticipationTagKind::Success => (
            "border-green-500 bg-green-500/5",
            "border-green-500 text-green-500",
            "text-white",
        ),
    };

    rsx! {
        div { class: "flex items-center justify-center px-[8px] py-[5.5px] rounded-full border {container_class}",
            div { class: "flex items-center gap-1",
                div { class: "flex items-center justify-center size-[14px] {icon_class}",
                    if kind == ParticipationTagKind::Warning {
                        crate::common::lucide_dioxus::CircleAlert { size: 15, class: "text-red-500" }
                    } else {
                        icons::validations::Check { class: "w-[15px] h-[15px] [&>path]:stroke-green-500" }
                    }
                }
                span { class: "font-bold font-raleway text-[12px]/[14px] tracking-[-0.12px] {text_class}",
                    {label}
                }
            }
        }
    }
}

translate! {
    ParticipationCardTranslate;

    title: {
        en: "Participation",
        ko: "참여",
    },

    description: {
        en: "You can read everything, but posting, voting and commenting require verification.",
        ko: "모든 내용을 읽을 수 있지만, 게시, 투표, 댓글은 검증이 필요합니다.",
    },

    age: {
        en: "Age",
        ko: "나이",
    },

    country: {
        en: "Country",
        ko: "국가",
    },

    participate: {
        en: "Participate",
        ko: "참여하기",
    },

    see_my_credential: {
        en: "See My Credential",
        ko: "내 Credential 보기",
    },
}
