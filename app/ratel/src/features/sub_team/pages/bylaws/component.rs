//! Public bylaws reader page. Mirrors
//! `assets/design/sub-team/bylaws-section.html`.
//!
//! Uses the public `get_sub_team_apply_context_handler` endpoint to pull
//! the team's required documents. If this team is itself a recognized
//! sub-team, the parent team's required docs are listed in a second
//! section.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::controllers::{
    get_parent_relationship_handler, get_sub_team_apply_context_handler,
};
use crate::features::sub_team::{ApplyContextDocument, SubTeamTranslate};
use crate::*;

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
    let team_pk = team_data.pk.clone();
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    let team_id_for_ctx = team_id.clone();
    use_context_provider(|| team_id_for_ctx);

    let team_id_for_apply = team_id.clone();
    let apply_ctx = use_loader(move || {
        let id = team_id_for_apply.clone();
        async move { get_sub_team_apply_context_handler(id).await }
    })?;

    let team_id_for_rel = team_id.clone();
    let relationship = use_loader(move || {
        let id = team_id_for_rel.clone();
        async move { get_parent_relationship_handler(id).await }
    })?;

    let ctx = apply_ctx();
    let rel = relationship();
    let team_docs: Vec<ApplyContextDocument> = ctx.required_docs.clone();

    // If this team has a recognized parent, load the parent's apply context
    // to surface its regulations as a second section.
    let parent_team_id = rel.parent_team_id.clone();
    let parent_docs_loader = use_loader(move || {
        let id = parent_team_id.clone();
        async move {
            match id {
                Some(id) if !id.is_empty() => {
                    match get_sub_team_apply_context_handler(TeamPartition(id)).await {
                        Ok(ctx) => Ok::<Vec<ApplyContextDocument>, crate::common::Error>(
                            ctx.required_docs,
                        ),
                        Err(_) => Ok(Vec::new()),
                    }
                }
                _ => Ok(Vec::new()),
            }
        }
    })?;
    let parent_docs: Vec<ApplyContextDocument> = parent_docs_loader();

    rsx! {
        SeoMeta { title: "{tr.bylaws_title}" }
        document::Stylesheet { href: asset!("./style.css") }

        div { class: "arena sub-team-bylaws",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "{tr.bylaws_page_eyebrow}" }
                        span { class: "topbar-title__main", "{team_display}" }
                    }
                }
            }

            div { class: "page page--wide",
                // Team's own regulations
                section { class: "bylaws-section",
                    div { class: "group-header",
                        div { class: "group-header__icon group-header__icon--child",
                            lucide_dioxus::FileText { class: "w-4 h-4 [&>path]:stroke-current" }
                        }
                        h2 { class: "group-header__title", "{tr.bylaws_team_regulations}" }
                        span { class: "group-header__count", "{team_docs.len()}" }
                    }

                    if team_docs.is_empty() {
                        div { class: "empty-state", "{tr.bylaws_empty}" }
                    } else {
                        div { class: "bylaws-grid",
                            for doc in team_docs.iter() {
                                BylawCard {
                                    key: "{doc.id}",
                                    doc: doc.clone(),
                                    parent: false,
                                }
                            }
                        }
                    }
                }

                // Parent team's regulations (if recognized sub-team)
                if !parent_docs.is_empty() {
                    section { class: "bylaws-section",
                        div { class: "group-header",
                            div { class: "group-header__icon group-header__icon--parent",
                                lucide_dioxus::Star { class: "w-4 h-4 [&>path]:stroke-current" }
                            }
                            h2 { class: "group-header__title",
                                "{tr.bylaws_parent_regulations}"
                            }
                            span { class: "group-header__count", "{parent_docs.len()}" }
                        }
                        div { class: "bylaws-grid",
                            for doc in parent_docs.iter() {
                                BylawCard {
                                    key: "{doc.id}",
                                    doc: doc.clone(),
                                    parent: true,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BylawCard(doc: ApplyContextDocument, parent: bool) -> Element {
    let tr: SubTeamTranslate = use_translate();
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
    rsx! {
        article { class: "{variant_class}", "data-testid": "{testid}",
            div { class: "bylaw-card__top",
                span { class: "bylaw-card__num", "{tr.bylaws_required_badge}" }
            }
            h3 { class: "bylaw-card__title", "{doc.title}" }
            p { class: "bylaw-card__excerpt", "{excerpt}" }
        }
    }
}
