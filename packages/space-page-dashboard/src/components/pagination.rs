use crate::*;

#[component]
pub fn Pagination(current_page: Signal<usize>, total_pages: usize) -> Element {
    if total_pages <= 1 {
        return rsx! {};
    }

    rsx! {
        div { class: "flex items-center justify-end gap-2 pt-4",

            // Previous Button
            button {
                class: if current_page() == 0 { "flex items-center justify-center w-8 h-8 text-sm text-white/30 border border-white/30 rounded-lg" } else { "flex items-center justify-center w-8 h-8 text-sm text-white border border-white rounded-lg" },
                disabled: current_page() == 0,
                onclick: move |_| {
                    if current_page() > 0 {
                        current_page.set(current_page() - 1);
                    }
                },
                "‹"
            }

            // Page Numbers
            for page_num in 0..total_pages {
                button {
                    class: if current_page() == page_num { "flex items-center justify-center w-8 h-8 text-[14px] font-bold text-space-dashboard-accent border border-space-dashboard-accent rounded-lg transition-colors" } else { "flex items-center justify-center w-8 h-8 text-[14px] font-bold text-white border border-white rounded-lg transition-colors" },
                    onclick: move |_| current_page.set(page_num),
                    "{page_num + 1}"
                }
            }

            // Next Button
            button {
                class: if current_page() >= total_pages - 1 { "flex items-center justify-center w-8 h-8 text-sm text-white/30 border border-white/30 rounded-lg" } else { "flex items-center justify-center w-8 h-8 text-sm text-white border border-white rounded-lg" },
                disabled: current_page() >= total_pages - 1,
                onclick: move |_| {
                    if current_page() < total_pages - 1 {
                        current_page.set(current_page() + 1);
                    }
                },
                "›"
            }
        }
    }
}

