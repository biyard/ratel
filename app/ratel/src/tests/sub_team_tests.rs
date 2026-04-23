use super::*;

use crate::common::types::{EntityType, ListResponse, Partition};
use crate::features::auth::{UserTeam, UserTeamQueryOption};
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::{
    SubTeamApplication, SubTeamApplicationStatus, SubTeamDocument, SubTeamFormField,
    SubTeamFormFieldType, SubTeamLink,
};
use crate::features::sub_team::types::{
    ApplyContextResponse, SubTeamDocumentResponse, SubTeamFormFieldResponse,
    SubTeamSettingsResponse,
};

// ── Helpers ──────────────────────────────────────────────────────

async fn create_parent_team(ctx: &TestContext) -> Partition {
    let owner = &ctx.test_user.0;
    Team::create_new_team(
        owner,
        &ctx.ddb,
        format!("parent{}", uuid::Uuid::new_v4().simple()),
        String::new(),
        format!("p-{}", uuid::Uuid::new_v4().simple()),
        "parent desc".to_string(),
    )
    .await
    .unwrap()
}

fn team_id_from(pk: &Partition) -> String {
    match pk {
        Partition::Team(id) => id.clone(),
        _ => panic!("expected Team pk, got {:?}", pk),
    }
}

// ── Settings ─────────────────────────────────────────────────────

#[tokio::test]
async fn test_update_settings_toggles_parent_eligible() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/settings", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "is_parent_eligible": true,
                "min_sub_team_members": 3
            }
        },
        response_type: SubTeamSettingsResponse,
    };
    assert_eq!(status, 200, "update_settings: {:?}", body);
    assert!(body.is_parent_eligible);
    assert_eq!(body.min_sub_team_members, 3);

    // Verify persisted
    let team = Team::get(&ctx.ddb, &team_pk, Some(EntityType::Team))
        .await
        .unwrap()
        .unwrap();
    assert!(team.is_parent_eligible);
    assert_eq!(team.min_sub_team_members, 3);
}

#[tokio::test]
async fn test_update_settings_requires_admin() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    // Fetch the created team so we can add `other` as a Member role.
    let team = Team::get(&ctx.ddb, &team_pk, Some(EntityType::Team))
        .await
        .unwrap()
        .unwrap();

    let (other_user, other_headers) = ctx.create_another_user().await;
    // Other user is NOT a member of this team — expect rejection.
    let (status_non_member, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/settings", team_id),
        headers: other_headers.clone(),
        body: {
            "body": {
                "is_parent_eligible": true,
            }
        }
    };
    assert_ne!(status_non_member, 200, "non-member must be rejected");

    // Add the other user as a plain Member, then try again — still denied.
    let user_team = UserTeam::new(
        other_user.pk.clone(),
        team.pk.clone(),
        team.display_name.clone(),
        team.profile_url.clone(),
        team.username.clone(),
        team.dao_address.clone(),
        TeamRole::Member,
    );
    user_team.create(&ctx.ddb).await.unwrap();

    let (status_member, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/settings", team_id),
        headers: other_headers,
        body: {
            "body": {
                "is_parent_eligible": true,
            }
        }
    };
    assert_ne!(status_member, 200, "plain member must be rejected");

    // Clean: make sure fully unauthenticated also fails.
    let (status_anon, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/settings", team_id),
        body: {
            "body": {
                "is_parent_eligible": true,
            }
        }
    };
    assert_ne!(status_anon, 200, "anon must be rejected");
}

// ── Form fields ──────────────────────────────────────────────────

#[tokio::test]
async fn test_create_list_update_delete_form_field_round_trip() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    // Create
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "label": "Faculty advisor",
                "field_type": "ShortText",
                "required": true,
                "order": 1,
                "options": []
            }
        },
        response_type: SubTeamFormFieldResponse,
    };
    assert_eq!(status, 200, "create field: {:?}", body);
    assert_eq!(body.label, "Faculty advisor");
    assert!(body.required);
    let field_id = body.id.clone();

    // List
    let (status, _, listed) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<SubTeamFormFieldResponse>,
    };
    assert_eq!(status, 200);
    assert_eq!(listed.items.len(), 1);
    assert_eq!(listed.items[0].id, field_id);

    // Update
    let (status, _, updated) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields/{}", team_id, field_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "label": "Lead advisor"
            }
        },
        response_type: SubTeamFormFieldResponse,
    };
    assert_eq!(status, 200, "update field: {:?}", updated);
    assert_eq!(updated.label, "Lead advisor");

    // Delete
    let (status, _, _) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields/{}", team_id, field_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200);

    let (_, _, after) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<SubTeamFormFieldResponse>,
    };
    assert!(after.items.is_empty(), "after delete: {:?}", after);
}

#[tokio::test]
async fn test_reorder_form_fields_updates_order() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    // Create three fields in order 0, 1, 2.
    let mut created_ids: Vec<String> = Vec::new();
    for i in 0..3i32 {
        let (_, _, body) = crate::test_post! {
            app: ctx.app.clone(),
            path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
            headers: ctx.test_user.1.clone(),
            body: {
                "body": {
                    "label": format!("field{}", i),
                    "field_type": "ShortText",
                    "required": false,
                    "order": i,
                    "options": []
                }
            },
            response_type: SubTeamFormFieldResponse,
        };
        created_ids.push(body.id);
    }

    // Reorder: reverse the list.
    let reversed: Vec<String> = created_ids.iter().rev().cloned().collect();
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields/reorder", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "field_ids": reversed.clone()
            }
        }
    };
    assert_eq!(status, 200, "reorder status");

    // Fetch list and verify order matches reversed.
    let (_, _, listed) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<SubTeamFormFieldResponse>,
    };
    let ids_in_order: Vec<String> = listed.items.iter().map(|f| f.id.clone()).collect();
    assert_eq!(ids_in_order, reversed, "order mismatch: {:?}", listed.items);
}

#[tokio::test]
async fn test_form_field_update_requires_admin() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    // Create field as owner first.
    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "label": "stay",
                "field_type": "ShortText",
                "required": false,
                "order": 0,
                "options": []
            }
        },
        response_type: SubTeamFormFieldResponse,
    };
    let field_id = body.id;

    let (_other_user, other_headers) = ctx.create_another_user().await;

    // Non-member updates -> fail.
    let (status_non_member, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields/{}", team_id, field_id),
        headers: other_headers,
        body: {
            "body": {
                "label": "hacked"
            }
        }
    };
    assert_ne!(status_non_member, 200);

    // Unauthenticated -> fail.
    let (status_anon, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields/{}", team_id, field_id),
        body: {
            "body": {
                "label": "hacked"
            }
        }
    };
    assert_ne!(status_anon, 200);
}

// ── Docs ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_list_update_delete_doc_round_trip() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "Bylaws",
                "body": "# Hello",
                "required": true,
                "order": 0
            }
        },
        response_type: SubTeamDocumentResponse,
    };
    assert_eq!(status, 200, "create doc: {:?}", body);
    assert_eq!(body.title, "Bylaws");
    assert!(body.required);
    assert!(!body.body_hash.is_empty());
    let doc_id = body.id.clone();
    let initial_hash = body.body_hash.clone();

    // List
    let (_, _, listed) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<SubTeamDocumentResponse>,
    };
    assert_eq!(listed.items.len(), 1);
    assert_eq!(listed.items[0].id, doc_id);

    // Update title only (should NOT change hash).
    let (_, _, updated) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs/{}", team_id, doc_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "Club Bylaws"
            }
        },
        response_type: SubTeamDocumentResponse,
    };
    assert_eq!(updated.title, "Club Bylaws");
    assert_eq!(updated.body_hash, initial_hash, "hash must not change when body unchanged");

    // Delete
    let (status, _, _) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs/{}", team_id, doc_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_doc_create_rejects_oversized_body() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    let big = "a".repeat(64 * 1024 + 1);
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "Huge",
                "body": big,
                "required": false,
                "order": 0
            }
        }
    };
    assert_ne!(status, 200, "oversized body should be rejected");
    assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_doc_update_body_rehashes() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "Doc",
                "body": "v1",
                "required": false,
                "order": 0
            }
        },
        response_type: SubTeamDocumentResponse,
    };
    let doc_id = body.id;
    let initial_hash = body.body_hash;

    let (_, _, updated) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs/{}", team_id, doc_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "body": "v2"
            }
        },
        response_type: SubTeamDocumentResponse,
    };
    assert_eq!(updated.body, "v2");
    assert_ne!(updated.body_hash, initial_hash, "hash must change with body");
}

#[tokio::test]
async fn test_reorder_docs_updates_order() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    // Create three docs.
    let mut ids: Vec<String> = Vec::new();
    for i in 0..3 {
        let (_, _, body) = crate::test_post! {
            app: ctx.app.clone(),
            path: &format!("/api/teams/{}/sub-teams/docs", team_id),
            headers: ctx.test_user.1.clone(),
            body: {
                "body": {
                    "title": format!("doc{}", i),
                    "body": "x",
                    "required": false,
                    "order": i
                }
            },
            response_type: SubTeamDocumentResponse,
        };
        ids.push(body.id);
    }

    let reversed: Vec<String> = ids.iter().rev().cloned().collect();
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs/reorder", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "doc_ids": reversed.clone()
            }
        }
    };
    assert_eq!(status, 200);

    let (_, _, listed) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<SubTeamDocumentResponse>,
    };
    let got: Vec<String> = listed.items.iter().map(|d| d.id.clone()).collect();
    assert_eq!(got, reversed);
}

#[tokio::test]
async fn test_doc_update_requires_admin() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    let (_, _, created) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "D",
                "body": "b",
                "required": false,
                "order": 0
            }
        },
        response_type: SubTeamDocumentResponse,
    };
    let doc_id = created.id;

    let (_other_user, other_headers) = ctx.create_another_user().await;

    let (status, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs/{}", team_id, doc_id),
        headers: other_headers,
        body: {
            "body": {
                "title": "hijack"
            }
        }
    };
    assert_ne!(status, 200);

    let (status_anon, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs/{}", team_id, doc_id),
        body: {
            "body": {
                "title": "hijack"
            }
        }
    };
    assert_ne!(status_anon, 200);

    // Also verify list is admin-only.
    let (status_list, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
    };
    assert_ne!(status_list, 200);
}

// ── Apply context ────────────────────────────────────────────────

#[tokio::test]
async fn test_apply_context_returns_form_and_required_docs_for_unauthenticated_caller() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    // Set eligibility.
    let (status, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/settings", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "is_parent_eligible": true,
                "min_sub_team_members": 3
            }
        }
    };
    assert_eq!(status, 200);

    // Create a form field.
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "label": "Advisor",
                "field_type": "ShortText",
                "required": true,
                "order": 0,
                "options": []
            }
        }
    };

    // Create one required doc.
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "Bylaws",
                "body": "# Hi",
                "required": true,
                "order": 0
            }
        }
    };

    // Insert a SubTeamLink directly so recognized_count is 1.
    let link = SubTeamLink::new(
        team_pk.clone(),
        "child-1".to_string(),
        ctx.test_user.0.pk.to_string(),
        "app-1".to_string(),
    );
    link.create(&ctx.ddb).await.unwrap();

    // Insert a pending application directly so pending_count is 1.
    let applicant_pk = Partition::Team("applicant-a".to_string());
    let mut app = SubTeamApplication::new(
        team_pk.clone(),
        applicant_pk,
        team_id.clone(),
        "applicant-a".to_string(),
        ctx.test_user.0.pk.to_string(),
    );
    app.status = SubTeamApplicationStatus::Pending;
    app.create(&ctx.ddb).await.unwrap();

    // Fetch context WITHOUT auth headers.
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/apply-context", team_id),
        response_type: ApplyContextResponse,
    };
    assert_eq!(status, 200, "apply-context: {:?}", body);
    assert!(body.is_parent_eligible);
    assert_eq!(body.min_sub_team_members, 3);
    assert_eq!(body.recognized_count, 1);
    assert_eq!(body.pending_count, 1);
    assert_eq!(body.form_fields.len(), 1);
    assert_eq!(body.form_fields[0].label, "Advisor");
    assert_eq!(body.required_docs.len(), 1);
    assert_eq!(body.required_docs[0].title, "Bylaws");
    assert!(!body.required_docs[0].body_hash.is_empty());
    assert_eq!(body.required_docs[0].body, "# Hi");
}

#[tokio::test]
async fn test_apply_context_only_includes_required_docs() {
    let ctx = TestContext::setup().await;
    let team_pk = create_parent_team(&ctx).await;
    let team_id = team_id_from(&team_pk);

    // required doc
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "Req",
                "body": "b",
                "required": true,
                "order": 0
            }
        }
    };
    // optional doc
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": "Optional",
                "body": "b",
                "required": false,
                "order": 1
            }
        }
    };

    let (status, _, ctx_resp) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/apply-context", team_id),
        response_type: ApplyContextResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(ctx_resp.required_docs.len(), 1);
    assert_eq!(ctx_resp.required_docs[0].title, "Req");
}

// ── Compile-time silence (unused-import guard) ───────────────────
#[allow(dead_code)]
fn _unused_guard() {
    let _ = SubTeamFormField::default();
    let _ = SubTeamDocument::default();
    let _ = SubTeamFormFieldType::ShortText;
    let _ = UserTeamQueryOption::builder();
}
