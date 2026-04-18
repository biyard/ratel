use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::actions::poll::views::main::creator::PollCreatorTranslate;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SaveStatus {
    Idle,
    Saving,
    Saved,
    Unsaved,
}

fn qtype_str(q: &Question) -> &'static str {
    match q {
        Question::SingleChoice(_) => "single",
        Question::MultipleChoice(_) => "multi",
        Question::Subjective(_) | Question::ShortAnswer(_) => "subjective",
        Question::LinearScale(_) => "linear",
        Question::Checkbox(_) => "multi",
        Question::Dropdown(_) => "single",
    }
}

fn options_of(q: &Question) -> Vec<String> {
    match q {
        Question::SingleChoice(cq) | Question::MultipleChoice(cq) => cq.options.clone(),
        Question::Checkbox(c) => c.options.clone(),
        Question::Dropdown(d) => d.options.clone(),
        _ => Vec::new(),
    }
}

fn title_of(q: &Question) -> String {
    q.title().to_string()
}

fn set_title(q: &mut Question, value: String) {
    match q {
        Question::SingleChoice(cq) | Question::MultipleChoice(cq) => cq.title = value,
        Question::Subjective(s) | Question::ShortAnswer(s) => s.title = value,
        Question::Checkbox(c) => c.title = value,
        Question::Dropdown(d) => d.title = value,
        Question::LinearScale(l) => l.title = value,
    }
}

fn convert_to_qtype(existing: &Question, target: &str) -> Question {
    let title = title_of(existing);
    let options = options_of(existing);
    match target {
        "single" => Question::SingleChoice(ChoiceQuestion {
            title,
            options: if options.is_empty() {
                vec![String::new(), String::new()]
            } else {
                options
            },
            ..Default::default()
        }),
        "multi" => Question::MultipleChoice(ChoiceQuestion {
            title,
            options: if options.is_empty() {
                vec![String::new(), String::new()]
            } else {
                options
            },
            ..Default::default()
        }),
        "subjective" => Question::Subjective(SubjectiveQuestion {
            title,
            ..Default::default()
        }),
        "linear" => Question::LinearScale(LinearScaleQuestion {
            title,
            min_value: 1,
            max_value: 5,
            ..Default::default()
        }),
        _ => existing.clone(),
    }
}

#[component]
pub fn ContentCard() -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let mut ctx = use_space_poll_context();
    let mut toast = use_toast();

    let space_id = ctx.space_id;
    let poll_id = ctx.poll_id;

    // ── Title state (autosave) ─────
    let initial_title = ctx.poll.read().title.clone();
    let mut title = use_signal(|| initial_title.clone());
    let mut last_saved_title = use_signal(|| initial_title);
    let mut title_version = use_signal(|| 0u64);
    let mut title_status = use_signal(|| SaveStatus::Idle);

    // ── Questions state ─────
    let initial_questions = ctx.poll.read().questions.clone();
    let mut questions = use_signal(|| initial_questions);

    let mut save_title = move || {
        let current = title();
        if current == last_saved_title() {
            return;
        }
        title_status.set(SaveStatus::Saving);
        spawn(async move {
            let req = UpdatePollRequest::Title {
                title: current.clone(),
            };
            if let Err(err) = update_poll(space_id(), poll_id(), req).await {
                error!("Failed to save title: {:?}", err);
                title_status.set(SaveStatus::Unsaved);
                toast.error(err);
            } else {
                last_saved_title.set(current);
                title_status.set(SaveStatus::Saved);
                ctx.poll.restart();
            }
        });
    };

    let save_questions = move || {
        spawn(async move {
            let req = UpdatePollRequest::Question {
                questions: questions(),
            };
            if let Err(err) = update_poll(space_id(), poll_id(), req).await {
                error!("Failed to save poll questions: {:?}", err);
                toast.error(err);
            }
        });
    };

    // Autosave title — 3-second debounce.
    use_effect(move || {
        let version = title_version();
        if version == 0 {
            return;
        }
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;
            if title_version() != version {
                return;
            }
            if title() == last_saved_title() {
                return;
            }
            save_title();
        });
    });

    let qs = questions.read().clone();

    rsx! {
        section { class: "pager__page", "data-page": "0",
            article { class: "page-card", "data-testid": "page-card-content",
                header { class: "page-card__head",
                    div { class: "page-card__title-wrap",
                        span { class: "page-card__num", "{tr.card_index_1}" }
                        div {
                            h1 { class: "page-card__title", "{tr.card_content_title}" }
                            div { class: "page-card__subtitle", "{tr.card_content_subtitle}" }
                        }
                    }
                }

                // ── Title section ─────
                section { class: "section", "data-testid": "section-content",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_content_label}" }
                    }
                    div { class: "field",
                        div {
                            style: "display:flex;align-items:center;justify-content:space-between;gap:8px",
                            label { class: "field__label", "{tr.title_label}" }
                            AutosaveStatusBadge { status: title_status() }
                        }
                        input {
                            class: "input",
                            "data-testid": "poll-title",
                            placeholder: "{tr.title_placeholder}",
                            value: "{title()}",
                            oninput: move |e| {
                                title.set(e.value());
                                title_status.set(SaveStatus::Unsaved);
                                title_version.set(title_version() + 1);
                            },
                            onblur: move |_| save_title(),
                        }
                    }
                }

                // ── Questions section ─────
                section { class: "section", "data-testid": "section-questions",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_questions_label}" }
                        span { class: "section__hint", "{tr.section_questions_hint}" }
                    }

                    for (idx , question) in qs.iter().enumerate() {
                        QuestionBlock {
                            key: "q-{idx}",
                            idx,
                            question: question.clone(),
                            on_title_change: move |(i, title): (usize, String)| {
                                let mut qs = questions.write();
                                if let Some(q) = qs.get_mut(i) {
                                    set_title(q, title);
                                }
                            },
                            on_type_change: move |(i, target): (usize, String)| {
                                let new_q = {
                                    let qs = questions.read();
                                    if let Some(existing) = qs.get(i) {
                                        convert_to_qtype(existing, &target)
                                    } else {
                                        return;
                                    }
                                };
                                {
                                    let mut qs = questions.write();
                                    if let Some(q) = qs.get_mut(i) {
                                        *q = new_q;
                                    }
                                }
                                save_questions();
                            },
                            on_option_change: move |(i, opt_idx, text): (usize, usize, String)| {
                                let mut qs = questions.write();
                                if let Some(q) = qs.get_mut(i) {
                                    match q {
                                        Question::SingleChoice(cq) | Question::MultipleChoice(cq) => {
                                            if let Some(o) = cq.options.get_mut(opt_idx) {
                                                *o = text;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            },
                            on_remove: move |i: usize| {
                                {
                                    let mut qs = questions.write();
                                    if i < qs.len() {
                                        qs.remove(i);
                                    }
                                }
                                save_questions();
                            },
                            on_subjective_change: move |(i, text): (usize, String)| {
                                let mut qs = questions.write();
                                if let Some(q) = qs.get_mut(i) {
                                    if let Question::Subjective(s) = q {
                                        s.description = text;
                                    } else if let Question::ShortAnswer(s) = q {
                                        s.description = text;
                                    }
                                }
                            },
                            on_linear_change: move |(i, min, max): (usize, i64, i64)| {
                                let mut qs = questions.write();
                                if let Some(q) = qs.get_mut(i) {
                                    if let Question::LinearScale(l) = q {
                                        l.min_value = min;
                                        l.max_value = max;
                                    }
                                }
                            },
                            on_blur_save: move |_| save_questions(),
                        }
                    }

                    button {
                        class: "add-btn",
                        r#type: "button",
                        "data-testid": "poll-question-add",
                        onclick: move |_| {
                            let new_q = Question::SingleChoice(ChoiceQuestion {
                                title: String::new(),
                                options: vec![String::new(), String::new()],
                                ..Default::default()
                            });
                            questions.write().push(new_q);
                            save_questions();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line {
                                x1: "12",
                                y1: "5",
                                x2: "12",
                                y2: "19",
                            }
                            line {
                                x1: "5",
                                y1: "12",
                                x2: "19",
                                y2: "12",
                            }
                        }
                        "{tr.add_question}"
                    }
                }
            }
        }
    }
}

#[component]
fn AutosaveStatusBadge(status: SaveStatus) -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let (label, modifier) = match status {
        SaveStatus::Idle => return rsx! {},
        SaveStatus::Saving => (tr.autosave_saving.to_string(), "autosave--saving"),
        SaveStatus::Saved => (tr.autosave_saved.to_string(), "autosave--saved"),
        SaveStatus::Unsaved => (tr.autosave_unsaved.to_string(), "autosave--unsaved"),
    };
    rsx! {
        span { class: "autosave {modifier}",
            span { class: "autosave__dot" }
            "{label}"
        }
    }
}

#[component]
fn QuestionBlock(
    idx: usize,
    question: Question,
    on_title_change: EventHandler<(usize, String)>,
    on_type_change: EventHandler<(usize, String)>,
    on_option_change: EventHandler<(usize, usize, String)>,
    on_remove: EventHandler<usize>,
    on_subjective_change: EventHandler<(usize, String)>,
    on_linear_change: EventHandler<(usize, i64, i64)>,
    on_blur_save: EventHandler<()>,
) -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let qtype = qtype_str(&question);
    let title = title_of(&question);
    let q_num = idx + 1;

    let is_single = qtype == "single";
    let is_multi = qtype == "multi";
    let is_subjective = qtype == "subjective";
    let is_linear = qtype == "linear";

    rsx! {
        div {
            class: "q-block",
            "data-qtype": qtype,
            "data-testid": "poll-question-{idx}",
            div { class: "q-block__head",
                span { class: "q-block__num", "Q{q_num}" }
                div {
                    class: "segmented segmented--sm",
                    role: "tablist",
                    "data-testid": "poll-question-{idx}-type",
                    button {
                        class: "segmented__btn",
                        r#type: "button",
                        role: "tab",
                        "aria-selected": is_single,
                        onclick: move |_| on_type_change.call((idx, "single".into())),
                        "{tr.qtype_single}"
                    }
                    button {
                        class: "segmented__btn",
                        r#type: "button",
                        role: "tab",
                        "aria-selected": is_multi,
                        onclick: move |_| on_type_change.call((idx, "multi".into())),
                        "{tr.qtype_multi}"
                    }
                    button {
                        class: "segmented__btn",
                        r#type: "button",
                        role: "tab",
                        "aria-selected": is_subjective,
                        onclick: move |_| on_type_change.call((idx, "subjective".into())),
                        "{tr.qtype_subjective}"
                    }
                    button {
                        class: "segmented__btn",
                        r#type: "button",
                        role: "tab",
                        "aria-selected": is_linear,
                        onclick: move |_| on_type_change.call((idx, "linear".into())),
                        "{tr.qtype_linear}"
                    }
                }
                div { class: "q-block__head-spacer" }
                button {
                    class: "icon-btn",
                    r#type: "button",
                    aria_label: "{tr.remove_question}",
                    onclick: move |_| on_remove.call(idx),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "18",
                            y1: "6",
                            x2: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            y1: "6",
                            x2: "18",
                            y2: "18",
                        }
                    }
                }
            }

            input {
                class: "input",
                value: "{title}",
                oninput: move |e| on_title_change.call((idx, e.value())),
                onblur: move |_| on_blur_save.call(()),
            }

            if is_single {
                ChoiceOptions {
                    idx,
                    question: question.clone(),
                    is_check: false,
                    on_option_change,
                    on_blur_save,
                }
            }
            if is_multi {
                ChoiceOptions {
                    idx,
                    question: question.clone(),
                    is_check: true,
                    on_option_change,
                    on_blur_save,
                }
            }
            if is_subjective {
                SubjectiveBody {
                    idx,
                    question: question.clone(),
                    on_subjective_change,
                    on_blur_save,
                }
            }
            if is_linear {
                LinearBody {
                    idx,
                    question: question.clone(),
                    on_linear_change,
                    on_blur_save,
                }
            }
        }
    }
}

#[component]
fn ChoiceOptions(
    idx: usize,
    question: Question,
    is_check: bool,
    on_option_change: EventHandler<(usize, usize, String)>,
    on_blur_save: EventHandler<()>,
) -> Element {
    let options = options_of(&question);

    let body_class = if is_check {
        "q-body q-body--multi"
    } else {
        "q-body q-body--single"
    };
    let opt_class = if is_check { "q-opt q-opt--check" } else { "q-opt" };

    rsx! {
        div { class: "{body_class}",
            for (i , opt) in options.iter().enumerate() {
                div {
                    key: "opt-{i}",
                    class: "{opt_class}",
                    "aria-checked": "false",
                    "data-testid": "poll-question-{idx}-opt-{i}",
                    span { class: "q-opt__radio" }
                    input {
                        class: "input",
                        value: "{opt}",
                        oninput: move |e| on_option_change.call((idx, i, e.value())),
                        onblur: move |_| on_blur_save.call(()),
                    }
                }
            }
        }
    }
}

#[component]
fn SubjectiveBody(
    idx: usize,
    question: Question,
    on_subjective_change: EventHandler<(usize, String)>,
    on_blur_save: EventHandler<()>,
) -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let value = match &question {
        Question::Subjective(s) | Question::ShortAnswer(s) => s.description.clone(),
        _ => String::new(),
    };

    rsx! {
        div { class: "q-body q-body--subjective",
            span { class: "q-subjective-hint", "{tr.subjective_hint}" }
            textarea {
                class: "textarea",
                "data-testid": "poll-question-{idx}-hint",
                placeholder: "{tr.subjective_placeholder}",
                value: "{value}",
                oninput: move |e| on_subjective_change.call((idx, e.value())),
                onblur: move |_| on_blur_save.call(()),
            }
        }
    }
}

#[component]
fn LinearBody(
    idx: usize,
    question: Question,
    on_linear_change: EventHandler<(usize, i64, i64)>,
    on_blur_save: EventHandler<()>,
) -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let (min_v, max_v) = match &question {
        Question::LinearScale(l) => (l.min_value, l.max_value),
        _ => (1, 5),
    };

    rsx! {
        div { class: "q-body q-body--linear",
            div { class: "q-linear-row",
                div { class: "field",
                    label { class: "field__label", "{tr.linear_min_label}" }
                    input {
                        class: "input input--num",
                        r#type: "number",
                        "data-testid": "poll-question-{idx}-min",
                        value: "{min_v}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<i64>() {
                                on_linear_change.call((idx, v, max_v));
                            }
                        },
                        onblur: move |_| on_blur_save.call(()),
                    }
                }
                div { class: "q-linear-preview",
                    span { "{min_v}" }
                    span { class: "q-linear-preview__dot" }
                    span { class: "q-linear-preview__dot" }
                    span { class: "q-linear-preview__dot q-linear-preview__dot--filled" }
                    span { class: "q-linear-preview__dot" }
                    span { class: "q-linear-preview__dot" }
                    span { "{max_v}" }
                }
                div { class: "field",
                    label { class: "field__label", "{tr.linear_max_label}" }
                    input {
                        class: "input input--num",
                        r#type: "number",
                        "data-testid": "poll-question-{idx}-max",
                        value: "{max_v}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<i64>() {
                                on_linear_change.call((idx, min_v, v));
                            }
                        },
                        onblur: move |_| on_blur_save.call(()),
                    }
                }
            }
        }
    }
}
