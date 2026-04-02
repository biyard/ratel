use crate::common::*;
use crate::features::ai_moderator::controllers::*;
use dioxus_primitives::{ContentAlign, ContentSide};

use super::MaterialList;

#[component]
pub fn AiModeratorSetting(
    space_id: ReadSignal<SpacePartition>,
    discussion_sk: String,
) -> Element {
    let tr: AiModeratorSettingTranslate = use_translate();
    let mut toast = use_toast();
    let disc_sk = discussion_sk.clone();

    let config_res = use_server_future(move || {
        let disc_sk = disc_sk.clone();
        async move { get_ai_moderator_config(space_id(), disc_sk).await }
    })?;

    let config = config_res
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let AiModeratorConfigResponse {
        enabled: init_enabled,
        reply_interval: init_interval,
        guidelines: init_guidelines,
    } = config;

    let mut enabled = use_signal(move || init_enabled);
    let mut reply_interval = use_signal(move || init_interval);
    let mut guidelines = use_signal({
        let init_guidelines = init_guidelines.clone();
        move || init_guidelines
    });
    let mut save_version = use_signal(|| 0u64);

    let mut last_synced = use_signal(move || (init_enabled, init_interval, init_guidelines));
    use_effect(move || {
        let current = config_res
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .cloned()
            .unwrap_or_default();
        let latest = (
            current.enabled,
            current.reply_interval,
            current.guidelines.clone(),
        );
        if last_synced() != latest {
            last_synced.set(latest.clone());
            enabled.set(latest.0);
            reply_interval.set(latest.1);
            guidelines.set(latest.2);
        }
    });

    // Debounced autosave (3 seconds after last change)
    use_effect({
        let discussion_sk = discussion_sk.clone();
        move || {
            let version = save_version();
            if version == 0 {
                return;
            }
            let discussion_sk = discussion_sk.clone();
            spawn(async move {
                crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;
                if save_version() != version {
                    return;
                }
                let req = UpdateAiModeratorConfigRequest {
                    enabled: enabled(),
                    reply_interval: reply_interval(),
                    guidelines: guidelines(),
                };
                match update_ai_moderator_config(space_id(), discussion_sk, req).await {
                    Ok(_) => {
                        toast.info(tr.config_saved.to_string());
                    }
                    Err(e) => {
                        toast.error(e);
                    }
                }
            });
        }
    });

    let label_class =
        "font-semibold font-raleway text-[13px]/[16px] tracking-[-0.14px] text-web-font-neutral";

    let toggle_disc_sk = discussion_sk.clone();
    let material_disc_sk = discussion_sk.clone();

    rsx! {
        Collapsible { open: enabled(),
            CollapsibleTrigger {
                r#as: move |_attrs: Vec<Attribute>| {
                    rsx! {
                        Card {
                            direction: CardDirection::Row,
                            main_axis_align: MainAxisAlign::Between,
                            cross_axis_align: CrossAxisAlign::Center,
                            class: if enabled() { "!rounded-b-none" } else { "" },
                            div { class: "flex gap-1 items-center",
                                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                                    {tr.ai_moderator}
                                }
                                Tooltip {
                                    TooltipTrigger {
                                        icons::help_support::Info {
                                            width: "14",
                                            height: "14",
                                            class: "cursor-help text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                                        }
                                    }
                                    TooltipContent {
                                        side: ContentSide::Bottom,
                                        align: ContentAlign::Start,
                                        {tr.ai_moderator_tooltip}
                                    }
                                }
                            }
                            div { "data-testid": "ai-moderator-toggle",
                                PremiumSwitch {
                                    active: enabled(),
                                    on_toggle: {
                                        let discussion_sk = toggle_disc_sk.clone();
                                        move |_| {
                                            let new_enabled = !enabled();
                                            enabled.set(new_enabled);
                                            let discussion_sk = discussion_sk.clone();
                                            spawn(async move {
                                                let req = UpdateAiModeratorConfigRequest {
                                                    enabled: new_enabled,
                                                    reply_interval: reply_interval(),
                                                    guidelines: guidelines(),
                                                };
                                                match update_ai_moderator_config(
                                                    space_id(),
                                                    discussion_sk,
                                                    req,
                                                )
                                                .await
                                                {
                                                    Ok(_) => {
                                                        toast.info(tr.config_saved.to_string());
                                                    }
                                                    Err(e) => {
                                                        toast.error(e);
                                                    }
                                                }
                                            });
                                        }
                                    },
                                }
                            }
                        }
                    }
                },
            }
            CollapsibleContent {
                Card { class: "gap-4 w-full rounded-t-none!",
                    // Reply Interval
                    div { class: "flex flex-col gap-2.5 w-full",
                        p { class: label_class, {tr.reply_interval_label} }
                        div { class: "flex gap-3 items-center",
                            Input {
                                r#type: InputType::Number,
                                "data-testid": "ai-moderator-reply-interval",
                                class: "font-semibold text-right !w-[120px] font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                                value: "{reply_interval()}",
                                oninput: move |evt: FormEvent| {
                                    let val = evt.value().parse::<i64>().unwrap_or(1).max(1);
                                    reply_interval.set(val);
                                    save_version += 1;
                                },
                            }
                            span { class: "text-[13px] text-foreground-muted",
                                {tr.reply_interval_suffix}
                            }
                        }
                    }

                    // Moderation Guidelines
                    div { class: "flex flex-col gap-2.5 w-full",
                        p { class: label_class, {tr.guidelines_label} }
                        TextArea {
                            "data-testid": "ai-moderator-guidelines",
                            class: "min-h-[100px] w-full",
                            placeholder: tr.guidelines_placeholder,
                            value: guidelines(),
                            oninput: move |evt: FormEvent| {
                                guidelines.set(evt.value());
                                save_version += 1;
                            },
                        }
                    }

                    // Reference Materials
                    MaterialList {
                        space_id,
                        discussion_sk: material_disc_sk.clone(),
                    }

                    // Info notice
                    div { class: "flex gap-5 items-start p-5 w-full border rounded-[12px] border-web-card-stroke3 bg-web-card-bg2",
                        icons::help_support::Info {
                            width: "18",
                            height: "18",
                            class: "mt-0.5 shrink-0 text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                        }
                        div { class: "flex flex-col flex-1 gap-2 min-w-0",
                            p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                                {tr.how_it_works}
                            }
                            ul { class: "pl-4 font-medium list-disc text-[13px]/[20px] font-raleway text-web-font-body",
                                li { {tr.info_line_one} }
                                li { {tr.info_line_two} }
                                li { {tr.info_line_three} }
                            }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    AiModeratorSettingTranslate;

    ai_moderator: { en: "AI Moderator", ko: "AI 중재자" },
    ai_moderator_tooltip: {
        en: "Enable an AI moderator to automatically summarize and respond to discussion replies at set intervals.",
        ko: "AI 중재자를 활성화하면 설정된 간격마다 자동으로 토론 답변을 요약하고 응답합니다.",
    },
    reply_interval_label: { en: "Reply Interval", ko: "답변 간격" },
    reply_interval_suffix: { en: "replies between each AI response", ko: "개의 답변마다 AI 응답" },
    guidelines_label: { en: "Moderation Guidelines", ko: "중재 가이드라인" },
    guidelines_placeholder: {
        en: "e.g., Summarize all replies in under 200 words. Focus on balancing positive and negative opinions.",
        ko: "예: 모든 답변을 200자 이내로 요약하세요. 긍정적 의견과 부정적 의견의 균형에 초점을 맞추세요.",
    },
    config_saved: { en: "AI moderator settings saved.", ko: "AI 중재자 설정이 저장되었습니다." },
    how_it_works: { en: "How AI Moderator Works", ko: "AI 중재자 작동 방식" },
    info_line_one: {
        en: "The AI moderator automatically generates a response after the set number of replies.",
        ko: "AI 중재자는 설정된 답변 수에 도달하면 자동으로 응답을 생성합니다.",
    },
    info_line_two: {
        en: "Use moderation guidelines to instruct how the AI should summarize and moderate.",
        ko: "중재 가이드라인을 사용하여 AI가 요약하고 중재하는 방법을 지시하세요.",
    },
    info_line_three: {
        en: "Upload reference materials (PDF) to provide additional context for better AI responses.",
        ko: "참고 자료(PDF)를 업로드하면 더 나은 AI 응답을 위한 추가 컨텍스트를 제공합니다.",
    },
}
