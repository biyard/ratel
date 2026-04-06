use super::components::*;
use crate::common::use_query;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::actions::poll::controllers::{
    get_poll, get_poll_result, PollResultResponse, PollResultSummary,
};
use crate::features::spaces::pages::actions::actions::poll::{
    Answer, PollResponse, Question, SpacePollUserAnswer,
};
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::apps::panels::{
    list_panels, CollectiveAttribute, PanelAttribute, SpacePanelQuotaResponse, VerifiableAttribute,
};
use crate::features::spaces::space_common::hooks::use_space_role;
use crate::features::spaces::space_common::types::space_page_actions_poll_key;

const ANALYZE_CHART_COLORS: [&str; 6] = [
    "#f97316", "#6366f1", "#22c55e", "#3b82f6", "#8b5cf6", "#eab308",
];
const DAY_MILLIS: i64 = 24 * 60 * 60 * 1000;

#[derive(Debug, Clone, PartialEq)]
struct AnalyzeFilterOption {
    key: String,
    label: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct AnalyzeExportAttributes {
    include_gender: bool,
    include_age: bool,
    include_university: bool,
}

const PANELS_QUERY_KEY: &str = "Panels";

fn panels_key(space_id: &SpacePartition) -> Vec<String> {
    vec![
        "Space".to_string(),
        space_id.to_string(),
        PANELS_QUERY_KEY.to_string(),
    ]
}

#[component]
pub fn SpaceAnalyzeDetailPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let role = use_space_role()();
    let mut toast = use_toast();

    if role != SpaceUserRole::Creator {
        return rsx! {};
    }

    let poll_key = space_page_actions_poll_key(&space_id(), &poll_id());
    let panels_query_key = panels_key(&space_id());
    let mut result_key = poll_key.clone();
    result_key.push("results".into());

    let panels_query = use_query(&panels_query_key, move || list_panels(space_id()))?;
    let poll_query = use_query(&poll_key, move || get_poll(space_id(), poll_id()))?;
    let result_query = use_query(&result_key, move || get_poll_result(space_id(), poll_id()))?;

    let panels = panels_query.read().clone();
    let poll = poll_query.read().clone();
    let result = result_query.read().clone();

    let filter_groups = build_filter_group_options(&result, &tr);
    let mut selected_filter_group = use_signal(|| "overall".to_string());
    let mut selected_filter_value = use_signal(String::new);
    let active_group = active_filter_group(&filter_groups, &selected_filter_group());
    let filter_values = build_filter_value_options(&result, &active_group, &tr);
    let active_value = active_filter_value(&filter_values, &selected_filter_value());
    let active_filter_key = compose_filter_key(&active_group, &active_value);
    let active_summaries = select_summaries(&result, &active_filter_key);
    let response_count = active_summaries
        .first()
        .map(summary_total_count)
        .unwrap_or(poll.user_response_count);

    let poll_for_download = poll.clone();
    let panels_for_download = panels.clone();
    let result_for_download = result.clone();
    let download_started_text = tr.download_started.to_string();
    let tr_for_excel = tr.clone();
    let result_for_filter_group = result.clone();
    let tr_for_filter_group = tr.clone();

    rsx! {
        div { class: "flex w-full flex-col gap-5",
            div { class: "flex items-center justify-between gap-3 max-tablet:flex-col max-tablet:items-start",
                div { class: "flex min-w-0 flex-col gap-2",
                    div { class: "flex items-center gap-2 max-mobile:flex-wrap",
                        h3 { class: "font-bold font-raleway text-[28px]/[32px] tracking-[-0.28px] text-text-primary",
                            "{poll.title}"
                        }
                    }
                }
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "flex flex-row w-fit justify-center items-center min-w-[140px]".to_string(),
                    onclick: move |_| {
                        let poll = poll_for_download.clone();
                        let panels = panels_for_download.clone();
                        let result = result_for_download.clone();
                        let tr_for_excel = tr_for_excel.clone();
                        let excel_data = build_excel_data(&poll, &panels, &result, &tr_for_excel);
                        let mut toast = toast;
                        let download_started_text = download_started_text.clone();
                        spawn(async move {
                            match download_analyze_excel(DownloadAnalyzeExcelRequest {
                                    file_name: build_excel_file_name(&poll),
                                    sheet_name: "Responses".to_string(),
                                    rows: excel_data.rows,
                                    merges: excel_data.merges,
                                })
                                .await
                            {
                                Ok(_) => {
                                    toast.info(download_started_text);
                                }
                                Err(err) => {
                                    toast.error(err);
                                }
                            }
                        });
                    },
                    {tr.download_excel}
                }
            }

            div { class: "grid gap-3 md:grid-cols-3",
                AnalyzeMetricCard {
                    label: tr.responses_count.to_string(),
                    value: response_count.to_string(),
                }
                AnalyzeMetricCard {
                    label: tr.remaining_days.to_string(),
                    value: remaining_days_label(poll.ended_at),
                }
                AnalyzeMetricCard {
                    label: tr.survey_period.to_string(),
                    value: format_period(poll.started_at, poll.ended_at),
                }
            }

            div { class: "flex items-center justify-between gap-3 max-tablet:flex-col max-tablet:items-stretch",
                div { class: "flex items-center gap-3 max-tablet:flex-col max-tablet:items-stretch",
                    crate::common::components::Select::<String> {
                        value: Some(active_group.clone()),
                        on_value_change: move |value: Option<String>| {
                            let Some(value) = value else {
                                return;
                            };
                            let next_values = build_filter_value_options(
                                &result_for_filter_group,
                                &value,
                                &tr_for_filter_group,
                            );
                            let next_value = next_values
                                .first()
                                .map(|option| option.key.clone())
                                .unwrap_or_default();
                            selected_filter_group.set(value);
                            selected_filter_value.set(next_value);
                        },
                        SelectTrigger {
                            min_width: "13.75rem",
                            aria_label: tr.filter_group_label,
                            SelectValue {}
                        }
                        SelectList { aria_label: tr.filter_group_label,
                            SelectGroup {
                                for (idx , option) in filter_groups.iter().enumerate() {
                                    SelectOption::<String> {
                                        key: "{option.key}",
                                        index: idx,
                                        value: option.key.clone(),
                                        text_value: "{option.label}",
                                        "{option.label}"
                                        SelectItemIndicator {}
                                    }
                                }
                            }
                        }
                    }

                    if active_group != "overall" && !filter_values.is_empty() {
                        crate::common::components::Select::<String> {
                            value: Some(active_value.clone()),
                            on_value_change: move |value: Option<String>| {
                                let Some(value) = value else {
                                    return;
                                };
                                selected_filter_value.set(value);
                            },
                            SelectTrigger {
                                min_width: "13.75rem",
                                aria_label: tr.filter_value_label,
                                SelectValue {}
                            }
                            SelectList { aria_label: tr.filter_value_label,
                                SelectGroup {
                                    for (idx , option) in filter_values.iter().enumerate() {
                                        SelectOption::<String> {
                                            key: "{option.key}",
                                            index: idx,
                                            value: option.key.clone(),
                                            text_value: "{option.label}",
                                            "{option.label}"
                                            SelectItemIndicator {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex w-full flex-col gap-4",
                for (idx , question) in poll.questions.iter().enumerate() {
                    if let Some(summary) = active_summaries.get(idx) {
                        {render_question_card(idx, question, summary, &active_filter_key, &tr)}
                    }
                }
            }
        }
    }
}

fn render_question_card(
    idx: usize,
    question: &Question,
    summary: &PollResultSummary,
    filter_key: &str,
    tr: &SpaceAnalyzesAppTranslate,
) -> Element {
    let filter_suffix = filter_dom_suffix(filter_key);
    match summary {
        PollResultSummary::ShortAnswer {
            total_count,
            answers,
        }
        | PollResultSummary::Subjective {
            total_count,
            answers,
        } => rsx! {
            SubjectiveQuestionSummary {
                key: "analyze-question-{filter_suffix}-{idx}",
                title: question.title().to_string(),
                total_responses: *total_count,
                response_unit: tr.total_response_count_unit.to_string(),
                responses: sorted_text_answers(answers),
                no_responses_text: tr.no_text_responses.to_string(),
            }
        },
        _ => {
            let (bars, other_answers) = build_choice_stats(question, summary);
            rsx! {
                ObjectiveQuestionSummary {
                    key: "analyze-question-{filter_suffix}-{idx}",
                    title: question.title().to_string(),
                    total_responses: summary_total_count(summary),
                    response_unit: tr.total_response_count_unit.to_string(),
                    chart_base_id: format!("analyze-question-chart-{filter_suffix}-{idx}"),
                    answers: bars,
                    other_answers,
                }
            }
        }
    }
}

fn filter_dom_suffix(filter_key: &str) -> String {
    filter_key
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

fn build_filter_group_options(
    result: &PollResultResponse,
    tr: &SpaceAnalyzesAppTranslate,
) -> Vec<AnalyzeFilterOption> {
    let mut options = vec![AnalyzeFilterOption {
        key: "overall".to_string(),
        label: tr.filter_all.to_string(),
    }];

    if !result.summaries_by_gender.is_empty() {
        options.push(AnalyzeFilterOption {
            key: "gender".to_string(),
            label: tr.filter_gender.to_string(),
        });
    }

    if !result.summaries_by_age.is_empty() {
        options.push(AnalyzeFilterOption {
            key: "age".to_string(),
            label: tr.filter_age.to_string(),
        });
    }

    if !result.summaries_by_school.is_empty() {
        options.push(AnalyzeFilterOption {
            key: "school".to_string(),
            label: tr.filter_school.to_string(),
        });
    }

    options
}

fn build_filter_value_options(
    result: &PollResultResponse,
    group: &str,
    tr: &SpaceAnalyzesAppTranslate,
) -> Vec<AnalyzeFilterOption> {
    let mut values = match group {
        "gender" => result
            .summaries_by_gender
            .keys()
            .cloned()
            .map(|key| AnalyzeFilterOption {
                label: humanize_group_value(&key, tr),
                key,
            })
            .collect::<Vec<_>>(),
        "age" => result
            .summaries_by_age
            .keys()
            .cloned()
            .map(|key| AnalyzeFilterOption {
                label: key.clone(),
                key,
            })
            .collect::<Vec<_>>(),
        "school" => result
            .summaries_by_school
            .keys()
            .cloned()
            .map(|key| AnalyzeFilterOption {
                label: humanize_group_value(&key, tr),
                key,
            })
            .collect::<Vec<_>>(),
        _ => vec![],
    };

    values.sort_by(|left, right| left.label.cmp(&right.label));
    values
}

fn active_filter_group(options: &[AnalyzeFilterOption], key: &str) -> String {
    options
        .iter()
        .find(|option| option.key == key)
        .map(|option| option.key.clone())
        .unwrap_or_else(|| "overall".to_string())
}

fn active_filter_value(options: &[AnalyzeFilterOption], key: &str) -> String {
    options
        .iter()
        .find(|option| option.key == key)
        .map(|option| option.key.clone())
        .or_else(|| options.first().map(|option| option.key.clone()))
        .unwrap_or_default()
}

fn compose_filter_key(group: &str, value: &str) -> String {
    if group == "overall" || value.is_empty() {
        "overall".to_string()
    } else {
        format!("{group}:{value}")
    }
}

fn select_summaries(result: &PollResultResponse, key: &str) -> Vec<PollResultSummary> {
    if let Some(gender) = key.strip_prefix("gender:") {
        return result
            .summaries_by_gender
            .get(gender)
            .cloned()
            .unwrap_or_else(|| result.summaries.clone());
    }

    if let Some(age) = key.strip_prefix("age:") {
        return result
            .summaries_by_age
            .get(age)
            .cloned()
            .unwrap_or_else(|| result.summaries.clone());
    }

    if let Some(school) = key.strip_prefix("school:") {
        return result
            .summaries_by_school
            .get(school)
            .cloned()
            .unwrap_or_else(|| result.summaries.clone());
    }

    result.summaries.clone()
}

fn summary_total_count(summary: &PollResultSummary) -> i64 {
    match summary {
        PollResultSummary::SingleChoice { total_count, .. }
        | PollResultSummary::MultipleChoice { total_count, .. }
        | PollResultSummary::ShortAnswer { total_count, .. }
        | PollResultSummary::Subjective { total_count, .. }
        | PollResultSummary::Checkbox { total_count, .. }
        | PollResultSummary::Dropdown { total_count, .. }
        | PollResultSummary::LinearScale { total_count, .. } => *total_count,
    }
}

fn build_choice_stats(
    question: &Question,
    summary: &PollResultSummary,
) -> (Vec<AnalyzeChoiceStat>, Vec<(String, i64)>) {
    match (question, summary) {
        (
            Question::SingleChoice(question) | Question::MultipleChoice(question),
            PollResultSummary::SingleChoice {
                total_count,
                answers,
                other_answers,
            }
            | PollResultSummary::MultipleChoice {
                total_count,
                answers,
                other_answers,
            },
        ) => (
            question
                .options
                .iter()
                .enumerate()
                .map(|(idx, _option)| {
                    build_choice_stat(
                        &(idx + 1).to_string(),
                        *answers.get(&idx.to_string()).unwrap_or(&0),
                        *total_count,
                        idx,
                    )
                })
                .chain(build_other_choice_stat(
                    question.allow_other.unwrap_or(false),
                    other_answers,
                    *total_count,
                    question.options.len(),
                ))
                .collect(),
            sorted_text_answers(other_answers),
        ),
        (
            Question::Checkbox(question),
            PollResultSummary::Checkbox {
                total_count,
                answers,
            },
        ) => (
            question
                .options
                .iter()
                .enumerate()
                .map(|(idx, _option)| {
                    build_choice_stat(
                        &(idx + 1).to_string(),
                        *answers.get(&idx.to_string()).unwrap_or(&0),
                        *total_count,
                        idx,
                    )
                })
                .collect(),
            vec![],
        ),
        (
            Question::Dropdown(question),
            PollResultSummary::Dropdown {
                total_count,
                answers,
            },
        ) => (
            question
                .options
                .iter()
                .enumerate()
                .map(|(idx, _option)| {
                    build_choice_stat(
                        &(idx + 1).to_string(),
                        *answers.get(&idx.to_string()).unwrap_or(&0),
                        *total_count,
                        idx,
                    )
                })
                .collect(),
            vec![],
        ),
        (
            Question::LinearScale(question),
            PollResultSummary::LinearScale {
                total_count,
                answers,
            },
        ) => (
            (question.min_value..=question.max_value)
                .enumerate()
                .map(|(idx, value)| {
                    build_choice_stat(
                        &value.to_string(),
                        *answers.get(&value.to_string()).unwrap_or(&0),
                        *total_count,
                        idx,
                    )
                })
                .collect(),
            vec![],
        ),
        _ => (vec![], vec![]),
    }
}

fn build_choice_stat(label: &str, count: i64, total_count: i64, index: usize) -> AnalyzeChoiceStat {
    let percentage = if total_count > 0 {
        (count as f64 / total_count as f64) * 100.0
    } else {
        0.0
    };

    AnalyzeChoiceStat {
        label: label.to_string(),
        count,
        percentage,
        color: ANALYZE_CHART_COLORS[index % ANALYZE_CHART_COLORS.len()],
    }
}

fn build_other_choice_stat(
    allow_other: bool,
    other_answers: &std::collections::HashMap<String, i64>,
    total_count: i64,
    index: usize,
) -> std::option::IntoIter<AnalyzeChoiceStat> {
    if !allow_other {
        return None.into_iter();
    }

    let count: i64 = other_answers.values().copied().sum();
    Some(build_choice_stat(
        &(index + 1).to_string(),
        count,
        total_count,
        index,
    ))
    .into_iter()
}

fn sorted_text_answers(answers: &std::collections::HashMap<String, i64>) -> Vec<(String, i64)> {
    let mut items: Vec<_> = answers
        .iter()
        .filter_map(|(text, count)| {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some((trimmed.to_string(), *count))
            }
        })
        .collect();

    items.sort_by(|left, right| {
        right
            .1
            .cmp(&left.1)
            .then_with(|| left.0.to_lowercase().cmp(&right.0.to_lowercase()))
    });
    items
}

fn remaining_days_label(ended_at: i64) -> String {
    let now = get_now_timestamp_millis();
    if ended_at <= now {
        return "0 Day".to_string();
    }

    let remaining = ((ended_at - now) + (DAY_MILLIS - 1)) / DAY_MILLIS;
    format!("{remaining} Day")
}

fn format_period(started_at: i64, ended_at: i64) -> String {
    let Some(start) = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(started_at) else {
        return "-".to_string();
    };
    let Some(end) = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ended_at) else {
        return "-".to_string();
    };

    format!("{} - {}", start.format("%Y.%m.%d"), end.format("%Y.%m.%d"))
}

fn humanize_group_value(value: &str, tr: &SpaceAnalyzesAppTranslate) -> String {
    match value {
        "male" => tr.gender_male.to_string(),
        "female" => tr.gender_female.to_string(),
        "UNKNOWN" => tr.gender_unknown.to_string(),
        _ => value.to_string(),
    }
}

fn build_excel_file_name(poll: &PollResponse) -> String {
    let slug = poll
        .title
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_lowercase();
    format!(
        "{}-analysis.xlsx",
        if slug.is_empty() { "poll" } else { &slug }
    )
}

#[derive(Debug, Clone, Default, PartialEq)]
struct AnalyzeExcelData {
    rows: Vec<Vec<String>>,
    merges: Vec<AnalyzeExcelMerge>,
}

fn build_excel_data(
    poll: &PollResponse,
    panels: &[SpacePanelQuotaResponse],
    result: &PollResultResponse,
    tr: &SpaceAnalyzesAppTranslate,
) -> AnalyzeExcelData {
    let export_attributes = build_export_attributes(panels);
    let user_rows = build_response_rows(result, tr);
    let question_count = poll.questions.len();

    let mut col = 0usize;
    let col_id = col;
    col += 1;
    let col_attr_start = col;
    let col_gender = if export_attributes.include_gender {
        let value = col;
        col += 1;
        Some(value)
    } else {
        None
    };
    let col_age = if export_attributes.include_age {
        let value = col;
        col += 1;
        Some(value)
    } else {
        None
    };
    let col_university = if export_attributes.include_university {
        let value = col;
        col += 1;
        Some(value)
    } else {
        None
    };
    let attr_count = usize::from(export_attributes.include_gender)
        + usize::from(export_attributes.include_age)
        + usize::from(export_attributes.include_university);
    let col_category = col;
    col += 1;
    let col_type = col;
    col += 1;
    let col_question_start = col;
    let total_columns = col_question_start + question_count;

    let mut header_top = vec![String::new(); total_columns];
    header_top[col_id] = tr.id.to_string();
    if attr_count > 0 {
        header_top[col_attr_start] = tr.attribute.to_string();
    }
    header_top[col_category] = tr.category.to_string();
    header_top[col_type] = tr.type_.to_string();
    if question_count > 0 {
        header_top[col_question_start] = tr.questionnaire.to_string();
    }

    let mut header_bottom = vec![String::new(); total_columns];
    if let Some(index) = col_gender {
        header_bottom[index] = tr.filter_gender.to_string();
    }
    if let Some(index) = col_age {
        header_bottom[index] = tr.filter_age.to_string();
    }
    if let Some(index) = col_university {
        header_bottom[index] = tr.university.to_string();
    }

    let mut rows = vec![header_top, header_bottom];
    let mut merges = vec![
        AnalyzeExcelMerge {
            start_row: 0,
            start_col: col_id,
            end_row: 1,
            end_col: col_id,
        },
        AnalyzeExcelMerge {
            start_row: 0,
            start_col: col_category,
            end_row: 1,
            end_col: col_category,
        },
        AnalyzeExcelMerge {
            start_row: 0,
            start_col: col_type,
            end_row: 1,
            end_col: col_type,
        },
    ];

    if attr_count > 0 {
        merges.push(AnalyzeExcelMerge {
            start_row: 0,
            start_col: col_attr_start,
            end_row: 0,
            end_col: col_attr_start + attr_count - 1,
        });
    }

    if question_count > 0 {
        merges.push(AnalyzeExcelMerge {
            start_row: 0,
            start_col: col_question_start,
            end_row: 1,
            end_col: col_question_start + question_count - 1,
        });
    }

    let sample_category_label = if poll.space_action.prerequisite {
        tr.sample_survey.to_string()
    } else {
        tr.final_survey.to_string()
    };

    for response_row in user_rows {
        let start_row = rows.len();

        if let Some(sample) = response_row.sample.as_ref() {
            push_excel_block(
                &mut rows,
                &mut merges,
                poll,
                sample,
                &response_row,
                sample_category_label.clone(),
                tr.question.to_string(),
                tr.answer.to_string(),
                col_category,
                col_type,
                col_question_start,
                total_columns,
            );
        }

        if let Some(final_answer) = response_row.final_answer.as_ref() {
            push_excel_block(
                &mut rows,
                &mut merges,
                poll,
                final_answer,
                &response_row,
                tr.final_survey.to_string(),
                tr.question.to_string(),
                tr.answer.to_string(),
                col_category,
                col_type,
                col_question_start,
                total_columns,
            );
        }

        if rows.len() == start_row {
            continue;
        }

        let end_row = rows.len() - 1;
        merges.push(AnalyzeExcelMerge {
            start_row,
            start_col: col_id,
            end_row,
            end_col: col_id,
        });

        if let Some(index) = col_gender {
            merges.push(AnalyzeExcelMerge {
                start_row,
                start_col: index,
                end_row,
                end_col: index,
            });
            rows[start_row][index] = response_row.gender.clone();
        }

        if let Some(index) = col_age {
            merges.push(AnalyzeExcelMerge {
                start_row,
                start_col: index,
                end_row,
                end_col: index,
            });
            rows[start_row][index] = response_row.age.clone();
        }

        if let Some(index) = col_university {
            merges.push(AnalyzeExcelMerge {
                start_row,
                start_col: index,
                end_row,
                end_col: index,
            });
            rows[start_row][index] = response_row.university.clone();
        }

        rows[start_row][col_id] = response_row.display_name.clone();
    }
    AnalyzeExcelData { rows, merges }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct AnalyzeResponseRow {
    display_name: String,
    gender: String,
    age: String,
    university: String,
    sample: Option<SpacePollUserAnswer>,
    final_answer: Option<SpacePollUserAnswer>,
}

fn build_export_attributes(panels: &[SpacePanelQuotaResponse]) -> AnalyzeExportAttributes {
    let mut attributes = AnalyzeExportAttributes::default();

    for panel in panels {
        let panel_attributes = if panel.attributes_vec.is_empty()
            && !matches!(panel.attributes, PanelAttribute::None)
        {
            vec![panel.attributes]
        } else {
            panel.attributes_vec.clone()
        };

        for attribute in panel_attributes {
            match attribute {
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender)
                | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)) => {
                    attributes.include_gender = true;
                }
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)
                | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_)) => {
                    attributes.include_age = true;
                }
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
                    attributes.include_university = true;
                }
                _ => {}
            }
        }
    }

    attributes
}

fn build_response_rows(
    result: &PollResultResponse,
    tr: &SpaceAnalyzesAppTranslate,
) -> Vec<AnalyzeResponseRow> {
    use std::collections::HashMap;

    let mut final_by_user: HashMap<String, SpacePollUserAnswer> = HashMap::new();
    for answer in &result.final_answers {
        final_by_user.insert(user_key_from_pk(&answer.pk.to_string()), answer.clone());
    }

    let mut sample_by_user: HashMap<String, SpacePollUserAnswer> = HashMap::new();
    for answer in &result.sample_answers {
        sample_by_user.insert(user_key_from_pk(&answer.pk.to_string()), answer.clone());
    }

    let mut user_order = Vec::new();
    for user in final_by_user.keys() {
        user_order.push(user.clone());
    }
    for user in sample_by_user.keys() {
        if !final_by_user.contains_key(user) {
            user_order.push(user.clone());
        }
    }
    user_order.sort();

    user_order
        .into_iter()
        .filter_map(|user_key| {
            let final_answer = final_by_user.get(&user_key).cloned();
            let sample = sample_by_user.get(&user_key).cloned();
            let meta = final_answer.as_ref().or(sample.as_ref())?;

            Some(AnalyzeResponseRow {
                display_name: meta
                    .display_name
                    .clone()
                    .or_else(|| meta.username.clone())
                    .unwrap_or(user_key),
                gender: meta
                    .respondent
                    .as_ref()
                    .and_then(|respondent| respondent.gender.as_ref())
                    .map(|gender| humanize_group_value(&gender.to_string(), tr))
                    .unwrap_or_default(),
                age: meta
                    .respondent
                    .as_ref()
                    .and_then(|respondent| respondent.age.as_ref())
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                university: meta
                    .respondent
                    .as_ref()
                    .and_then(|respondent| respondent.school.clone())
                    .unwrap_or_default(),
                sample,
                final_answer,
            })
        })
        .collect()
}

fn push_excel_block(
    rows: &mut Vec<Vec<String>>,
    merges: &mut Vec<AnalyzeExcelMerge>,
    poll: &PollResponse,
    response: &SpacePollUserAnswer,
    row_meta: &AnalyzeResponseRow,
    category_label: String,
    question_label: String,
    answer_label: String,
    col_category: usize,
    col_type: usize,
    col_question_start: usize,
    total_columns: usize,
) {
    let start_row = rows.len();
    let mut question_row = vec![String::new(); total_columns];
    question_row[col_category] = category_label.clone();
    question_row[col_type] = question_label;

    for (index, question) in poll.questions.iter().enumerate() {
        question_row[col_question_start + index] = question.title().to_string();
    }

    let mut answer_row = vec![String::new(); total_columns];
    answer_row[col_category] = category_label;
    answer_row[col_type] = answer_label;

    for (index, question) in poll.questions.iter().enumerate() {
        answer_row[col_question_start + index] =
            to_answer_display(question, response.answers.get(index));
    }

    rows.push(question_row);
    rows.push(answer_row);

    merges.push(AnalyzeExcelMerge {
        start_row,
        start_col: col_category,
        end_row: start_row + 1,
        end_col: col_category,
    });

    let _ = row_meta;
}

fn user_key_from_pk(pk: &str) -> String {
    pk.split("#USER#").nth(1).unwrap_or(pk).to_string()
}

fn to_answer_display(question: &Question, answer: Option<&Answer>) -> String {
    let Some(answer) = answer else {
        return String::new();
    };

    match (question, answer) {
        (
            Question::SingleChoice(question) | Question::MultipleChoice(question),
            Answer::SingleChoice { answer, other },
        ) => combine_parts([
            answer.and_then(|value| label_of_option(&question.options, value)),
            non_empty_optional_text(other),
        ]),
        (
            Question::SingleChoice(question) | Question::MultipleChoice(question),
            Answer::MultipleChoice { answer, other },
        ) => combine_parts([
            answer.as_ref().map(|indices| {
                indices
                    .iter()
                    .filter_map(|value| label_of_option(&question.options, *value))
                    .collect::<Vec<_>>()
                    .join(", ")
            }),
            non_empty_optional_text(other),
        ]),
        (Question::ShortAnswer(_), Answer::ShortAnswer { answer })
        | (Question::Subjective(_), Answer::Subjective { answer }) => {
            sanitized_optional_text(answer)
        }
        (Question::Checkbox(question), Answer::Checkbox { answer }) => answer
            .as_ref()
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|value| label_of_option(&question.options, *value))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default(),
        (Question::Dropdown(question), Answer::Dropdown { answer }) => answer
            .and_then(|value| label_of_option(&question.options, value))
            .unwrap_or_default(),
        (Question::LinearScale(_), Answer::LinearScale { answer }) => {
            answer.map(|value| value.to_string()).unwrap_or_default()
        }
        _ => String::new(),
    }
}

fn label_of_option(options: &[String], index: i32) -> Option<String> {
    if index < 0 {
        return None;
    }

    options.get(index as usize).cloned()
}

fn sanitized_optional_text(value: &Option<String>) -> String {
    non_empty_optional_text(value).unwrap_or_default()
}

fn non_empty_optional_text(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn combine_parts(parts: [Option<String>; 2]) -> String {
    parts
        .into_iter()
        .flatten()
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}
