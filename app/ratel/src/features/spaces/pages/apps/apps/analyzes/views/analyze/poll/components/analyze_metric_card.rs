use crate::features::spaces::pages::apps::apps::analyzes::*;

#[component]
pub fn AnalyzeMetricCard(label: String, value: String) -> Element {
    rsx! {
        Card {
            class: "min-w-[160px] gap-2 rounded-[12px] border border-separator bg-card px-4 py-4".to_string(),
            p { class: "text-[12px] font-medium text-text-secondary", "{label}" }
            p { class: "text-[22px]/[26px] font-bold font-raleway text-text-primary", "{value}" }
        }
    }
}
