use crate::*;

/// TODO: 추후 디자인 구현 예정
#[component]
pub fn StatCard(data: StatCardData, title: String) -> Element {
    rsx! {
        div {
            h4 { "{title}" }
            p { "Icon: {data.icon}" }
            p { "Label: {data.label}" }
            p { "Value: {data.value}" }
            p { "Trend: {data.trend}%" }
            p { "{data.trend_label}" }
        }
    }
}
