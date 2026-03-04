use crate::*;
use crate::i18n::DashboardTranslate;

#[component]
pub fn StatCard(data: StatCardData) -> Element {
    let tr: DashboardTranslate = use_translate();
    rsx! {
        div { class: "flex h-full w-full min-h-0 flex-col gap-2.5 max-mobile:gap-2 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-web-card-bg",

            // Header Section
            div { class: "flex items-start justify-between gap-3 max-mobile:gap-2",

                // Left: Icon
                div { class: "flex h-11 w-11 shrink-0 flex-col items-center justify-center gap-2.5 px-6 py-3 rounded-[10px] {data.icon.bg_class()}",
                    icons::ratel::Chest {
                        width: "24",
                        height: "24",
                        class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
                    }
                }

                // Right: Main Stats
                div { class: "flex-1 min-w-0 text-right",
                    div { class: "flex items-baseline justify-end gap-1",
                        span { class: "self-stretch text-[24px] leading-[24px] max-mobile:text-[20px] max-mobile:leading-[22px] font-bold text-text-primary font-inter",
                            "{data.value}"
                        }
                        if !data.trend_label.is_empty() {
                            span { class: "self-stretch text-[15px] leading-[18px] max-mobile:text-[13px] max-mobile:leading-[16px] tracking-[-0.16px] font-semibold text-web-font-neutral font-raleway",
                                "{data.trend_label}"
                            }
                        }
                    }
                    div { class: "mt-0.5 text-[15px] max-mobile:text-[13px] font-semibold text-text-primary-muted [line-height:18px] max-mobile:[line-height:16px]",
                        "{tr.incentive_pool}"
                    }
                }
            }

            // Additional Info
            div { class: "flex-1 min-h-0 pr-1 space-y-0.5 max-mobile:space-y-1 overflow-y-auto",
                if !data.total_winners.is_empty() {
                    div { class: "flex items-center justify-between text-text-primary",
                        span { class: "text-xs leading-4 font-medium font-raleway",
                            "{tr.total_winners}"
                        }
                        span { class: "text-xs leading-4 font-medium font-raleway",
                            "{data.total_winners}"
                        }
                    }
                }

                if !data.rank_rate.is_empty() {
                    p { class: "text-web-font-neutral text-xs leading-4 font-medium font-inter",
                        "{tr.rank_rate}"
                    }
                }
            }

            // Bottom Section
            if !data.incentive_pool.is_empty() {
                div { class: "border-t border-separator pt-2",
                    div { class: "flex items-center justify-between gap-2",
                        span { class: "shrink-0 text-text-primary text-xs leading-4 font-medium font-raleway",
                            "{tr.incentive_pool}"
                        }
                        span { class: "min-w-0 max-w-[60%] truncate text-right text-text-primary text-xs leading-4 font-medium font-raleway",
                            "{data.incentive_pool}"
                        }
                    }
                }
            }
        }
    }
}
