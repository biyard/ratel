use crate::features::spaces::pages::dashboard::i18n::DashboardTranslate;
use crate::features::spaces::pages::dashboard::*;

#[component]
pub fn RankingTable(data: RankingTableData) -> Element {
    let tr: DashboardTranslate = use_translate();
    let current_page = use_signal(|| 0usize);
    let page_size = data.page_size;
    let total_pages = (data.entries.len() + page_size - 1) / page_size;

    let start_idx = current_page() * page_size;
    let end_idx = (start_idx + page_size).min(data.entries.len());
    let page_entries = &data.entries[start_idx..end_idx];

    rsx! {
        div { class: "flex h-full w-full min-h-0 min-w-0 flex-col overflow-hidden rounded-2xl max-mobile:rounded-xl bg-web-card-bg",

            // Header
            div { class: "bg-web-graph-bg px-[30px] max-tablet:px-5 max-mobile:px-4",
                div { class: "flex min-w-[620px] max-tablet:min-w-[520px] max-mobile:min-w-0 self-stretch",
                    div { class: "flex h-14 w-[88px] max-tablet:w-[64px] max-mobile:w-[56px] items-center gap-1 rounded-tl-[10px] border-b border-separator bg-web-graph-bg p-4 max-tablet:p-3 max-mobile:p-2.5",
                        span { class: "text-left text-[13px] max-mobile:text-[12px] font-semibold tracking-[-0.14px] text-text-primary",
                            "{tr.rank}"
                        }
                        icons::ratel::Sorter { width: "20", height: "20" }
                    }
                    div { class: "flex h-14 min-w-0 flex-1 items-center gap-1 border-b border-separator bg-web-graph-bg p-4 max-tablet:p-3 max-mobile:p-2.5",
                        span { class: "truncate text-left text-[13px] max-mobile:text-[12px] font-semibold tracking-[-0.14px] text-text-primary",
                            "{tr.participant}"
                        }
                    }
                    div { class: "flex h-14 w-[170px] max-tablet:w-[120px] max-mobile:w-[96px] items-center gap-1 border-b border-separator bg-web-graph-bg p-4 max-tablet:p-3 max-mobile:p-2.5",
                        span { class: "text-left text-[13px] max-mobile:text-[12px] font-semibold tracking-[-0.14px] text-text-primary",
                            "{tr.point}"
                        }
                        icons::help_support::Info {
                            width: "20",
                            height: "20",
                            class: "h-5 w-5 [&>path]:stroke-text-secondary [&>path]:fill-none [&>circle]:stroke-text-secondary [&>circle]:fill-none cursor-pointer",
                        }
                        div { class: "ml-auto shrink-0",
                            icons::ratel::Sorter { width: "20", height: "20" }
                        }
                    }
                    div { class: "flex h-14 w-[170px] max-tablet:w-[120px] max-mobile:w-[96px] items-center gap-1 rounded-tr-[10px] border-b border-separator bg-web-graph-bg p-4 max-tablet:p-3 max-mobile:p-2.5",
                        span { class: "text-left text-[13px] max-mobile:text-[12px] font-semibold tracking-[-0.14px] text-text-primary",
                            "{tr.score}"
                        }
                        icons::help_support::Info {
                            width: "20",
                            height: "20",
                            class: "h-5 w-5 [&>path]:stroke-text-secondary [&>path]:fill-none [&>circle]:stroke-text-secondary [&>circle]:fill-none cursor-pointer",
                        }
                        div { class: "ml-auto shrink-0",
                            icons::ratel::Sorter { width: "20", height: "20" }
                        }
                    }
                }
            }

            // Body
            div { class: "flex-1 min-h-0 px-[30px] max-tablet:px-5 max-mobile:px-4",
                div { class: "h-full min-h-0 overflow-y-auto",
                    for entry in page_entries.iter() {
                        div { class: "flex items-stretch self-stretch min-w-[620px] max-tablet:min-w-[520px] max-mobile:min-w-0",

                            // Rank
                            div { class: "flex w-[88px] max-tablet:w-[64px] max-mobile:w-[56px] items-center border-b border-separator px-4 max-tablet:px-3 py-4 max-mobile:py-3",
                                span { class: "flex-1 basis-0 text-center text-text-primary text-sm leading-[22px] max-mobile:text-[13px] max-mobile:leading-5 font-normal font-raleway",
                                    "{entry.rank}"
                                }
                            }

                            // Participant
                            div { class: "min-w-0 flex-1 border-b border-separator px-4 max-tablet:px-3 py-4 max-mobile:py-3",
                                div { class: "flex items-center gap-2",
                                    if !entry.avatar.is_empty() {
                                        img {
                                            class: "h-6 w-6 rounded-full",
                                            src: "{entry.avatar}",
                                            alt: "{entry.name}",
                                        }
                                    } else {
                                        div { class: "flex h-6 w-6 items-center justify-center rounded-full bg-web-primary",
                                            span { class: "text-xs font-medium text-web-font-ab-bk",
                                                "{entry.name.chars().next().unwrap_or('U')}"
                                            }
                                        }
                                    }
                                    span { class: "truncate text-text-primary text-[13px] leading-5 max-mobile:text-xs font-medium font-raleway",
                                        "{entry.name}"
                                    }
                                }
                            }

                            // Point
                            div { class: "flex w-[170px] max-tablet:w-[120px] max-mobile:w-[96px] items-center border-b border-separator px-4 max-tablet:px-3 py-4 max-mobile:py-3",
                                span { class: "flex-1 basis-0 text-right text-text-primary text-sm leading-[22px] max-mobile:text-[13px] max-mobile:leading-5 font-normal font-roboto",
                                    "{entry.score:.0} P"
                                }
                            }

                            // Score
                            div { class: "flex w-[170px] max-tablet:w-[120px] max-mobile:w-[96px] items-center border-b border-separator px-4 max-tablet:px-3 py-4 max-mobile:py-3",
                                span { class: "flex-1 basis-0 text-right text-text-primary text-sm leading-[22px] max-mobile:text-[13px] max-mobile:leading-5 font-normal font-roboto",
                                    "{entry.change}"
                                }
                            }
                        }
                    }
                }
            }

            // Pagination
            div { class: "px-[30px] max-tablet:px-5 max-mobile:px-4 py-4 max-mobile:py-3",
                Pagination { current_page, total_pages }
            }
        }
    }
}
