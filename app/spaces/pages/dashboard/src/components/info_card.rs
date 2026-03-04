use crate::*;
use crate::i18n::DashboardTranslate;

#[component]
pub fn InfoCard(data: InfoCardData) -> Element {
    let tr: DashboardTranslate = use_translate();
    rsx! {
        div { class: "flex h-full w-full min-h-0 self-stretch flex-col items-start gap-2.5 max-mobile:gap-2 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-web-card-bg",

            // Header Section
            div { class: "flex w-full items-center justify-between gap-3 max-mobile:gap-2",

                // Left: Icon
                div { class: "flex h-11 w-11 items-center justify-center max-mobile:h-9 max-mobile:w-9 rounded-[10px] {data.icon.bg_class()}",
                    icons::ratel::Clock {
                        width: "24",
                        height: "24",
                        class: "h-6 w-6 max-mobile:h-4 max-mobile:w-4 [&>circle]:fill-none",
                    }
                }

                // Right: Main Stats
                div { class: "flex-1 min-w-0 text-right",
                    div { class: "text-[24px] leading-[24px] max-mobile:text-[20px] max-mobile:leading-[22px] font-bold text-text-primary font-inter",
                        "{data.main_value}"
                        if !data.unit.is_empty() {
                            span { class: "ml-1 text-[15px] max-mobile:text-[11px] text-text-primary",
                                "{data.unit}"
                            }
                        }
                    }
                    div { class: "mt-1 text-[15px] leading-[18px] max-mobile:text-[13px] max-mobile:leading-[16px] tracking-[-0.16px] font-semibold text-web-font-neutral font-raleway",
                        "{tr.points_available}"
                    }
                }
            }

            // Info Items
            div { class: "flex flex-1 min-h-0 self-stretch flex-col justify-end gap-2 max-mobile:gap-1 mt-4 max-mobile:mt-2 pr-1 overflow-y-auto",

                for item in data.items.iter() {
                    {
                        let raw_label = item.label.trim();
                        let boost_label = raw_label.strip_prefix("✕ ").map(str::trim);

                        rsx! {
                            div { class: "flex items-center justify-between gap-2 text-text-primary",
                                if let Some(text) = boost_label {
                                    div { class: "flex min-w-0 items-center gap-1",
                                        div { class: "h-[18px] w-[18px] shrink-0 max-mobile:h-4 max-mobile:w-4 [transform:rotate(-90deg)]",
                                            icons::validations::Clear {
                                                width: "18",
                                                height: "18",
                                                class: "h-full w-full text-icon-primary [&>path]:stroke-current",
                                            }
                                        }
                                        span { class: "truncate text-text-primary text-xs leading-4 font-medium font-inter",
                                            "{text}"
                                        }
                                    }
                                } else {
                                    span { class: "min-w-0 truncate text-xs max-mobile:text-[11px]", "{item.label}" }
                                }
                                span { class: "shrink-0 text-text-primary text-xs leading-4 font-semibold font-inter",
                                    "{item.value}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
