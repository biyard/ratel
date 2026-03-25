use crate::features::spaces::pages::apps::apps::analyzes::*;

#[component]
pub fn SubjectiveQuestionSummary(
    title: String,
    total_responses: i64,
    response_unit: String,
    responses: Vec<(String, i64)>,
    no_responses_text: String,
) -> Element {
    rsx! {
        div { class: "flex w-full flex-col gap-5 rounded-xl border border-input-box-border bg-transparent p-5",
            div { class: "flex items-center justify-between gap-4 border-b border-input-box-border pb-2",
                div { class: "min-w-0 text-base font-semibold text-text-secondary", "{title}" }
                div { class: "shrink-0 text-sm font-medium text-text-secondary",
                    "{total_responses} {response_unit}"
                }
            }

            if responses.is_empty() {
                div { class: "rounded-md border border-input-box-border bg-input-box-bg px-4 py-2 text-sm text-text-primary",
                    "{no_responses_text}"
                }
            } else {
                div { class: "flex flex-col gap-2",
                    for (idx, (text, count)) in responses.into_iter().enumerate() {
                        div {
                            key: "text-response-{idx}",
                            class: "rounded-md border border-input-box-border bg-input-box-bg px-4 py-2 text-sm whitespace-pre-wrap text-text-primary",
                            "{text} ({count})"
                        }
                    }
                }
            }
        }
    }
}
