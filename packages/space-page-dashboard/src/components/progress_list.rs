use crate::*;

/// TODO: 추후 디자인 구현 예정
#[component]
pub fn ProgressList(data: ProgressListData, title: String) -> Element {
    rsx! {
        div {
            h4 { "{title}" }
            p { "Icon: {data.icon}" }
            p { "Main: {data.main_value} - {data.main_label}" }
            ul {
                for item in data.items.iter() {
                    li {
                        "{item.label}: {item.current}/{item.total}"
                    }
                }
            }
        }
    }
}
