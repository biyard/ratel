use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// Dashed-border card with "+ Add chapter" CTA at the bottom of the
/// chapter list. Clicking it triggers the on_add callback which
/// creates a new chapter via the server function.
#[component]
pub fn AddChapterSlot(on_add: EventHandler) -> Element {
    let tr: GamificationTranslate = use_translate();

    rsx! {
        div {
            class: "flex flex-col gap-2 justify-center items-center p-5 w-full border-2 border-dashed transition-colors duration-150 cursor-pointer rounded-[12px] border-border hover:border-primary/40",
            "data-testid": "add-chapter-slot",
            onclick: move |_| on_add.call(()),

            Row { cross_axis_align: CrossAxisAlign::Center, class: "gap-2",
                lucide_dioxus::Plus { class: "w-5 h-5 text-primary" }
                span { class: "text-sm font-semibold text-primary", "{tr.add_chapter}" }
            }

            span { class: "text-xs text-foreground-muted", "{tr.add_chapter_hint}" }
        }
    }
}
