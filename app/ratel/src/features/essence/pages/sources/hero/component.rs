use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceHero() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let hook = use_essence_sources();

    // Stats derived from the current sources list; updates reactively when
    // rows are added/removed/bulked.
    let total_sources = use_memo(move || hook.sources.read().len());
    let total_chunks = use_memo(move || hook.sources.read().iter().map(|s| s.chunks as u64).sum::<u64>());
    // Token estimate (roughly 0.75 tokens per word) — matches the "~3.8M
    // tokens" summary in the mockup even at the mock data's small size.
    let total_tokens_m = use_memo(move || {
        let words: u64 = hook.sources.read().iter().map(|s| s.word_count as u64).sum();
        let tokens = (words as f64) * 1.33 / 1_000_000.0;
        format!("{tokens:.1}M")
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        section { class: "essence-hero",
            div { class: "essence-hero__main",
                span { class: "essence-hero__eyebrow", "{tr.hero_eyebrow}" }
                h1 { class: "essence-hero__title",
                    span { class: "essence-hero__title-count",
                        "{format_thousands(total_sources() as u64)}"
                    }
                    " {tr.hero_sources_word} · {format_thousands(total_chunks())} {tr.hero_chunks_word} · ~{total_tokens_m()} {tr.hero_tokens_word}"
                }
                p { class: "essence-hero__sub", "{tr.hero_subtitle}" }
            }
            button { class: "essence-hero__cta",
                // TODO: wire to the Essence House page once its route exists.
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                    polyline { points: "9 22 9 12 15 12 15 22" }
                }
                "{tr.hero_cta_open_house}"
            }
        }
    }
}

/// Format a u64 with thousands separators ("1,204"). Kept local here
/// because the two other call sites in this page (hero, pagination info)
/// are the only users; promote to `common::utils` if a third caller shows
/// up.
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
