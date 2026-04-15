use crate::common::hooks::use_infinite_query;
use crate::common::*;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::controllers::list_user_posts::list_team_posts_handler;
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::social::pages::home::HomeTranslate;
use crate::features::social::pages::team_arena::{use_team_arena, TeamArenaTab};
use crate::route::Route;

#[component]
pub fn Home(username: String) -> Element {
    let tr: HomeTranslate = use_translate();
    let nav = use_navigator();

    // Highlight the "Home" tab in the arena topbar.
    let mut arena = use_team_arena();
    use_effect(move || arena.active_tab.set(TeamArenaTab::Home));

    // Fetch team profile (for description, created_at, team pk).
    let team_resource = use_server_future(use_reactive((&username,), |(name,)| async move {
        find_team_handler(name).await
    }))?;
    let team_data = team_resource.read();
    let team = team_data.as_ref().and_then(|r| r.as_ref().ok()).cloned();

    let description_html = team
        .as_ref()
        .map(|t| t.html_contents.clone())
        .unwrap_or_default();
    let created_at = team.as_ref().map(|t| t.created_at).unwrap_or(0);
    let team_pk_str = team.as_ref().map(|t| t.pk.clone());
    let can_edit = arena.can_edit.read().clone();

    // Posts feed with category filter + infinite pagination.
    let mut selected_category: Signal<Option<String>> = use_signal(|| None);
    let mut cat_signal = use_signal(|| selected_category());
    let mut name_signal = use_signal(|| username.clone());

    let mut feed = use_infinite_query(move |bookmark| {
        let name = name_signal();
        let category = cat_signal();
        async move { list_team_posts_handler(name, category, bookmark).await }
    })?;

    let mut feed_clone = feed.clone();
    use_effect(use_reactive(
        (&username, &selected_category),
        move |(name, cat)| {
            let cat_val = cat();
            let changed = *name_signal.peek() != name || *cat_signal.peek() != cat_val;
            if changed {
                name_signal.set(name);
                cat_signal.set(cat_val);
                feed_clone.restart();
            }
        },
    ));

    let items = feed.items();

    // Derived category list with counts (across loaded posts).
    let category_summary = derive_category_summary(&items);
    let total_posts = items.len();
    let total_likes: i64 = items.iter().map(|p| p.likes).sum();
    let total_comments: i64 = items.iter().map(|p| p.comments).sum();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "home-arena", "data-testid": "team-home-arena",

            // ── Team intro strip ─────────────────────────
            div { class: "team-intro",
                if !description_html.is_empty() {
                    div {
                        class: "team-intro__desc",
                        dangerous_inner_html: "{description_html}",
                    }
                } else {
                    div { class: "team-intro__desc", "{tr.no_description}" }
                }
                div { class: "team-intro__meta",
                    span { class: "team-intro__chip",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "4",
                                width: "18",
                                height: "18",
                                rx: "2",
                                ry: "2",
                            }
                            line {
                                x1: "16",
                                y1: "2",
                                x2: "16",
                                y2: "6",
                            }
                            line {
                                x1: "8",
                                y1: "2",
                                x2: "8",
                                y2: "6",
                            }
                            line {
                                x1: "3",
                                y1: "10",
                                x2: "21",
                                y2: "10",
                            }
                        }
                        "{tr.since_label} "
                        strong { {format_since(created_at)} }
                    }
                }
            }

            // ── Section label ────────────────────────────
            div { class: "home-section-label",
                span { class: "home-section-label__dash" }
                span { class: "home-section-label__title",
                    "{tr.team_label} "
                    strong { "{tr.posts_label}" }
                }
                span { class: "home-section-label__dash" }
            }

            // ── Carousel ─────────────────────────────────
            if items.is_empty() {
                div { class: "home-empty-state",
                    div { class: "home-empty-state__title", "{tr.no_posts}" }
                    span { "{tr.no_posts_desc}" }
                }
            } else {
                div { class: "carousel-wrapper",
                    div { class: "carousel-track",
                        for (idx , post) in items.iter().cloned().enumerate() {
                            PostCard { key: "{post.pk}", index: idx, post }
                        }
                    }
                    div { class: "carousel-dots" }
                }
            }

            // Infinite-scroll trigger (hidden unless more pages exist)
            if feed.has_more() {
                {feed.more_element()}
            }

            // ── Bottom bar ───────────────────────────────
            div { class: "home-bottom-bar",
                div { class: "home-bottom-bar__row",
                    HudStat {
                        label: tr.posts_loaded.to_string(),
                        value_main: format!("{total_posts}"),
                        accent: false,
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                polyline { points: "14 2 14 8 20 8" }
                                line {
                                    x1: "16",
                                    y1: "13",
                                    x2: "8",
                                    y2: "13",
                                }
                                line {
                                    x1: "16",
                                    y1: "17",
                                    x2: "8",
                                    y2: "17",
                                }
                                polyline { points: "10 9 9 9 8 9" }
                            }
                        },
                    }
                    HudStat {
                        label: tr.likes_label.to_string(),
                        value_main: format!("{total_likes}"),
                        value_small: Some(tr.on_page_label.to_string()),
                        accent: true,
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3" }
                            }
                        },
                    }
                    HudStat {
                        label: tr.comments_label.to_string(),
                        value_main: format!("{total_comments}"),
                        value_small: Some(tr.on_page_label.to_string()),
                        accent: false,
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
                    div { style: "flex:1" }
                    if can_edit {
                        button {
                            class: "home-browse-btn",
                            r#type: "button",
                            "data-testid": "team-home-create-post",
                            onclick: move |_| {
                                let team_pk = team_pk_str.clone();
                                let nav = nav;
                                async move {
                                    let team_id = team_pk.and_then(|pk| pk.parse().ok());
                                    match create_post_handler(team_id).await {
                                        Ok(resp) => {
                                            let post_pk: FeedPartition = resp.post_pk.into();
                                            nav.push(Route::PostEdit {
                                                post_id: post_pk,
                                            });
                                        }
                                        Err(e) => debug!("Failed to create post: {:?}", e),
                                    }
                                }
                            },
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line {
                                    x1: "12",
                                    y1: "5",
                                    x2: "12",
                                    y2: "19",
                                }
                                line {
                                    x1: "5",
                                    y1: "12",
                                    x2: "19",
                                    y2: "12",
                                }
                            }
                            "{tr.create_post}"
                        }
                    }
                }

                if !category_summary.is_empty() {
                    div { class: "home-bottom-bar__row home-bottom-bar__row--filters",
                        div { class: "cat-filters",
                            CatFilter {
                                label: tr.all_label.to_string(),
                                count: total_posts as i64,
                                active: selected_category().is_none(),
                                onclick: move |_| selected_category.set(None),
                            }
                            for entry in category_summary.iter().cloned() {
                                CatFilterItem {
                                    key: "cat-{entry.0}",
                                    name: entry.0,
                                    count: entry.1,
                                    selected: selected_category,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Sub-components ──────────────────────────────────────

#[component]
fn PostCard(index: usize, post: PostResponse) -> Element {
    let nav = use_navigator();
    let route = post.url();
    let thumbnail = post.urls.first().cloned();
    let category = post.categories.first().cloned();
    let cat_modifier = category_modifier(category.as_deref());
    let preview = strip_html(&post.html_contents);

    rsx! {
        article {
            class: "post-card",
            "data-index": "{index}",
            onclick: move |_| {
                nav.push(route.clone());
            },
            div { class: "post-card__wave" }

            div { class: "post-card__top",
                if let Some(cat) = category.clone() {
                    span { class: "post-card__cat {cat_modifier}",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                        }
                        "{cat}"
                    }
                } else {
                    span {}
                }
                span { class: "post-card__date", {format_post_date(post.created_at)} }
            }

            div { class: "post-card__title", "{post.title}" }

            if let Some(thumb) = thumbnail {
                img { class: "post-card__thumb", src: "{thumb}" }
            }

            if !preview.is_empty() {
                p { class: "post-card__desc", "{preview}" }
            }

            div { class: "post-card__stats",
                div { class: "post-card__stat",
                    span { class: if post.liked { "post-card__stat-value post-card__stat-value--liked" } else { "post-card__stat-value" },
                        "{post.likes}"
                    }
                    span { class: "post-card__stat-label", "Likes" }
                }
                div { class: "post-card__stat",
                    span { class: "post-card__stat-value", "{post.comments}" }
                    span { class: "post-card__stat-label", "Comments" }
                }
                div { class: "post-card__stat",
                    span { class: "post-card__stat-value", "{post.shares}" }
                    span { class: "post-card__stat-label", "Shares" }
                }
            }

            if !post.categories.is_empty() {
                div { class: "post-card__chips",
                    for c in post.categories.iter() {
                        span { key: "{c}", class: "post-card__chip", "#{c}" }
                    }
                }
            }

            div { class: "post-card__footer",
                match post.rewards {
                    Some(r) if r > 0 => rsx! {
                        div { class: "post-card__reward",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                circle { cx: "12", cy: "12", r: "10" }
                                path { d: "M12 6v12" }
                                path { d: "M16 10H8" }
                            }
                            "{r} "
                            small { "reward" }
                        }
                    },
                    _ => rsx! {
                        div { class: "post-card__reward post-card__reward--muted", "—" }
                    },
                }
                span { class: "post-card__cta",
                    "Read Post"
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
            }
        }
    }
}

#[component]
fn HudStat(
    label: String,
    value_main: String,
    #[props(default)] value_small: Option<String>,
    accent: bool,
    icon: Element,
) -> Element {
    let class = if accent {
        "hud-stat hud-stat--accent"
    } else {
        "hud-stat"
    };
    rsx! {
        div { class: "{class}",
            div { class: "hud-stat__icon", {icon} }
            div { class: "hud-stat__body",
                span { class: "hud-stat__label", "{label}" }
                span { class: "hud-stat__value",
                    strong { "{value_main}" }
                    if let Some(s) = value_small {
                        " "
                        small { "{s}" }
                    }
                }
            }
        }
    }
}

#[component]
fn CatFilter(label: String, count: i64, active: bool, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "cat-filter",
            r#type: "button",
            aria_pressed: active,
            onclick: move |_| onclick.call(()),
            "{label} "
            span { class: "cat-filter__count", "{count}" }
        }
    }
}

#[component]
fn CatFilterItem(name: String, count: i64, selected: Signal<Option<String>>) -> Element {
    let active = selected().as_deref() == Some(name.as_str());
    let name_for_click = name.clone();
    rsx! {
        button {
            class: "cat-filter",
            r#type: "button",
            aria_pressed: active,
            onclick: move |_| {
                let current = selected();
                if current.as_deref() == Some(name_for_click.as_str()) {
                    selected.set(None);
                } else {
                    selected.set(Some(name_for_click.clone()));
                }
            },
            "{name} "
            span { class: "cat-filter__count", "{count}" }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────

fn derive_category_summary(posts: &[PostResponse]) -> Vec<(String, i64)> {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<String, i64> = BTreeMap::new();
    for p in posts {
        for c in &p.categories {
            *counts.entry(c.clone()).or_insert(0) += 1;
        }
    }
    counts.into_iter().collect()
}

fn category_modifier(category: Option<&str>) -> &'static str {
    let Some(c) = category else {
        return "";
    };
    let lower = c.to_lowercase();
    if lower.contains("discuss") || lower.contains("dev") {
        "post-card__cat--discuss"
    } else if lower.contains("quiz") || lower.contains("update") {
        "post-card__cat--quiz"
    } else if lower.contains("reward") || lower.contains("governance") {
        "post-card__cat--gold"
    } else if lower.contains("community") {
        "post-card__cat--teal"
    } else {
        ""
    }
}

fn format_post_date(timestamp_ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    match Utc.timestamp_millis_opt(timestamp_ms).single() {
        Some(dt) => dt.format("%b %-d, %Y").to_string(),
        None => String::new(),
    }
}

fn format_since(timestamp_ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    if timestamp_ms == 0 {
        return "—".to_string();
    }
    match Utc.timestamp_millis_opt(timestamp_ms).single() {
        Some(dt) => dt.format("%b %Y").to_string(),
        None => "—".to_string(),
    }
}

fn strip_html(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut in_tag = false;
    for ch in raw.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}
