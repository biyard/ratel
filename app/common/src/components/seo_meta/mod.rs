use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Display)]
pub enum Robots {
    #[default]
    #[strum(to_string = "index, follow")]
    IndexFollow,
    #[strum(to_string = "noindex, follow")]
    NoindexFollow,
    #[strum(to_string = "index, nofollow")]
    IndexNofollow,
    #[strum(to_string = "noindex, nofollow")]
    NoindexNofollow,
}

#[component]
pub fn SeoMeta(
    title: String,
    #[props(default)] description: String,
    #[props(default)] image: String,
    #[props(default)] url: String,
    #[props(default = "website".to_string())] og_type: String,
    #[props(default)] keywords: Vec<String>,
    #[props(default)] canonical: String,
    #[props(default)] robots: Robots,
) -> Element {
    let robots = robots.to_string();
    rsx! {
        // Basic page title
        document::Title { "{title}" }

        // Standard meta tags
        document::Meta { name: "description", content: "{description}" }
        if !keywords.is_empty() {
            { let kw = keywords.join(", "); rsx! { document::Meta { name: "keywords", content: "{kw}" } } }
        }
        document::Meta { name: "robots", content: "{robots}" }

        // Open Graph tags
        document::Meta { property: "og:title", content: "{title}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:type", content: "{og_type}" }
        if !url.is_empty() {
            document::Meta { property: "og:url", content: "{url}" }
        }
        if !image.is_empty() {
            document::Meta { property: "og:image", content: "{image}" }
        }

        // Twitter Card tags
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{title}" }
        document::Meta { name: "twitter:description", content: "{description}" }
        if !image.is_empty() {
            document::Meta { name: "twitter:image", content: "{image}" }
        }

        // Canonical URL
        if !canonical.is_empty() {
            document::Link { rel: "canonical", href: "{canonical}" }
        }
    }
}
