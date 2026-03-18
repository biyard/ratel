use crate::features::spaces::pages::apps::apps::panels::*;
use dioxus_primitives::{ContentAlign, ContentSide};

translate! {
    CollectivePanelTranslate;

    collective_title: {
        en: "Collective Panel Attributes",
        ko: "Collective 패널 속성",
    },
    collective_desc: {
        en: "Users must have these attributes to participate.",
        ko: "참여하기 위해서는 해당 속성을 가지고 있어야 합니다.",
    },
    no_attributes: {
        en: "No attributes selected. Toggle attributes above to add.",
        ko: "선택된 속성이 없습니다. 위에서 속성을 토글하세요.",
    },
    move_to_conditional: {
        en: "Move to conditional",
        ko: "Conditional로 이동",
    },
    university: {
        en: "University",
        ko: "대학교",
    },
    age: {
        en: "Age",
        ko: "나이",
    },
    gender: {
        en: "Gender",
        ko: "성별",
    },
}

#[component]
pub fn CollectivePanel(
    space_id: ReadSignal<SpacePartition>,
    panels: Vec<SpacePanelQuotaResponse>,
    current_quota: i64,
    panels_query_key: Vec<String>,
) -> Element {
    let tr: CollectivePanelTranslate = use_translate();
    let mut toast = use_toast();
    let mut query = use_query_store();
    let mut show_menu = use_signal(|| false);

    let has_university_collective = is_collective_option(PanelOption::University, &panels);
    let has_age_collective = is_collective_option(PanelOption::Age, &panels);
    let has_gender_collective = is_collective_option(PanelOption::Gender, &panels);

    let has_any_collective =
        has_university_collective || has_age_collective || has_gender_collective;

    let can_move_age = has_age_collective && current_quota > 0;
    let can_move_gender = has_gender_collective && current_quota > 0;
    let has_movable = can_move_age || can_move_gender;

    let is_age_conditional = is_conditional_option(PanelOption::Age, &panels);
    let is_gender_conditional = is_conditional_option(PanelOption::Gender, &panels);

    let move_to_conditional = {
        move |option: PanelOption| {
            let panels = panels.clone();
            let panels_query_key = panels_query_key.clone();
            let mut toast = toast;
            let mut query = query;

            move |_: MouseEvent| {
                let panels = panels.clone();
                let panels_query_key = panels_query_key.clone();
                show_menu.set(false);

                let will_age_be_conditional = option == PanelOption::Age || is_age_conditional;
                let will_gender_be_conditional =
                    option == PanelOption::Gender || is_gender_conditional;

                let mut keys = option_keys(option, &panels);
                if will_age_be_conditional && will_gender_be_conditional {
                    let other = if option == PanelOption::Age {
                        PanelOption::Gender
                    } else {
                        PanelOption::Age
                    };
                    keys.extend(option_keys(other, &panels));
                    let mut seen = std::collections::HashSet::new();
                    keys.retain(|k| seen.insert(k.panel_id.clone()));
                }

                let groups = if will_age_be_conditional && will_gender_be_conditional {
                    build_conditional_groups(PanelOption::Age, current_quota, true)
                } else {
                    build_conditional_groups(option, current_quota, false)
                };

                spawn(async move {
                    match rebuild_panels(space_id(), keys, groups).await {
                        Ok(_) => query.invalidate(&panels_query_key),
                        Err(err) => {
                            error!("Failed to move to conditional: {:?}", err);
                            toast.error(err);
                        }
                    }
                });
            }
        }
    };

    rsx! {
        SpaceCard { class: "!p-6".to_string(),
            div { class: "flex flex-col gap-4 min-w-0",
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center gap-2",
                        h2 { class: "text-lg font-semibold text-panel-title", {tr.collective_title} }
                        Tooltip {
                            TooltipTrigger {
                                icons::help_support::Info {
                                    width: "16",
                                    height: "16",
                                    class: "h-4 w-4 [&>path]:stroke-text-secondary [&>circle]:fill-text-secondary cursor-help",
                                }
                            }
                            TooltipContent {
                                side: ContentSide::Bottom,
                                align: ContentAlign::Start,
                                {tr.collective_desc}
                            }
                        }
                    }
                    if has_movable {
                        div { class: "relative",
                            Button {
                                size: ButtonSize::Icon,
                                style: ButtonStyle::Outline,
                                shape: ButtonShape::Square,
                                class: "size-8 !p-0 flex items-center justify-center".to_string(),
                                onclick: move |_| {
                                    show_menu.set(!show_menu());
                                },
                                icons::validations::Add { width: "16", height: "16", class: "h-4 w-4 [&>path]:stroke-current" }
                            }
                            if show_menu() {
                                div { class: "absolute right-0 top-10 z-10 flex flex-col gap-1 p-2 min-w-[120px] rounded-lg border border-input-box-border bg-popover shadow-lg",
                                    if can_move_age {
                                        button {
                                            class: "px-3 py-1.5 text-sm text-left rounded hover:bg-hover text-text-primary",
                                            onclick: move_to_conditional(PanelOption::Age),
                                            {tr.age}
                                        }
                                    }
                                    if can_move_gender {
                                        button {
                                            class: "px-3 py-1.5 text-sm text-left rounded hover:bg-hover text-text-primary",
                                            onclick: move_to_conditional(PanelOption::Gender),
                                            {tr.gender}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if has_any_collective {
                    div { class: "flex flex-wrap gap-2",
                        if has_university_collective {
                            Badge {
                                color: BadgeColor::Blue,
                                size: BadgeSize::Normal,
                                {tr.university}
                            }
                        }
                        if has_age_collective {
                            Badge {
                                color: BadgeColor::Green,
                                size: BadgeSize::Normal,
                                {tr.age}
                            }
                        }
                        if has_gender_collective {
                            Badge {
                                color: BadgeColor::Purple,
                                size: BadgeSize::Normal,
                                {tr.gender}
                            }
                        }
                    }
                } else {
                    p { class: "text-sm text-text-secondary py-4 text-center", {tr.no_attributes} }
                }
            }
        }
    }
}
