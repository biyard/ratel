use crate::features::activity::controllers::{get_my_score_handler, get_ranking_handler};
use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;

#[component]
pub fn RankingWidget(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: ActivityTranslate = use_translate();

    let ranking_loader =
        use_loader(move || async move { get_ranking_handler(space_id(), None).await })?;

    let my_score_loader =
        use_loader(move || async move { get_my_score_handler(space_id()).await })?;

    let ranking = ranking_loader();
    let my_score = my_score_loader();

    let top3: Vec<_> = ranking.items.iter().take(3).cloned().collect();

    if top3.is_empty() {
        return rsx! {};
    }

    rsx! {
        Card { variant: CardVariant::Outlined, class: "mx-4 mb-2",
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
                                class: "py-1.5 px-3 aria-highlighted:bg-primary/10",
                                "aria-highlighted": is_me,
                                main_axis_align: MainAxisAlign::Between,
                                cross_axis_align: CrossAxisAlign::Center,
                                Row { class: "gap-2", cross_axis_align: CrossAxisAlign::Center,
                                    span { class: "w-4 text-xs font-medium text-center text-foreground-muted", "{entry.rank}" }
                                    if !entry.avatar.is_empty() {
                                        img {
                                            class: "object-cover w-5 h-5 rounded-full",
                                            src: "{entry.avatar}",
                                            alt: "{entry.name}",
                                        }
                                    } else {
                                        div { class: "flex justify-center items-center w-5 h-5 rounded-full bg-primary",
                                            span { class: "font-medium text-[10px] text-btn-primary-text",
                                                "{entry.name.chars().next().unwrap_or('?')}"
                                            }
                                        }
                                    }
                                    span { class: "text-xs text-text-primary truncate max-w-[80px]",
                                        if is_me {
                                            "{entry.name} ({tr.you})"
                                        } else {
                                            "{entry.name}"
                                        }
                                    }
                                }
                                span {
                                    class: "text-xs font-medium text-text-primary aria-highlighted:text-primary",
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
                        class: "py-1.5 px-3 rounded-b-lg bg-primary/10",
                        main_axis_align: MainAxisAlign::Between,
                        cross_axis_align: CrossAxisAlign::Center,
                        Row {
                            class: "gap-2",
                            cross_axis_align: CrossAxisAlign::Center,
                            span { class: "w-4 text-xs font-medium text-center text-foreground-muted",
                                "{my_score.rank}"
                            }
                            span { class: "text-xs font-semibold text-primary", "{tr.you}" }
                        }
                        span { class: "text-xs font-medium text-primary", "{my_score.total_score}" }
                    }
                }
            }
        }
    }
}
