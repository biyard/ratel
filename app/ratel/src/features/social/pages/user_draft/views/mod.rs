mod i18n;

use dioxus::prelude::*;

use crate::common::hooks::use_infinite_query;
use crate::common::utils::time::{now, time_ago};
use crate::common::FeedPartition;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::posts::controllers::delete_post::delete_post_handler;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::list_user_drafts::list_user_drafts_handler;
use crate::features::social::pages::user_draft::*;
use crate::route::Route;

use i18n::UserDraftsTranslate;

const DAY_MS: i64 = 86_400_000;
const WEEK_MS: i64 = DAY_MS * 7;

#[derive(Clone, Copy, PartialEq)]
enum Bucket {
    Today,
    Week,
    Older,
}

fn bucket_of(updated_at: i64) -> Bucket {
    let diff = now() - updated_at;
    if diff < DAY_MS {
        Bucket::Today
    } else if diff < WEEK_MS {
        Bucket::Week
    } else {
        Bucket::Older
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Today,
    Week,
    Older,
    Space,
}

#[derive(Clone, Copy, PartialEq)]
enum SortMode {
    Recent,
    Oldest,
    Title,
    Words,
}

fn strip_html(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn word_count(html: &str) -> usize {
    strip_html(html).split_whitespace().count()
}

fn format_commas(n: i64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

#[component]
pub fn Home(username: String) -> Element {
    let _ = username;
    let tr: UserDraftsTranslate = use_translate();
    let nav = use_navigator();

    let go_create_post = use_callback(move |_: ()| {
        spawn(async move {
            match create_post_handler(None).await {
                Ok(resp) => {
                    nav.push(Route::PostEdit {
                        post_id: resp.post_pk.into(),
                    });
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Failed to create post: {:?}", e);
                }
            }
        });
    });

    let mut drafts_query = use_infinite_query(move |bookmark| async move {
        list_user_drafts_handler(bookmark).await
    })?;

    let deleted = use_signal(std::collections::HashSet::<String>::new);
    let mut filter = use_signal(|| Filter::All);
    let mut sort_mode = use_signal(|| SortMode::Recent);
    let mut sort_open = use_signal(|| false);
    let menu_open_id = use_signal(|| Option::<String>::None);

    let deleted_keys = deleted.read().clone();
    let items: Vec<PostResponse> = drafts_query
        .items()
        .into_iter()
        .filter(|post| !deleted_keys.contains(&post.pk.to_string()))
        .collect();

    let all_count = items.len();
    let today_count = items.iter().filter(|p| bucket_of(p.updated_at) == Bucket::Today).count();
    let week_count = items.iter().filter(|p| bucket_of(p.updated_at) == Bucket::Week).count();
    let older_count = items.iter().filter(|p| bucket_of(p.updated_at) == Bucket::Older).count();
    let space_count = items.iter().filter(|p| p.has_space()).count();
    let total_words: usize = items.iter().map(|p| word_count(&p.html_contents)).sum();
    let last_edited = items.iter().map(|p| p.updated_at).max();

    let last_edited_text = match last_edited {
        Some(ts) => time_ago(ts),
        None => tr.time_never.to_string(),
    };

    let filtered: Vec<PostResponse> = items
        .iter()
        .filter(|p| match filter() {
            Filter::All => true,
            Filter::Today => bucket_of(p.updated_at) == Bucket::Today,
            Filter::Week => bucket_of(p.updated_at) == Bucket::Week,
            Filter::Older => bucket_of(p.updated_at) == Bucket::Older,
            Filter::Space => p.has_space(),
        })
        .cloned()
        .collect();

    let mut sorted = filtered.clone();
    match sort_mode() {
        SortMode::Recent => sorted.sort_by(|a, b| b.updated_at.cmp(&a.updated_at)),
        SortMode::Oldest => sorted.sort_by(|a, b| a.updated_at.cmp(&b.updated_at)),
        SortMode::Title => sorted.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase())),
        SortMode::Words => sorted.sort_by(|a, b| word_count(&b.html_contents).cmp(&word_count(&a.html_contents))),
    }

    let mut today_posts: Vec<PostResponse> = Vec::new();
    let mut week_posts: Vec<PostResponse> = Vec::new();
    let mut older_posts: Vec<PostResponse> = Vec::new();
    for p in &sorted {
        match bucket_of(p.updated_at) {
            Bucket::Today => today_posts.push(p.clone()),
            Bucket::Week => week_posts.push(p.clone()),
            Bucket::Older => older_posts.push(p.clone()),
        }
    }

    let sort_label = match sort_mode() {
        SortMode::Recent => tr.sort_recent,
        SortMode::Oldest => tr.sort_oldest,
        SortMode::Title => tr.sort_title,
        SortMode::Words => tr.sort_words,
    };

    let sorted_is_empty = sorted.is_empty();
    let words_formatted = format_commas(total_words as i64);

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap",
        }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "drafts-arena",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    button {
                        class: "back-btn",
                        "aria-label": tr.back,
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__main", "{tr.page_title}" }
                        span { class: "topbar-title__count",
                            strong { "{all_count}" }
                            " {tr.total_label}"
                        }
                    }
                }
                div { class: "arena-topbar__right",
                    button {
                        class: "topbar-btn topbar-btn--primary",
                        onclick: move |_| go_create_post.call(()),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
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
                        "{tr.new_post}"
                    }
                }
            }

            div { class: "page",
                div { class: "stats-strip",
                    div { class: "stat-card",
                        div { class: "stat-card__icon",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                polyline { points: "14 2 14 8 20 8" }
                            }
                        }
                        div { class: "stat-card__body",
                            span { class: "stat-card__label", "{tr.stat_total}" }
                            span { class: "stat-card__value", "{all_count}" }
                        }
                    }
                    div { class: "stat-card stat-card--teal",
                        div { class: "stat-card__icon",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                                path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                            }
                        }
                        div { class: "stat-card__body",
                            span { class: "stat-card__label", "{tr.stat_words}" }
                            span { class: "stat-card__value",
                                "{words_formatted} "
                                small { "{tr.unit_words}" }
                            }
                        }
                    }
                    div { class: "stat-card",
                        div { class: "stat-card__icon",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "10" }
                                polyline { points: "12 6 12 12 16 14" }
                            }
                        }
                        div { class: "stat-card__body",
                            span { class: "stat-card__label", "{tr.stat_last}" }
                            span { class: "stat-card__value", "{last_edited_text}" }
                        }
                    }
                }

                div { class: "filter-row",
                    div { class: "filter-chips",
                        FilterChip {
                            label: tr.filter_all,
                            count: all_count,
                            selected: filter() == Filter::All,
                            on_click: move |_| filter.set(Filter::All),
                        }
                        FilterChip {
                            label: tr.filter_today,
                            count: today_count,
                            selected: filter() == Filter::Today,
                            on_click: move |_| filter.set(Filter::Today),
                        }
                        FilterChip {
                            label: tr.filter_week,
                            count: week_count,
                            selected: filter() == Filter::Week,
                            on_click: move |_| filter.set(Filter::Week),
                        }
                        FilterChip {
                            label: tr.filter_older,
                            count: older_count,
                            selected: filter() == Filter::Older,
                            on_click: move |_| filter.set(Filter::Older),
                        }
                        FilterChip {
                            label: tr.filter_space,
                            count: space_count,
                            selected: filter() == Filter::Space,
                            on_click: move |_| filter.set(Filter::Space),
                        }
                    }
                    div {
                        class: "sort",
                        id: "drafts-sort",
                        "data-open": sort_open(),
                        button {
                            class: "sort__btn",
                            onclick: move |e: MouseEvent| {
                                e.stop_propagation();
                                let next = !sort_open();
                                sort_open.set(next);
                            },
                            small { "{tr.sort_label}" }
                            span { "{sort_label}" }
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "6 9 12 15 18 9" }
                            }
                        }
                        div { class: "sort__menu", role: "listbox",
                            SortItem {
                                label: tr.sort_recent,
                                selected: sort_mode() == SortMode::Recent,
                                on_click: move |_| {
                                    sort_mode.set(SortMode::Recent);
                                    sort_open.set(false);
                                },
                            }
                            SortItem {
                                label: tr.sort_oldest,
                                selected: sort_mode() == SortMode::Oldest,
                                on_click: move |_| {
                                    sort_mode.set(SortMode::Oldest);
                                    sort_open.set(false);
                                },
                            }
                            SortItem {
                                label: tr.sort_title,
                                selected: sort_mode() == SortMode::Title,
                                on_click: move |_| {
                                    sort_mode.set(SortMode::Title);
                                    sort_open.set(false);
                                },
                            }
                            SortItem {
                                label: tr.sort_words,
                                selected: sort_mode() == SortMode::Words,
                                on_click: move |_| {
                                    sort_mode.set(SortMode::Words);
                                    sort_open.set(false);
                                },
                            }
                        }
                    }
                }

                if all_count == 0 {
                    EmptyState {
                        title: tr.empty_title,
                        desc: tr.empty_desc,
                        cta: tr.empty_cta,
                        on_cta: move |_| go_create_post.call(()),
                    }
                } else if sorted_is_empty {
                    EmptyState {
                        title: tr.empty_filtered_title,
                        desc: tr.empty_filtered_desc,
                        cta: tr.filter_all,
                        on_cta: move |_| filter.set(Filter::All),
                    }
                } else {
                    DraftSection {
                        title: tr.section_today,
                        posts: today_posts,
                        deleted,
                        menu_open_id,
                        tr: tr.clone(),
                    }
                    DraftSection {
                        title: tr.section_week,
                        posts: week_posts,
                        deleted,
                        menu_open_id,
                        tr: tr.clone(),
                    }
                    DraftSection {
                        title: tr.section_older,
                        posts: older_posts,
                        deleted,
                        menu_open_id,
                        tr: tr.clone(),
                    }
                }

                if drafts_query.has_more() {
                    {drafts_query.more_element()}
                }
            }
        }
    }
}

#[component]
fn FilterChip(label: String, count: usize, selected: bool, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "chip",
            "aria-selected": selected,
            onclick: move |_| on_click.call(()),
            "{label}"
            span { class: "chip__count", "{count}" }
        }
    }
}

#[component]
fn SortItem(label: String, selected: bool, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "sort__item",
            role: "option",
            "aria-selected": selected,
            onclick: move |_| on_click.call(()),
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "3",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                polyline { points: "20 6 9 17 4 12" }
            }
            "{label}"
        }
    }
}

#[component]
fn DraftSection(
    title: String,
    posts: Vec<PostResponse>,
    deleted: Signal<std::collections::HashSet<String>>,
    menu_open_id: Signal<Option<String>>,
    tr: UserDraftsTranslate,
) -> Element {
    if posts.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "section-label",
            span { class: "section-label__title", "{title}" }
            span { class: "section-label__line" }
        }
        div { class: "drafts-list",
            for post in posts.iter() {
                DraftCard {
                    key: "{post.pk}",
                    post: post.clone(),
                    deleted,
                    menu_open_id,
                    tr: tr.clone(),
                }
            }
        }
    }
}

#[component]
fn DraftCard(
    post: PostResponse,
    mut deleted: Signal<std::collections::HashSet<String>>,
    mut menu_open_id: Signal<Option<String>>,
    tr: UserDraftsTranslate,
) -> Element {
    let nav = use_navigator();
    let pk_str = post.pk.to_string();
    let menu_open = menu_open_id().as_deref() == Some(pk_str.as_str());
    let is_writing = now() - post.updated_at < 5 * 60 * 1000;

    let excerpt = strip_html(&post.html_contents);
    let excerpt_trim = excerpt.trim();
    let (excerpt_text, excerpt_empty) = if excerpt_trim.is_empty() {
        (tr.empty_excerpt.to_string(), true)
    } else {
        (excerpt_trim.chars().take(220).collect::<String>(), false)
    };

    let title_text = post.title.trim().to_string();
    let (title_display, title_untitled) = if title_text.is_empty() {
        (tr.untitled.to_string(), true)
    } else {
        (title_text, false)
    };

    let words = word_count(&post.html_contents);
    let image_count = post.urls.len();
    let has_space = post.has_space();
    let saved_ago = time_ago(post.updated_at);
    let tags = post.categories.clone();

    let card_class = if is_writing { "draft-card draft-card--writing" } else { "draft-card" };

    let pk_for_resume = post.pk.clone();
    let pk_for_menu_resume = post.pk.clone();
    let pk_for_delete = post.pk.clone();
    let pk_for_card = pk_str.clone();
    let pk_for_toggle = pk_str.clone();

    rsx! {
        div {
            class: "{card_class}",
            "data-menu-open": menu_open,
            onclick: move |_| {
                let pk: FeedPartition = pk_for_resume.clone().into();
                nav.push(format!("/posts/{pk}/edit"));
            },
            div { class: "draft-card__dot",
                span {}
            }
            div { class: "draft-card__body",
                div { class: "draft-card__title-row",
                    if title_untitled {
                        span { class: "draft-card__title draft-card__title--untitled",
                            "{title_display}"
                        }
                    } else {
                        span { class: "draft-card__title", "{title_display}" }
                    }
                    if is_writing {
                        span { class: "draft-card__badge draft-card__badge--writing",
                            "{tr.badge_writing}"
                        }
                    }
                    if has_space {
                        span { class: "draft-card__badge draft-card__badge--space",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                circle { cx: "12", cy: "12", r: "10" }
                                path { d: "M2 12h20" }
                                path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                            }
                            "{tr.badge_space}"
                        }
                    }
                }
                if excerpt_empty {
                    p { class: "draft-card__excerpt draft-card__excerpt--empty", "{excerpt_text}" }
                } else {
                    p { class: "draft-card__excerpt", "{excerpt_text}" }
                }
                div { class: "draft-card__meta",
                    span { class: "draft-card__meta-item",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                            path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                        }
                        strong { "{words}" }
                        " {tr.unit_words}"
                    }
                    if image_count > 0 {
                        span { class: "draft-card__meta-item",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                rect {
                                    x: "3",
                                    y: "3",
                                    width: "18",
                                    height: "18",
                                    rx: "2",
                                    ry: "2",
                                }
                                circle { cx: "8.5", cy: "8.5", r: "1.5" }
                                polyline { points: "21 15 16 10 5 21" }
                            }
                            strong { "{image_count}" }
                            " {tr.meta_images}"
                        }
                    }
                    span { class: "draft-card__meta-item",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            circle { cx: "12", cy: "12", r: "10" }
                            polyline { points: "12 6 12 12 16 14" }
                        }
                        "{tr.meta_saved} "
                        strong { "{saved_ago}" }
                    }
                    if !tags.is_empty() {
                        span { class: "draft-card__tags",
                            for tag in tags.iter() {
                                span { class: "draft-card__tag", "{tag}" }
                            }
                        }
                    }
                }
            }
            div { class: "draft-card__actions",
                button {
                    class: "draft-card__resume",
                    onclick: {
                        let pk = post.pk.clone();
                        move |e: MouseEvent| {
                            e.stop_propagation();
                            let pk: FeedPartition = pk.clone().into();
                            nav.push(format!("/posts/{pk}/edit"));
                        }
                    },
                    "{tr.resume}"
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
                button {
                    class: "draft-card__more",
                    "aria-label": tr.more_options,
                    onclick: move |e: MouseEvent| {
                        e.stop_propagation();
                        let pk = pk_for_toggle.clone();
                        let current = menu_open_id();
                        if current.as_deref() == Some(pk.as_str()) {
                            menu_open_id.set(None);
                        } else {
                            menu_open_id.set(Some(pk));
                        }
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "1" }
                        circle { cx: "12", cy: "5", r: "1" }
                        circle { cx: "12", cy: "19", r: "1" }
                    }
                }
            }
            div {
                class: "draft-menu",
                onclick: move |e: MouseEvent| e.stop_propagation(),
                button {
                    class: "draft-menu__item",
                    onclick: move |e: MouseEvent| {
                        e.stop_propagation();
                        let pk: FeedPartition = pk_for_menu_resume.clone().into();
                        menu_open_id.set(None);
                        nav.push(format!("/posts/{pk}/edit"));
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                    "{tr.menu_resume}"
                }
                div { class: "draft-menu__separator" }
                button {
                    class: "draft-menu__item draft-menu__item--danger",
                    onclick: move |e: MouseEvent| {
                        e.stop_propagation();
                        menu_open_id.set(None);
                        let pk = pk_for_delete.clone();
                        let pk_id = pk_for_card.clone();
                        spawn(async move {
                            if delete_post_handler(pk.clone(), None).await.is_ok() {
                                deleted.write().insert(pk_id);
                            }
                        });
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "3 6 5 6 21 6" }
                        path { d: "M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6" }
                    }
                    "{tr.menu_delete}"
                }
            }
        }
    }
}

#[component]
fn EmptyState(
    title: String,
    desc: String,
    cta: String,
    on_cta: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "empty-state",
            div { class: "empty-state__icon",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.6",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M12 20h9" }
                    path { d: "M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" }
                }
            }
            div { class: "empty-state__title", "{title}" }
            div { class: "empty-state__desc", "{desc}" }
            button { class: "empty-state__cta", onclick: move |_| on_cta.call(()),
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
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
                "{cta}"
            }
        }
    }
}
