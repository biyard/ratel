//! Public bylaws reader page. Mirrors
//! `assets/design/sub-team/bylaws-section.html`.
//!
//! Data source: `SubTeamDocument` rows authored via the bylaws-mode
//! doc composer (`TeamSubTeamBylawsComposePage`). Each one is
//! dual-written with a backing `Post` so the card can show likes /
//! comments and link straight to the post detail page.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::controllers::list_team_bylaws_handler;
use crate::features::sub_team::{SubTeamDocumentResponse, SubTeamTranslate};
use crate::*;

const CATEGORY_BYLAWS: &str = "Bylaws";

#[component]
pub fn TeamBylawsPage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_data = team_resource();
    let team_display = if team_data.nickname.is_empty() {
        team_data.username.clone()
    } else {
        team_data.nickname.clone()
    };
    let team_handle = team_data.username.clone();
    let team_initials: String = team_display
        .chars()
        .take(3)
        .collect::<String>()
        .to_uppercase();
    let has_parent = team_data
        .parent_team_id
        .as_deref()
        .is_some_and(|s| !s.is_empty());
    let parent_username = team_data.parent_username.clone();
    let is_admin = matches!(
        team_data.role,
        Some(TeamRole::Owner) | Some(TeamRole::Admin)
    );

    // BYLAWS section source — parent team's bylaws if this team is a
    // recognized sub-team, otherwise this team's own bylaws.
    let bylaws_owner_pk_str = match (has_parent, parent_username.clone()) {
        (true, Some(_)) => team_data.parent_team_id.clone().unwrap_or_default(),
        _ => team_data
            .pk
            .parse::<TeamPartition>()
            .map(|tp| tp.0)
            .unwrap_or_default(),
    };
    let bylaws_owner_team_id: TeamPartition = TeamPartition(bylaws_owner_pk_str.clone());

    let bylaws_owner_for_load = bylaws_owner_team_id.clone();
    let bylaws_resource = use_loader(move || {
        let id = bylaws_owner_for_load.clone();
        async move { list_team_bylaws_handler(id, Some(CATEGORY_BYLAWS.to_string())).await }
    })?;

    let bylaws_docs: Vec<SubTeamDocumentResponse> = bylaws_resource().items.clone();
    let bylaws_count = bylaws_docs.len();

    // Add gating: only teams without a parent (=상위팀) manage the
    // BYLAWS section themselves; ClubBylaws is editable by any admin.
    let can_add_bylaw = is_admin && !has_parent;

    let username_for_back = username.clone();
    let username_for_add_bylaw = username.clone();

    let bylaws_count_label = tr
        .bylaws_section_count_items
        .replace("{n}", &bylaws_count.to_string());

    rsx! {
        SeoMeta { title: "{tr.bylaws_title}" }

        div { class: "sub-team-bylaws",
            // ── Topbar ─────────────────────────────────────────────
            div { class: "arena-topbar",
                div { class: "arena-topbar__brand",
                    a {
                        class: "brand-home",
                        "aria-label": "Back",
                        href: "/{username_for_back}",
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    span { class: "brand-home__divider" }
                    div { class: "arena-topbar__logo arena-topbar__logo--child", "{team_initials}" }
                    div { class: "u-col",
                        span { class: "arena-topbar__title", "{team_display}" }
                        span { class: "arena-topbar__handle", "@{team_handle}" }
                    }
                    span { class: "arena-topbar__status arena-topbar__status--active",
                        "{tr.bylaws_status_chip}"
                    }
                }
                div { class: "arena-topbar__actions",
                    button { class: "hud-btn", "aria-label": "History",
                        lucide_dioxus::Clock { class: "w-3 h-3 [&>circle]:stroke-current [&>polyline]:stroke-current" }
                    }
                }
            }

            div { class: "page page--wide",
                div { class: "page-header",
                    div { class: "page-header__main",
                        span { class: "page-header__eyebrow",
                            lucide_dioxus::FileText { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.bylaws_page_eyebrow_fr}"
                        }
                        h1 { class: "page-header__title",
                            strong { "{tr.bylaws_page_title_strong}" }
                            "{tr.bylaws_page_title_rest}"
                        }
                        p { class: "page-header__sub", "{tr.bylaws_page_sub}" }
                    }
                }

                // BYLAWS section (gold)
                section { class: "bylaws-section",
                    div { class: "group-header",
                        div { class: "group-header__icon group-header__icon--parent",
                            lucide_dioxus::Star { class: "w-4 h-4 [&>path]:stroke-current" }
                        }
                        h2 { class: "group-header__title", "{tr.bylaws_section_team_regulations}" }
                        span { class: "group-header__count", "{bylaws_count_label}" }
                    }
                    div { class: "bylaws-grid",
                        for (i , doc) in bylaws_docs.iter().enumerate() {
                            BylawCard {
                                key: "{doc.id}",
                                index: i,
                                doc: doc.clone(),
                                parent: true,
                            }
                        }
                        // Mockup-style "+ Add" tile rendered as the last
                        // grid cell. Anchored to the bylaws-compose route
                        // with the section's category.
                        if can_add_bylaw {
                            a {
                                class: "bylaw-add",
                                "data-role": "parent",
                                "data-testid": "sub-team-bylaws-add-bylaw",
                                href: "/{username_for_add_bylaw}/bylaws/compose/Bylaws",
                                lucide_dioxus::Plus { class: "w-5 h-5 [&>path]:stroke-current" }
                                "{tr.bylaws_add_team}"
                            }
                        }
                    }
                    if bylaws_docs.is_empty() && !can_add_bylaw {
                        div { class: "empty-state", "{tr.bylaws_empty}" }
                    }
                }
            }
        }
    }
}

#[component]
fn BylawCard(index: usize, doc: SubTeamDocumentResponse, parent: bool) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();
    let excerpt: String = doc.body.chars().take(240).collect();
    let variant_class = if parent {
        "bylaw-card bylaw-card--parent"
    } else {
        "bylaw-card"
    };
    let testid = if parent {
        "sub-team-bylaws-parent-card"
    } else {
        "sub-team-bylaws-team-card"
    };
    let num_template: String = if parent {
        tr.bylaws_card_bylaw_num.to_string()
    } else {
        tr.bylaws_card_rule_num.to_string()
    };
    let num_label = num_template.replace("{n}", &format!("{:02}", index + 1));
    let meta_label = format_date(doc.updated_at);
    let backing_post_id = doc.backing_post_id.clone();

    rsx! {
        article {
            class: "{variant_class}",
            "data-testid": "{testid}",
            onclick: move |_| {
                let Some(pk) = backing_post_id.clone() else {
                    return;
                };
                let Ok(parsed) = pk.parse::<Partition>() else {
                    return;
                };
                if let Partition::Feed(id) = parsed {
                    nav.push(crate::Route::PostDetail {
                        post_id: FeedPartition(id),
                    });
                }
            },
            div { class: "bylaw-card__top",
                span { class: "bylaw-card__num", "{num_label}" }
                if doc.required {
                    span { class: "bylaw-card__meta", "{tr.bylaws_required_meta}" }
                } else {
                    span { class: "bylaw-card__meta", "{meta_label}" }
                }
            }
            h3 { class: "bylaw-card__title", "{doc.title}" }
            p { class: "bylaw-card__excerpt", "{excerpt}" }
            div { class: "bylaw-card__foot",
                div { class: "bylaw-card__stats",
                    span { class: "bylaw-card__stat",
                        lucide_dioxus::ThumbsUp { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{doc.likes}"
                    }
                    span { class: "bylaw-card__stat",
                        lucide_dioxus::MessageSquare { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{doc.comments}"
                    }
                }
            }
        }
    }
}

fn format_date(ts_ms: i64) -> String {
    if ts_ms <= 0 {
        return String::new();
    }
    use chrono::TimeZone;
    chrono::Utc
        .timestamp_millis_opt(ts_ms)
        .single()
        .map(|t| t.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}
