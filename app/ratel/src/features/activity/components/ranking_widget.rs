use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;
use crate::features::spaces::space_common::hooks::{use_my_score, use_ranking};

#[component]
pub fn RankingWidget(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: ActivityTranslate = use_translate();

    let ranking_loader = use_ranking();
    let my_score_loader = use_my_score();

    let ranking = ranking_loader();
    let my_score = my_score_loader();

    let top3: Vec<_> = ranking.items.iter().take(3).cloned().collect();

    if top3.is_empty() {
        return rsx! {};
    }

    rsx! {
        Card { variant: CardVariant::Outlined, class: "mx-4 my-2 !py-2.5",
            Col { class: "w-full",

                Row {
                    class: "py-2 px-3 border-b border-separator",
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    span { class: "text-xs font-semibold text-text-primary", "{tr.ranking}" }
                }

                for entry in top3.iter() {
                    {
                        let is_me = my_score.rank > 0 && entry.rank == my_score.rank;
                        rsx! {
                            Row {
                                class: "py-1.5 px-3 w-full aria-highlighted:bg-primary/10",
                                "aria-highlighted": is_me,
                                main_axis_align: MainAxisAlign::Between,
                                cross_axis_align: CrossAxisAlign::Center,
                                Row {
                                    class: "gap-2 min-w-0 flex-1",
                                    cross_axis_align: CrossAxisAlign::Center,
                                    span { class: "w-4 text-xs font-medium text-center text-foreground-muted shrink-0",
                                        "{entry.rank}"
                                    }
                                    if !entry.avatar.is_empty() {
                                        img {
                                            class: "object-cover w-5 h-5 rounded-full shrink-0",
                                            src: "{entry.avatar}",
                                            alt: "{entry.name}",
                                        }
                                    } else {
                                        div { class: "flex justify-center items-center w-5 h-5 rounded-full bg-primary shrink-0",
                                            span { class: "font-medium text-[10px] text-btn-primary-text",
                                                "{entry.name.chars().next().unwrap_or('?')}"
                                            }
                                        }
                                    }
                                    span { class: "text-xs text-text-primary truncate min-w-0",
                                        if is_me {
                                            "{entry.name} ({tr.you})"
                                        } else {
                                            "{entry.name}"
                                        }
                                    }
                                }
                                span {
                                    class: "w-10 text-xs font-medium text-right tabular-nums text-text-primary shrink-0 aria-highlighted:text-primary",
                                    "aria-highlighted": is_me,
                                    "{entry.total_score}"
                                }
                            }
                        }
                    }
                }

                if my_score.rank > 3 {
                    div { class: "border-t border-separator" }
                    Row {
                        class: "py-1.5 px-3 w-full rounded-b-lg bg-primary/10",
                        main_axis_align: MainAxisAlign::Between,
                        cross_axis_align: CrossAxisAlign::Center,
                        Row {
                            class: "gap-2 min-w-0 flex-1",
                            cross_axis_align: CrossAxisAlign::Center,
                            span { class: "w-4 text-xs font-medium text-center text-foreground-muted",
                                "{my_score.rank}"
                            }
                            span { class: "text-xs font-semibold text-primary", "{tr.you}" }
                        }
                        span { class: "w-10 text-xs font-medium text-right tabular-nums text-primary shrink-0",
                            "{my_score.total_score}"
                        }
                    }
                }
            }
        }
    }
}
