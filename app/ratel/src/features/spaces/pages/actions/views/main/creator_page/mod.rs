use super::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use i18n::CreatorActionPageTranslate;

mod i18n;

#[component]
pub fn CreatorActionPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: CreatorActionPageTranslate = use_translate();
    let mut layover = use_layover();
    let mut actions = use_loader(move || async move { list_actions(space_id()).await })?;
    let mut preview_mode = use_signal(|| false);

    // When preview mode is on, render the participant page instead
    if preview_mode() {
        return rsx! {
            div {
                id: "creator-action-page",
                class: "flex flex-col gap-5 items-start w-full text-web-font-primary",

                div { class: "flex flex-col gap-2.5 mx-auto w-full max-w-[1024px]",
                    // Header with back-to-editor button
                    Row {
                        main_axis_align: MainAxisAlign::Between,
                        cross_axis_align: CrossAxisAlign::Center,
                        class: "w-full",
                        h3 { {tr.title} }
                        Button {
                            size: ButtonSize::Medium,
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Rounded,
                            onclick: move |_| preview_mode.set(false),
                            Row {
                                cross_axis_align: CrossAxisAlign::Center,
                                class: "gap-2",
                                lucide_dioxus::ArrowLeft { class: "w-4 h-4" }
                                span { {tr.back_to_editor} }
                            }
                        }
                    }

                    ParticipantPage { space_id }
                }
            }
        };
    }

    rsx! {
        div {
            id: "creator-action-page",
            class: "flex flex-col gap-5 items-start w-full text-web-font-primary",

            div { class: "flex flex-col gap-2.5 mx-auto w-full max-w-[1024px]",
                div { class: "flex justify-between items-center w-full max-mobile:flex-col max-mobile:items-stretch max-mobile:gap-3",
                    div { class: "flex gap-2 items-center",
                        h3 { {tr.title} }
                        Tooltip {
                            TooltipTrigger {
                                icons::help_support::Info {
                                    width: "16",
                                    height: "16",
                                    class: "w-4 h-4 cursor-pointer [&>path]:stroke-text-secondary [&>path]:fill-none [&>circle]:stroke-text-secondary [&>circle]:fill-current",
                                }
                            }
                            TooltipContent {
                                side: ContentSide::Bottom,
                                align: ContentAlign::Start,
                                {tr.title_tooltip}
                            }
                        }
                    }

                    Row { class: "gap-2 max-mobile:flex-col",
                        // Preview as Participant button
                        Button {
                            size: ButtonSize::Medium,
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            onclick: move |_| preview_mode.set(true),
                            Row {
                                cross_axis_align: CrossAxisAlign::Center,
                                class: "gap-2",
                                lucide_dioxus::Eye { class: "w-4 h-4" }
                                span { {tr.preview_participant} }
                            }
                        }

                        Button {
                            size: ButtonSize::Medium,
                            style: ButtonStyle::Secondary,
                            shape: ButtonShape::Square,
                            class: "inline-flex hover:opacity-90 border-action-type-card-border font-raleway max-mobile:w-full bg-action-type-card-bg text-btn-action-settings-text hover:border-action-type-card-border hover:bg-action-type-card-bg hover:text-btn-action-settings-text",
                            onclick: move |_| {
                                layover
                                    .open(
                                        "space-action-settings-layover".to_string(),
                                        String::new(),
                                        rsx! {
                                            ActionSettingsModal {
                                                space_id: space_id(),
                                                actions: actions(),
                                                on_applied: move |_| {
                                                    actions.restart();
                                                },
                                            }
                                        },
                                    )
                                    .set_size(LayoverSize::Small);
                            },
                            div { class: "flex flex-row gap-2.5 justify-center items-center",
                                icons::settings::Settings2 {
                                    width: "16",
                                    height: "16",
                                    class: "[&>path]:fill-current [&>circle]:stroke-current [&>circle]:fill-none",
                                }
                                span { {tr.button_settings_label} }
                            }
                        }
                    }
                }

                // Chapter Editor (replaces old action grid)
                ChapterEditor { space_id }
            }
        }
    }
}
