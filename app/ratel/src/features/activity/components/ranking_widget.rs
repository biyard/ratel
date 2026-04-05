use crate::features::activity::controllers::{get_my_score_handler, get_ranking_handler};
use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;

#[component]
pub fn RankingWidget(space_id: SpacePartition) -> Element {
    let tr: ActivityTranslate = use_translate();

    let ranking_loader = use_server_future(use_reactive(
        (&space_id,),
        |(sid,)| async move { get_ranking_handler(sid.clone(), None).await },
    ))?;

    let my_score_loader = use_server_future(use_reactive(
        (&space_id,),
        |(sid,)| async move { get_my_score_handler(sid.clone()).await },
    ))?;

    let ranking = ranking_loader.read();
    let my_score = my_score_loader.read();

    let top3 = ranking
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|r| r.entries.iter().take(3).cloned().collect::<Vec<_>>())
        .unwrap_or_default();

    let my = my_score
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    if top3.is_empty() {
        return rsx! {};
    }

    rsx! {
        Card {
            variant: CardVariant::Outlined,
            class: "mx-4 mb-2",
            Col {
                class: "w-full",

                Row {
                    class: "px-3 py-2 border-b border-separator",
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    span {
                        class: "text-xs font-semibold text-text-primary",
                        "{tr.ranking}"
                    }
                }

                for entry in top3.iter() {
                    Row {
                        class: "px-3 py-1.5",
                        main_axis_align: MainAxisAlign::Between,
                        cross_axis_align: CrossAxisAlign::Center,
                        Row {
                            class: "gap-2",
                            cross_axis_align: CrossAxisAlign::Center,
                            span {
                                class: "text-xs font-medium text-foreground-muted w-4 text-center",
                                "{entry.rank}"
                            }
                            if !entry.avatar.is_empty() {
                                img {
                                    class: "w-5 h-5 rounded-full object-cover",
                                    src: "{entry.avatar}",
                                    alt: "{entry.name}",
                                }
                            } else {
                                div {
                                    class: "flex items-center justify-center w-5 h-5 rounded-full bg-primary",
                                    span {
                                        class: "text-[10px] font-medium text-btn-primary-text",
                                        "{entry.name.chars().next().unwrap_or('?')}"
                                    }
                                }
                            }
                            span {
                                class: "text-xs text-text-primary truncate max-w-[80px]",
                                "{entry.name}"
                            }
                        }
                        span {
                            class: "text-xs font-medium text-text-primary",
                            "{entry.total_score}"
                        }
                    }
                }

                if let Some(ref score) = my {
                    if score.rank > 0 {
                        div { class: "border-t border-separator" }
                        Row {
                            class: "px-3 py-1.5 bg-primary/10 rounded-b-lg",
                            main_axis_align: MainAxisAlign::Between,
                            cross_axis_align: CrossAxisAlign::Center,
                            Row {
                                class: "gap-2",
                                cross_axis_align: CrossAxisAlign::Center,
                                span {
                                    class: "text-xs font-medium text-foreground-muted w-4 text-center",
                                    "{score.rank}"
                                }
                                span {
                                    class: "text-xs font-semibold text-primary",
                                    "{tr.you}"
                                }
                            }
                            span {
                                class: "text-xs font-medium text-primary",
                                "{score.total_score}"
                            }
                        }
                    }
                }
            }
        }
    }
}
