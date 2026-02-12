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
                class: if current_page() == 0 { "rounded-lg flex items-center justify-center w-8 h-8 text-sm border border-white/30 text-white/30" } else { "rounded-lg flex items-center justify-center w-8 h-8 text-sm border border-white text-white" },
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
                    class: if current_page() == page_num { "rounded-lg flex items-center justify-center w-8 h-8 text-[14px] font-bold transition-colors border border-space-dashboard-accent text-space-dashboard-accent" } else { "rounded-lg flex items-center justify-center w-8 h-8 text-[14px] font-bold transition-colors border border-white text-white" },
                    onclick: move |_| current_page.set(page_num),
                    "{page_num + 1}"
                }
            }

            // Next Button
            button {
                class: if current_page() >= total_pages - 1 { "rounded-lg flex items-center justify-center w-8 h-8 text-sm border border-white/30 text-white/30" } else { "rounded-lg flex items-center justify-center w-8 h-8 text-sm border border-white text-white" },
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

