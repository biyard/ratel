use crate::*;

#[component]
pub fn SettingsTabs(
    active_index: usize,
    on_select: EventHandler<usize>,
    tab_one_label: String,
    tab_two_label: String,
) -> Element {
    let labels = [tab_one_label, tab_two_label];
    rsx! {
        div {
            role: "tablist",
            aria_label: "Profile tabs",
            class: "flex text-sm font-bold text-text-primary",
            for (idx , label) in labels.into_iter().enumerate() {
                button {
                    key: "{idx}",
                    role: "tab",
                    id: "tab-{idx}",
                    aria_controls: "panel-{idx}",
                    aria_selected: "{active_index == idx}",
                    class: {
                        let base = "group flex-1 flex flex-col items-center justify-center py-3 transition-colors text-tab-label";
                        let suffix = if active_index == idx {
                            "text-tab-label/80"
                        } else {
                            "hover:text-tab-label/80"
                        };
                        format!("{base}{suffix}")
                    },
                    onclick: move |_| on_select.call(idx),
                    span { "{label}" }
                    div { class: "mt-2 h-0.5 w-[29px] rounded bg-primary opacity-0 transition-opacity duration-200 group-aria-selected:opacity-100" }
                }
            }
        }
    }
}
