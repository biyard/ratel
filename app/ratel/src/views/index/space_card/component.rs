use crate::*;

#[derive(Clone, Copy, PartialEq)]
pub enum HeatLevel {
    Blazing,
    Trending,
    Rising,
}

impl HeatLevel {
    fn css(&self) -> &'static str {
        match self {
            HeatLevel::Blazing => "blazing",
            HeatLevel::Trending => "trending",
            HeatLevel::Rising => "rising",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ActionChip {
    pub kind: ChipKind,
    pub label: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChipKind {
    Poll,
    Discuss,
    Quiz,
    Follow,
}

impl ChipKind {
    fn css(&self) -> &'static str {
        match self {
            ChipKind::Poll => "action-chip--poll",
            ChipKind::Discuss => "action-chip--discuss",
            ChipKind::Quiz => "action-chip--quiz",
            ChipKind::Follow => "action-chip--follow",
        }
    }
}

#[component]
pub fn ArenaSpaceCard(
    heat: HeatLevel,
    rank: u32,
    logo: String,
    category: String,
    title: String,
    description: String,
    members: String,
    quests: String,
    heat_delta: String,
    chips: Vec<ActionChip>,
    reward_amount: String,
    #[props(default)] onenter: Option<EventHandler<()>>,
) -> Element {
    let t: super::super::HomeArenaTranslate = use_translate();
    let heat_css = heat.css();
    let heat_label = match heat {
        HeatLevel::Blazing => t.heat_blazing,
        HeatLevel::Trending => t.heat_trending,
        HeatLevel::Rising => t.heat_rising,
    };
    let rank_str = format!("#{:02}", rank);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "space-card space-card--{heat_css}", "data-heat": heat_css,
            div { class: "space-card__wave" }

            div { class: "space-card__top",
                span { class: "space-card__heat space-card__heat--{heat_css}",
                    {heat_icon(heat)}
                    "{heat_label}"
                }
                span { class: "space-card__rank",
                    "{t.rank_label} "
                    strong { "{rank_str}" }
                }
            }

            div { class: "space-card__identity",
                img { class: "space-card__logo", src: "{logo}", alt: "" }
                div { class: "space-card__id",
                    span { class: "space-card__category", "{category}" }
                    span { class: "space-card__title", "{title}" }
                }
            }

            div { class: "space-card__desc", "{description}" }

            div { class: "space-card__stats",
                div { class: "space-stat",
                    span { class: "space-stat__value", "{members}" }
                    span { class: "space-stat__label", "{t.stat_members}" }
                }
                div { class: "space-stat",
                    span { class: "space-stat__value", "{quests}" }
                    span { class: "space-stat__label", "{t.stat_quests}" }
                }
                div { class: "space-stat",
                    span { class: "space-stat__value", "{heat_delta}" }
                    span { class: "space-stat__label", "{t.stat_heat_7d}" }
                }
            }

            div { class: "space-card__chips",
                for chip in chips.iter() {
                    span { class: "action-chip {chip.kind.css()}",
                        {chip_icon(chip.kind)}
                        "{chip.label}"
                    }
                }
            }

            div { class: "space-card__footer",
                div { class: "space-card__reward",
                    {reward_icon()}
                    "{reward_amount} CR"
                    small { "{t.reward_pool}" }
                }
                button {
                    class: "space-card__cta",
                    onclick: move |_| {
                        if let Some(h) = onenter {
                            h.call(());
                        }
                    },
                    "{t.enter_arena}"
                    {chevron_icon()}
                }
            }
        }
    }
}

fn heat_icon(heat: HeatLevel) -> Element {
    match heat {
        HeatLevel::Blazing => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                view_box: "0 0 24 24",
                path { d: "M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" }
            }
        },
        HeatLevel::Trending => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                view_box: "0 0 24 24",
                polyline { points: "22 12 18 12 15 21 9 3 6 12 2 12" }
            }
        },
        HeatLevel::Rising => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                view_box: "0 0 24 24",
                polyline { points: "23 6 13.5 15.5 8.5 10.5 1 18" }
                polyline { points: "17 6 23 6 23 12" }
            }
        },
    }
}

fn chip_icon(kind: ChipKind) -> Element {
    match kind {
        ChipKind::Poll => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                view_box: "0 0 24 24",
                path { d: "M18 20V10" }
                path { d: "M12 20V4" }
                path { d: "M6 20v-6" }
            }
        },
        ChipKind::Discuss => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                view_box: "0 0 24 24",
                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
            }
        },
        ChipKind::Quiz => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                view_box: "0 0 24 24",
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
        ChipKind::Follow => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                view_box: "0 0 24 24",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                circle { cx: "9", cy: "7", r: "4" }
                line {
                    x1: "19",
                    y1: "8",
                    x2: "19",
                    y2: "14",
                }
                line {
                    x1: "22",
                    y1: "11",
                    x2: "16",
                    y2: "11",
                }
            }
        },
    }
}

fn reward_icon() -> Element {
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            circle { cx: "12", cy: "12", r: "10" }
            path { d: "M12 6v12" }
            path { d: "M16 10H8" }
        }
    }
}

fn chevron_icon() -> Element {
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2.5",
            view_box: "0 0 24 24",
            polyline { points: "9 18 15 12 9 6" }
        }
    }
}
