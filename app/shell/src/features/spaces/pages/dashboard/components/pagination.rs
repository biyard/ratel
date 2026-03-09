use crate::features::spaces::pages::dashboard::*;

#[component]
pub fn Pagination(current_page: Signal<usize>, total_pages: usize) -> Element {
    if total_pages <= 1 {
        return rsx! {};
    }

    rsx! {
        div { class: "flex items-center justify-end gap-2 pt-4 max-mobile:pt-3",

            // Previous Button
            button {
                class: if current_page() == 0 { "flex h-8 w-8 items-center justify-center rounded-lg text-sm text-text-primary opacity-30" } else { "flex h-8 w-8 items-center justify-center rounded-lg text-sm text-text-primary transition-colors hover:bg-white/10" },
                disabled: current_page() == 0,
                onclick: move |_| {
                    if current_page() > 0 {
                        current_page.set(current_page() - 1);
                    }
                },
                icons::arrows::ChevronLeft {
                    width: "16",
                    height: "16",
                    class: "h-4 w-4 [&>path]:stroke-current",
                }
            }

            // Page Numbers
            for page_num in 0..total_pages {
                button {
                    class: if current_page() == page_num { "flex h-8 w-8 flex-col items-center justify-center gap-2.5 rounded-lg border border-web-primary px-4 py-2 text-[14px] font-bold text-web-primary transition-colors" } else { "flex h-8 w-8 flex-col items-center justify-center gap-2.5 rounded-lg border border-web-primary px-4 py-2 text-[14px] font-bold text-text-primary transition-colors hover:bg-white/5" },
                    onclick: move |_| current_page.set(page_num),
                    "{page_num + 1}"
                }
            }

            // Next Button
            button {
                class: if current_page() >= total_pages - 1 { "flex h-8 w-8 items-center justify-center rounded-lg text-sm text-text-primary opacity-30" } else { "flex h-8 w-8 items-center justify-center rounded-lg text-sm text-text-primary transition-colors hover:bg-white/10" },
                disabled: current_page() >= total_pages - 1,
                onclick: move |_| {
                    if current_page() < total_pages - 1 {
                        current_page.set(current_page() + 1);
                    }
                },
                icons::arrows::ChevronLeft {
                    width: "16",
                    height: "16",
                    class: "h-4 w-4 rotate-180 [&>path]:stroke-current",
                }
            }
        }
    }
}
