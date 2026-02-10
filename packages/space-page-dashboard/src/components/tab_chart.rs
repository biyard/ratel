use crate::*;

/// TODO: 추후 디자인 구현 예정
#[component]
pub fn TabChart(data: TabChartData, title: String) -> Element {
    let mut selected_tab = use_signal(|| 0usize);

    rsx! {
        div {
            h4 { "{title}" }
            p { "Icon: {data.icon}" }
            p { "Main: {data.main_value} - {data.main_label}" }
            
            div {
                for (idx, tab) in data.tabs.iter().enumerate() {
                    button {
                        onclick: move |_| selected_tab.set(idx),
                        "{tab.label}"
                    }
                }
            }
            
            {data.tabs.get(selected_tab()).map(|tab| rsx! {
                ul {
                    for cat in tab.categories.iter() {
                        li {
                            "{cat.name}: {cat.value} ({cat.percentage})"
                        }
                    }
                }
            })}
        }
    }
}
