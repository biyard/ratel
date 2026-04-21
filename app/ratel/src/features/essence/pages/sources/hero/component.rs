use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceHero() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let hook = use_essence_sources()?;

    // Stats come from the `UserEssenceStats` singleton — accurate across
    // the entire user's library regardless of pagination.
    let stats = hook.stats;
    let total_sources = use_memo(move || stats.read().total_sources.max(0) as u64);
    let total_words = use_memo(move || stats.read().total_words.max(0) as u64);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        section { class: "essence-hero",
            div { class: "essence-hero__main",
                span { class: "essence-hero__eyebrow", "{tr.hero_eyebrow}" }
                h1 { class: "essence-hero__title",
                    span { class: "essence-hero__title-count", "{format_thousands(total_sources())}" }
                    " {tr.hero_sources_word} · {format_thousands(total_words())} {tr.hero_words_word}"
                }
                p { class: "essence-hero__sub", "{tr.hero_subtitle}" }
            }
        }
    }
}

/// Format a u64 with thousands separators ("1,204"). Kept local here
/// because the hero and pagination info are the only users; promote to
/// `common::utils` if a third caller shows up.
fn format_thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}
