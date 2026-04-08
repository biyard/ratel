use dioxus::prelude::*;

// ─── PageIntro ─────────────────────────────────────────────────────────────────
// Renders the large title + subtitle header at the top of every page.

#[derive(Props, Clone, PartialEq)]
pub struct PageIntroProps {
    pub title:    String,
    pub subtitle: String,
    #[props(optional)]
    pub badge:    Option<String>,
}

#[component]
pub fn PageIntro(props: PageIntroProps) -> Element {
    rsx! {
        div { class: "ds-page-intro",
            div { class: "flex items-start justify-between flex-wrap gap-4",
                div {
                    if let Some(badge) = &props.badge {
                        span {
                            class: "ds-badge mb-3",
                            style: "background: var(--ratel-color-generic-primary-opacity-10%); color: var(--ratel-color-generic-primary);",
                            { badge.as_str() }
                        }
                    }
                    h1 { class: "ds-page-title", { props.title.as_str() } }
                    p  { class: "ds-page-subtitle", { props.subtitle.as_str() } }
                }
            }
        }
    }
}

// ─── DocSection ───────────────────────────────────────────────────────────────
// A named documentation block: title + optional description + children.
// Used to group a preview, code snippet, token table, etc. under one heading.

#[derive(Props, Clone, PartialEq)]
pub struct DocSectionProps {
    pub title: String,
    #[props(optional)]
    pub description: Option<String>,
    /// Optional anchor id for deep-linking (e.g., id="variants")
    #[props(optional)]
    pub id: Option<String>,
    pub children: Element,
}

#[component]
pub fn DocSection(props: DocSectionProps) -> Element {
    rsx! {
        section {
            class: "ds-section",
            id: props.id.as_deref().unwrap_or(""),

            h2 { class: "ds-section-title", { props.title.as_str() } }

            if let Some(desc) = &props.description {
                p { class: "ds-section-desc", { desc.as_str() } }
            }

            { props.children }
        }
    }
}

// ─── SubSection ───────────────────────────────────────────────────────────────
// H3-level sub-block within a DocSection.

#[derive(Props, Clone, PartialEq)]
pub struct SubSectionProps {
    pub title: String,
    #[props(optional)]
    pub description: Option<String>,
    pub children: Element,
}

#[component]
pub fn SubSection(props: SubSectionProps) -> Element {
    rsx! {
        div { class: "mb-6",
            h3 {
                class: "text-h4 font-semibold mb-1.5",
                style: "color: var(--ratel-color-font-default); letter-spacing: var(--ratel-text-heading-h4-ls);",
                { props.title.as_str() }
            }
            if let Some(desc) = &props.description {
                p {
                    class: "text-body-2 mb-4",
                    style: "color: var(--ratel-color-font-body);",
                    { desc.as_str() }
                }
            }
            { props.children }
        }
    }
}
