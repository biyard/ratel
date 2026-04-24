use crate::features::spaces::pages::actions::actions::poll::controllers::{
    PollResultResponse, get_poll, get_poll_result,
};
use crate::features::spaces::pages::actions::actions::poll::{
    Answer, PollResponse, Question, SpacePollUserAnswer,
};
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::apps::panels::{
    CollectiveAttribute, PanelAttribute, SpacePanelQuotaResponse, VerifiableAttribute, list_panels,
};
use crate::*;

/// Controller hook for the Poll analyze detail page (arena).
///
/// Wraps the three loaders the page depends on (panels, poll, result),
/// the two filter signals, and the Excel export action. Loaders read the
/// `space_id` / `poll_id` signals reactively, so navigating between
/// polls re-runs them without resetting the hook.
#[derive(Clone, Copy)]
pub struct UseSpaceAnalyzePoll {
    pub space_id: ReadSignal<SpacePartition>,
    pub poll_id: ReadSignal<SpacePollEntityType>,

    pub panels: Loader<Vec<SpacePanelQuotaResponse>>,
    pub poll: Loader<PollResponse>,
    pub result: Loader<PollResultResponse>,

    pub selected_filter_group: Signal<String>,
    pub selected_filter_value: Signal<String>,

    pub handle_export_excel: Action<(), ()>,
}

#[track_caller]
pub fn use_space_analyze_poll(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> std::result::Result<UseSpaceAnalyzePoll, RenderError> {
    if let Some(ctx) = try_use_context::<UseSpaceAnalyzePoll>() {
        return Ok(ctx);
    }

    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut toast = use_toast();

    let panels = use_loader(move || async move { list_panels(space_id()).await })?;
    let poll = use_loader(move || async move { get_poll(space_id(), poll_id()).await })?;
    let result =
        use_loader(move || async move { get_poll_result(space_id(), poll_id()).await })?;

    let selected_filter_group = use_signal(|| "overall".to_string());
    let selected_filter_value = use_signal(String::new);

    // `tr` is Clone but not Copy, so snapshot the only field we need as a
    // String and clone the translate struct into the outer closure. The
    // inner async block then takes its own clone per invocation — matches
    // the pattern used by other arena hooks that surface toast messages.
    let download_started_text = tr.download_started.to_string();
    let tr_for_excel = tr.clone();
    let handle_export_excel = use_action(move || {
        let tr = tr_for_excel.clone();
        let download_started = download_started_text.clone();
        async move {
            let mut toast = toast;
            let poll_data = poll.read().clone();
            let panels_data = panels.read().clone();
            let result_data = result.read().clone();
            let excel_data = build_excel_data(&poll_data, &panels_data, &result_data, &tr);
            match download_analyze_excel(DownloadAnalyzeExcelRequest {
                file_name: build_excel_file_name(&space_id()),
                sheet_name: "Responses".to_string(),
                rows: excel_data.rows,
                merges: excel_data.merges,
            })
            .await
            {
                Ok(_) => {
                    toast.info(download_started);
                }
                Err(err) => {
                    toast.error(err);
                }
            }
            Ok::<(), crate::common::Error>(())
        }
    });

    Ok(use_context_provider(|| UseSpaceAnalyzePoll {
        space_id,
        poll_id,
        panels,
        poll,
        result,
        selected_filter_group,
        selected_filter_value,
        handle_export_excel,
    }))
}

// ─────────────────────────────────────────────────────────────────
// Excel export
// ─────────────────────────────────────────────────────────────────
//
// Everything below is pure business logic the export action depends
// on. It lives in the hook module (not in `page.rs`) because the
// component-tree rule is "views consume controllers" — the page
// shouldn't own server-shaped data transformations.
//
// `humanize_group_value` is also used by the page's filter-value
// label builder; it's `pub` so the page can reach it via the
// `analyzes::*` wildcard. Kept here because the translation table
// (backend enum → display label) is part of the controller layer,
// not the UI layer.

/// Excel export bundle — returned by `build_excel_data` and consumed
/// by `download_analyze_excel`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyzeExcelData {
    pub rows: Vec<Vec<String>>,
    pub merges: Vec<AnalyzeExcelMerge>,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct AnalyzeExportAttributes {
    include_gender: bool,
    include_age: bool,
    include_university: bool,
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

pub fn humanize_group_value(value: &str, tr: &SpaceAnalyzesAppTranslate) -> String {
    match value {
        "male" => tr.gender_male.to_string(),
        "female" => tr.gender_female.to_string(),
        "UNKNOWN" => tr.gender_unknown.to_string(),
        _ => value.to_string(),
    }
}

fn build_excel_file_name(space_id: &SpacePartition) -> String {
    format!("{}-analysis.xlsx", space_id)
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

    let category_label = if poll.space_action.prerequisite {
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
                category_label.clone(),
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
