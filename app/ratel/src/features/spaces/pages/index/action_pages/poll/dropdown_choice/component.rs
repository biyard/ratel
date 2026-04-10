use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::*;

#[component]
pub fn PollDropdown(
    idx: usize,
    question: DropdownQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::Dropdown { answer }) => *answer,
        _ => None,
    };

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/features/spaces/pages/index/action_pages/poll/subjective/style.css"),
        }
        select {
            class: "subjective-input",
            disabled,
            onchange: move |evt| {
                let idx: Option<i32> = evt.value().to_string().parse().ok();
                on_change.call(Answer::Dropdown { answer: idx });
            },
            option { value: "", selected: selected.is_none(), "Select..." }
            for (oi , opt) in question.options.iter().enumerate() {
                {
                    let v = format!("{oi}");
                    let is_sel = selected == Some(oi as i32);
                    rsx! {
                        option { value: "{v}", selected: is_sel, "{opt}" }
                    }
                }
            }
        }
    }
}
