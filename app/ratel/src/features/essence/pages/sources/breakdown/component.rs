use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceBreakdown() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let hook = use_essence_sources()?;

    // Per-kind totals come from the authoritative `UserEssenceStats`
    // counters so the cards show true totals, not whatever the current
    // paginated page happens to hold.
    //
    // Pre-migration fallback: when every per-kind counter is zero but the
    // global `total_sources` is non-zero, the user predates the per-kind
    // fields and hasn't been re-counted via `POST /api/admin/essences/migrate`.
    // In that case, fall back to bucketing whatever rows the current page
    // happens to hold so the breakdown is at least directionally useful.
    // The numbers will become exact once the migrate endpoint runs.
    let stats_handle = hook.stats;
    let items_handle = hook.items;
    let breakdown = use_memo(move || {
        let s = stats_handle.read();
        let total = s.total_sources.max(0) as u32;
        let per_kind_zero = s.total_notion == 0
            && s.total_post == 0
            && s.total_comment == 0
            && s.total_poll == 0
            && s.total_quiz == 0;
        if per_kind_zero && total > 0 {
            let mut counts = BreakdownCounts {
                total,
                ..Default::default()
            };
            for row in items_handle.read().iter() {
                match row.source_kind {
                    EssenceSourceKind::Notion => counts.notion += 1,
                    EssenceSourceKind::Post => counts.post += 1,
                    EssenceSourceKind::PostComment | EssenceSourceKind::DiscussionComment => {
                        counts.comment += 1
                    }
                    EssenceSourceKind::Poll => counts.poll += 1,
                    EssenceSourceKind::Quiz => counts.quiz += 1,
                }
            }
            return counts;
        }
        BreakdownCounts {
            total,
            notion: s.total_notion.max(0) as u32,
            post: s.total_post.max(0) as u32,
            comment: s.total_comment.max(0) as u32,
            poll: s.total_poll.max(0) as u32,
            quiz: s.total_quiz.max(0) as u32,
        }
    });

    let selected = hook.selected_kind;
    let mut set_kind_action = hook.set_kind;
    let mut set_kind = move |next: KindFilter| {
        set_kind_action.call(next);
    };

    let counts = breakdown();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        section { class: "essence-breakdown",
            KindCard {
                kind: KindFilter::All,
                selected: selected() == KindFilter::All,
                value: counts.total,
                total: counts.total,
                label: tr.kind_all.to_string(),
                on_select: move |_| set_kind(KindFilter::All),
                icon_variant: "all",
                icon: rsx! {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "10" }
                        line {
                            x1: "2",
                            y1: "12",
                            x2: "22",
                            y2: "12",
                        }
                        path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                    }
                },
            }
            KindCard {
                kind: KindFilter::Notion,
                selected: selected() == KindFilter::Notion,
                value: counts.notion,
                total: counts.total,
                label: tr.kind_notion.to_string(),
                on_select: move |_| set_kind(KindFilter::Notion),
                icon_variant: "notion",
                icon: rsx! { "N" },
            }
            KindCard {
                kind: KindFilter::Post,
                selected: selected() == KindFilter::Post,
                value: counts.post,
                total: counts.total,
                label: tr.kind_post.to_string(),
                on_select: move |_| set_kind(KindFilter::Post),
                icon_variant: "posts",
                icon: rsx! {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
                        path { d: "M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4z" }
                    }
                },
            }
            KindCard {
                kind: KindFilter::Comment,
                selected: selected() == KindFilter::Comment,
                value: counts.comment,
                total: counts.total,
                label: tr.kind_comment.to_string(),
                on_select: move |_| set_kind(KindFilter::Comment),
                icon_variant: "comments",
                icon: rsx! {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                    }
                },
            }
            KindCard {
                kind: KindFilter::Poll,
                selected: selected() == KindFilter::Poll,
                value: counts.poll,
                total: counts.total,
                label: tr.kind_poll.to_string(),
                on_select: move |_| set_kind(KindFilter::Poll),
                icon_variant: "actions",
                icon: rsx! {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M9 11l3 3L22 4" }
                        path { d: "M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" }
                    }
                },
            }
            KindCard {
                kind: KindFilter::Quiz,
                selected: selected() == KindFilter::Quiz,
                value: counts.quiz,
                total: counts.total,
                label: tr.kind_quiz.to_string(),
                on_select: move |_| set_kind(KindFilter::Quiz),
                icon_variant: "actions",
                icon: rsx! {
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                        line {
                            x1: "12",
                            y1: "17",
                            x2: "12.01",
                            y2: "17",
                        }
                    }
                },
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
struct BreakdownCounts {
    total: u32,
    notion: u32,
    post: u32,
    comment: u32,
    poll: u32,
    quiz: u32,
}

#[component]
fn KindCard(
    kind: KindFilter,
    selected: bool,
    value: u32,
    total: u32,
    label: String,
    on_select: EventHandler<KindFilter>,
    icon_variant: &'static str,
    icon: Element,
) -> Element {
    let pct = if total == 0 {
        0
    } else {
        ((value as f64 / total as f64) * 100.0).round() as u32
    };

    rsx! {
        article {
            class: "essence-kind-card",
            "data-selected": selected,
            onclick: move |_| on_select.call(kind),
            div { class: "essence-kind-card__head",
                span { class: "essence-kind-card__icon essence-kind-card__icon--{icon_variant}",
                    {icon}
                }
                span { class: "essence-kind-card__pct", "{pct}%" }
            }
            span { class: "essence-kind-card__value", "{value}" }
            span { class: "essence-kind-card__label", "{label}" }
        }
    }
}
