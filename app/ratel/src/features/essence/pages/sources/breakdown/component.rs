use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceBreakdown() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let hook = use_essence_sources();

    let breakdown = use_memo(move || {
        let list = hook.sources.read();
        let total = list.len() as u32;
        let notion = list.iter().filter(|s| s.kind == EssenceSourceKind::Notion).count() as u32;
        let posts = list.iter().filter(|s| s.kind == EssenceSourceKind::RatelPost).count() as u32;
        let comments = list.iter().filter(|s| s.kind == EssenceSourceKind::Comment).count() as u32;
        let actions = list.iter().filter(|s| s.kind == EssenceSourceKind::Action).count() as u32;
        BreakdownCounts { total, notion, posts, comments, actions }
    });

    let selected = hook.selected_kind;
    let mut selected_writable = hook.selected_kind;
    let mut set_kind = move |next: KindFilter| {
        selected_writable.set(next);
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
                kind: KindFilter::RatelPost,
                selected: selected() == KindFilter::RatelPost,
                value: counts.posts,
                total: counts.total,
                label: tr.kind_ratel_posts.to_string(),
                on_select: move |_| set_kind(KindFilter::RatelPost),
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
                value: counts.comments,
                total: counts.total,
                label: tr.kind_comments.to_string(),
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
                kind: KindFilter::Action,
                selected: selected() == KindFilter::Action,
                value: counts.actions,
                total: counts.total,
                label: tr.kind_actions.to_string(),
                on_select: move |_| set_kind(KindFilter::Action),
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
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct BreakdownCounts {
    total: u32,
    notion: u32,
    posts: u32,
    comments: u32,
    actions: u32,
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
