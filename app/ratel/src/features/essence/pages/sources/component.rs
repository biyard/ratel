use crate::common::components::SeoMeta;
use crate::features::essence::pages::sources::*;
use crate::*;

/// Top-level page component wired at `/essence`. Initializes the shared
/// `use_essence_sources` hook (which provides the context for every
/// sub-component) and composes the arena sections.
#[component]
pub fn EssenceSourcesPage() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    // Initialise the shared hook here so every sub-component picks it up
    // via context. The `?` surfaces `Loader` suspension to the parent
    // `SuspenseBoundary` rather than each individual child.
    let _hook = use_essence_sources()?;

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }
        SeoMeta { title: "{tr.seo_title}" }

        div { class: "essence-arena",
            EssenceTopbar {}
            main { class: "essence-page",
                EssenceHero {}
                EssenceBreakdown {}
                EssenceControls {}
                EssenceBulkBar {}
                EssenceSourcesTable {}
            }
        }
    }
}
