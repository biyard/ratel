use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;

#[component]
pub fn ActivityScoreSetting(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    action_setting: ReadSignal<crate::features::spaces::pages::actions::models::SpaceAction>,
) -> Element {
    let tr: ActivityTranslate = use_translate();
    let mut toast = crate::common::use_toast();
    let setting = action_setting();
    let show_additional = setting.space_action_type
        != crate::features::spaces::pages::actions::types::SpaceActionType::Follow;
    let mut current_activity_score = use_signal(move || action_setting().activity_score);
    let mut current_additional_score = use_signal(move || action_setting().additional_score);

    let save_scores = move || {
        spawn(async move {
            let req = crate::features::spaces::pages::actions::controllers::UpdateSpaceActionRequest::ActivityScore {
                activity_score: current_activity_score(),
                additional_score: current_additional_score(),
            };
            match crate::features::spaces::pages::actions::controllers::update_space_action(
                space_id(),
                action_id(),
                req,
            )
            .await
            {
                Ok(_) => {
                    toast.info(tr.activity_score_updated.to_string());
                }
                Err(e) => {
                    toast.error(e);
                }
            }
        });
    };

    rsx! {
        Collapsible {
            CollapsibleTrigger {
                r#as: move |_attrs: Vec<Attribute>| {
                    rsx! {
                        Card {
                            variant: CardVariant::Outlined,
                            class: "cursor-pointer",
                            Row {
                                class: "w-full p-4",
                                main_axis_align: MainAxisAlign::Between,
                                cross_axis_align: CrossAxisAlign::Center,
                                span {
                                    class: "text-sm font-semibold text-text-primary",
                                    "{tr.activity_score}"
                                }
                                lucide_dioxus::ChevronDown {
                                    class: "w-4 h-4 text-foreground-muted",
                                }
                            }
                        }
                    }
                },
            }
            CollapsibleContent {
                Card {
                    variant: CardVariant::Outlined,
                    class: "rounded-t-none border-t-0 p-4",
                    Col {
                        class: "gap-4 w-full",

                        Col {
                            class: "gap-2",
                            p {
                                class: "text-sm font-medium text-text-primary",
                                "{tr.activity_score}"
                            }
                            Input {
                                r#type: InputType::Number,
                                value: "{current_activity_score()}",
                                oninput: move |e: FormEvent| {
                                    if let Ok(v) = e.value().parse::<i64>() {
                                        current_activity_score.set(v);
                                    }
                                },
                                onconfirm: move |_: KeyboardEvent| {
                                    save_scores();
                                },
                            }
                        }

                        if show_additional {
                            Col {
                                class: "gap-2",
                                p {
                                    class: "text-sm font-medium text-text-primary",
                                    "{tr.additional_score}"
                                }
                                p {
                                    class: "text-xs text-foreground-muted",
                                    "{tr.additional_score_desc}"
                                }
                                Input {
                                    r#type: InputType::Number,
                                    value: "{current_additional_score()}",
                                    oninput: move |e: FormEvent| {
                                        if let Ok(v) = e.value().parse::<i64>() {
                                            current_additional_score.set(v);
                                        }
                                    },
                                    onconfirm: move |_: KeyboardEvent| {
                                        save_scores();
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
