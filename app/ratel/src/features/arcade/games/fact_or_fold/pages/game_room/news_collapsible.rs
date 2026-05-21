//! `RoundNewsCollapsible` — shared collapsible news card mounted by
//! `game_room/component.rs` so players can re-read the published
//! subject from any round stage. Built on native `<details>` so no JS
//! is needed. NewsReveal stage keeps its own dedicated `NewsCard`;
//! everywhere else this collapsible is the only news surface.

use crate::features::arcade::games::fact_or_fold::pages::game_room::news_reveal::render_difficulty;
use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::features::arcade::games::fact_or_fold::RoundSubjectResponse;
use crate::*;

#[component]
pub fn RoundNewsCollapsible(
    subject: RoundSubjectResponse,
    #[props(default = false)] default_open: bool,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let difficulty_label = render_difficulty(subject.difficulty);
    let primary_tag = subject
        .category_tags
        .first()
        .cloned()
        .unwrap_or_else(|| tr.news_pill_category_default.to_string());

    let headline = if subject.headline_text.is_empty() {
        tr.news_headline_pending.to_string()
    } else {
        subject.headline_text.clone()
    };

    rsx! {
        details { class: "card news-collapsible", open: default_open,
            summary { class: "news-summary",
                span { class: "news-summary-label", "{tr.news_label}" }
                span { class: "news-summary-title", "{headline}" }
                span { class: "news-summary-toggle", "▾" }
            }
            div { class: "news-body",
                div { class: "news-meta",
                    span { class: "pill", "{primary_tag}" }
                    span { class: "pill purple", "{tr.news_difficulty} {difficulty_label}" }
                }
                h2 { class: "news-headline-full", "{headline}" }
                if !subject.body_excerpt.is_empty() {
                    p { class: "news-excerpt", "{subject.body_excerpt}" }
                }
                div { class: "news-source",
                    span { "📰" }
                    span { "{subject.source_label}" }
                    span { class: "news-source-dot" }
                    span { "{tr.news_source_lock}" }
                }
            }
        }
    }
}
