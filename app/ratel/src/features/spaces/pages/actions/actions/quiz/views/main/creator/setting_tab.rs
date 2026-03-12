use super::*;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

#[component]
pub fn SettingTab(can_edit: bool, #[props(default = true)] show_back: bool) -> Element {
    let ctx = use_space_quiz_context();
    let current_section = use_signal(|| QuizCreatorSection::Setting);

    rsx! {
        SettingContent {
            space_id: ctx.space_id,
            quiz_id: ctx.quiz_id,
            can_edit,
            started_at: ctx.started_at,
            ended_at: ctx.ended_at,
            retry_count: ctx.retry_count,
            pass_score: ctx.pass_score,
            current_section,
            show_back,
        }
    }
}

#[component]
pub fn SettingContent(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
    can_edit: bool,
    started_at: Signal<i64>,
    ended_at: Signal<i64>,
    retry_count: Signal<i64>,
    pass_score: Signal<i64>,
    current_section: Signal<QuizCreatorSection>,
    #[props(default = true)] show_back: bool,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let nav = navigator();
    let mut toast = use_toast();

    let on_time_change = move |(start, end): (i64, i64)| {
        started_at.set(start);
        ended_at.set(end);
        let mut toast = toast;
        spawn(async move {
            let req = UpdateQuizRequest {
                started_at: Some(start),
                ended_at: Some(end),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to update time range: {:?}", err);
                toast.error(err);
            } else {
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                invalidate_query(&keys);
            }
        });
    };

    let on_pass_score_save = move |_| {
        let mut toast = toast;
        spawn(async move {
            let req = UpdateQuizRequest {
                pass_score: Some(pass_score()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to update pass score: {:?}", err);
                toast.error(err);
            } else {
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                invalidate_query(&keys);
            }
        });
    };

    let on_retry_save = move |_| {
        let mut toast = toast;
        spawn(async move {
            let req = UpdateQuizRequest {
                retry_count: Some(retry_count()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to update retry count: {:?}", err);
                toast.error(err);
            } else {
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                invalidate_query(&keys);
            }
        });
    };

    rsx! {
        div { class: "flex w-full max-w-[1024px] flex-col gap-6",
            div { class: "flex flex-col gap-1",
                h3 { class: "text-[24px]/[28px] font-bold tracking-[-0.24px] text-white",
                    {tr.setting_section_title}
                }
                p { class: "text-[15px]/[22px] font-medium text-[#D4D4D4]",
                    {tr.setting_section_description}
                }
            }

            if can_edit {
                TimeRangeSetting {
                    started_at: started_at(),
                    ended_at: ended_at(),
                    on_change: on_time_change,
                }
            } else {
                TimeRangeDisplay { started_at: started_at(), ended_at: ended_at() }
            }

            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400", "{tr.pass_score_label}" }
                Input {
                    r#type: InputType::Number,
                    class: "text-base",
                    placeholder: tr.pass_score_placeholder,
                    value: pass_score().to_string(),
                    disabled: !can_edit,
                    attributes: vec![Attribute::new("min", "0", None, false)],
                    oninput: move |e: Event<FormData>| {
                        if let Ok(v) = e.value().parse::<i64>() {
                            pass_score.set(v);
                        }
                    },
                    onblur: on_pass_score_save,
                }
            }

            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400", "{tr.retry_label}" }
                Input {
                    r#type: InputType::Number,
                    class: "text-base",
                    placeholder: tr.retry_placeholder,
                    value: retry_count().to_string(),
                    disabled: !can_edit,
                    attributes: vec![Attribute::new("min", "0", None, false)],
                    oninput: move |e: Event<FormData>| {
                        if let Ok(v) = e.value().parse::<i64>() {
                            retry_count.set(v);
                        }
                    },
                    onblur: on_retry_save,
                }
            }

            div { class: "flex w-full justify-end gap-3",
                if show_back {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "min-w-[110px]",
                        onclick: move |_| current_section.set(QuizCreatorSection::Quiz),
                        {tr.btn_back}
                    }
                }
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "min-w-[140px]",
                    onclick: move |_| nav.go_back(),
                    {tr.btn_done}
                }
            }
        }
    }
}
