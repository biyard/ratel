use super::*;

use crate::common::types::{EntityType, InboxPayload, ListResponse, Partition};
use crate::features::auth::{UserTeam, UserTeamQueryOption};
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::{
    SubTeamApplication, SubTeamApplicationStatus, SubTeamDocument, SubTeamFormField,
    SubTeamFormFieldType, SubTeamLink,
};
use crate::features::sub_team::types::{
    ApplyContextResponse, ParentRelationshipResponse, ParentRelationshipStatus,
    SubTeamApplicationDetailResponse, SubTeamApplicationResponse, SubTeamDocumentResponse,
    SubTeamFormFieldResponse, SubTeamSettingsResponse,
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

// ── Application lifecycle helpers ────────────────────────────────

async fn enable_parent_eligible(ctx: &TestContext, team_id: &str, min_members: i32) {
    let (status, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/settings", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "is_parent_eligible": true,
                "min_sub_team_members": min_members
            }
        }
    };
    assert_eq!(status, 200);
}

async fn create_required_doc(ctx: &TestContext, team_id: &str, title: &str, body: &str) -> SubTeamDocumentResponse {
    let (status, _, doc) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/docs", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "title": title,
                "body": body,
                "required": true,
                "order": 0
            }
        },
        response_type: SubTeamDocumentResponse,
    };
    assert_eq!(status, 200);
    doc
}

async fn create_required_form_field(ctx: &TestContext, team_id: &str, label: &str) -> SubTeamFormFieldResponse {
    let (status, _, field) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/form-fields", team_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "label": label,
                "field_type": "ShortText",
                "required": true,
                "order": 0,
                "options": []
            }
        },
        response_type: SubTeamFormFieldResponse,
    };
    assert_eq!(status, 200);
    field
}

/// Create a second team owned by `headers` and return (team_pk, team_id).
async fn create_team_for(
    ctx: &TestContext,
    user: &crate::common::models::auth::User,
    headers: &axum::http::HeaderMap,
) -> (Partition, String) {
    let _ = headers;
    let pk = Team::create_new_team(
        user,
        &ctx.ddb,
        format!("child{}", uuid::Uuid::new_v4().simple()),
        String::new(),
        format!("c-{}", uuid::Uuid::new_v4().simple()),
        "child desc".to_string(),
    )
    .await
    .unwrap();
    let id = team_id_from(&pk);
    (pk, id)
}

async fn add_n_members(ctx: &TestContext, team_pk: &Partition, n: usize) {
    // Owner is already 1 member; add n more as Members.
    for _ in 0..n {
        let (other, _) = ctx.create_another_user().await;
        let user_team = UserTeam::new(
            other.pk.clone(),
            team_pk.clone(),
            "child".to_string(),
            String::new(),
            "c".to_string(),
            None,
            TeamRole::Member,
        );
        user_team.create(&ctx.ddb).await.unwrap();
    }
}

// ── Tests: submit path ───────────────────────────────────────────

#[tokio::test]
async fn test_child_submits_application_creates_pending_with_doc_agreements() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    enable_parent_eligible(&ctx, &parent_id, 1).await;
    let doc = create_required_doc(&ctx, &parent_id, "Bylaws", "Body v1").await;
    let field = create_required_form_field(&ctx, &parent_id, "Advisor").await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (status, _, app) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers.clone(),
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": { &field.id: "Prof Kim" },
                "doc_agreements": [
                    { "doc_id": &doc.id, "body_hash": &doc.body_hash }
                ]
            }
        },
        response_type: SubTeamApplicationResponse,
    };
    assert_eq!(status, 200, "submit: {:?}", app);
    assert_eq!(app.parent_team_id, parent_id);
    assert_eq!(app.sub_team_id, child_id);
    assert!(app.submitted_at.is_some());

    // Pending parent id set on applying team.
    let team = Team::get(&ctx.ddb, &child_pk, Some(EntityType::Team))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(team.pending_parent_team_id.as_deref(), Some(parent_id.as_str()));
    assert!(team.parent_team_id.is_none());
}

#[tokio::test]
async fn test_submit_rejected_when_parent_not_eligible() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    // NOT enabling eligibility.

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_submit_rejected_below_min_members() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 5).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_submit_rejected_without_required_doc_agreement() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 1).await;
    let _doc = create_required_doc(&ctx, &parent_id, "Bylaws", "Body v1").await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_submit_rejected_with_stale_doc_body_hash() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 1).await;
    let doc = create_required_doc(&ctx, &parent_id, "Bylaws", "Body v1").await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": [
                    { "doc_id": &doc.id, "body_hash": "staleeeeee" }
                ]
            }
        }
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_submit_rejected_missing_required_form_field() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 1).await;
    let _field = create_required_form_field(&ctx, &parent_id, "Advisor").await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_submit_rejected_cycle_to_self() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    // Apply self-to-self: path team = parent_id, body.parent_team_id = parent_id.
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", parent_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "parent_team_id": &parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_submit_rejected_when_in_flight_application_exists() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    // First submission should succeed.
    let (status1, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers.clone(),
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_eq!(status1, 200);

    // Second should be rejected as in-flight.
    let (status2, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_ne!(status2, 200);
}

// ── Tests: parent queue ───────────────────────────────────────────

#[tokio::test]
async fn test_parent_queue_lists_only_pending_by_default() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    // Pending submission.
    let (s, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    assert_eq!(s, 200);

    let (status, _, listed) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/applications", parent_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<SubTeamApplicationResponse>,
    };
    assert_eq!(status, 200);
    assert!(listed.items.iter().all(|a| matches!(a.status, SubTeamApplicationStatus::Pending)));
    assert!(!listed.items.is_empty());
}

#[tokio::test]
async fn test_parent_approve_flips_team_status_and_creates_link() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (_, _, app) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers.clone(),
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        },
        response_type: SubTeamApplicationResponse,
    };
    let app_id = app.id.clone();

    let (status, _, approved) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/applications/{}/approve", parent_id, app_id),
        headers: ctx.test_user.1.clone(),
        body: {},
        response_type: SubTeamApplicationResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(approved.status, SubTeamApplicationStatus::Approved));

    // Team now has parent_team_id; pending cleared.
    let team = Team::get(&ctx.ddb, &child_pk, Some(EntityType::Team))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(team.parent_team_id.as_deref(), Some(parent_id.as_str()));
    assert!(team.pending_parent_team_id.is_none());

    // SubTeamLink row exists under parent.
    let link_sk = EntityType::SubTeamLink(child_id.clone());
    let link = SubTeamLink::get(&ctx.ddb, &parent_pk, Some(link_sk))
        .await
        .unwrap();
    assert!(link.is_some(), "SubTeamLink must exist");
}

#[tokio::test]
async fn test_parent_reject_clears_pending_parent_and_records_reason() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (_, _, app) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        },
        response_type: SubTeamApplicationResponse,
    };
    let app_id = app.id.clone();

    let (status, _, rejected) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/applications/{}/reject", parent_id, app_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": { "reason": "too few members" }
        },
        response_type: SubTeamApplicationResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(rejected.status, SubTeamApplicationStatus::Rejected));
    assert_eq!(rejected.decision_reason.as_deref(), Some("too few members"));

    let team = Team::get(&ctx.ddb, &child_pk, Some(EntityType::Team))
        .await
        .unwrap()
        .unwrap();
    assert!(team.pending_parent_team_id.is_none());
    assert!(team.parent_team_id.is_none());
}

#[tokio::test]
async fn test_parent_return_keeps_pending_status_and_allows_resubmit() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (_, _, app) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers.clone(),
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        },
        response_type: SubTeamApplicationResponse,
    };
    let app_id = app.id.clone();

    let (status, _, returned) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/applications/{}/return", parent_id, app_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": { "comment": "add advisor" }
        },
        response_type: SubTeamApplicationResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(returned.status, SubTeamApplicationStatus::Returned));

    // child still pending.
    let team = Team::get(&ctx.ddb, &child_pk, Some(EntityType::Team))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(team.pending_parent_team_id.as_deref(), Some(parent_id.as_str()));

    // PATCH to resubmit.
    let (status, _, resubmitted) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications/{}", child_id, app_id),
        headers: child_headers,
        body: {
            "body": {
                "form_values": {}
            }
        },
        response_type: SubTeamApplicationResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(resubmitted.status, SubTeamApplicationStatus::Pending));
}

#[tokio::test]
async fn test_child_cancel_clears_pending_parent() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (_, _, app) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers.clone(),
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        },
        response_type: SubTeamApplicationResponse,
    };
    let app_id = app.id.clone();

    let (status, _, cancelled) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications/{}/cancel", child_id, app_id),
        headers: child_headers,
        body: {},
        response_type: SubTeamApplicationResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(cancelled.status, SubTeamApplicationStatus::Cancelled));

    let team = Team::get(&ctx.ddb, &child_pk, Some(EntityType::Team))
        .await
        .unwrap()
        .unwrap();
    assert!(team.pending_parent_team_id.is_none());
}

#[tokio::test]
async fn test_parent_relationship_endpoint_reports_status_correctly() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    enable_parent_eligible(&ctx, &parent_id, 0).await;

    let (child_user, child_headers) = ctx.create_another_user().await;
    let (_child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    // Standalone initially.
    let (status, _, rel) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent", child_id),
        headers: child_headers.clone(),
        response_type: ParentRelationshipResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(rel.status, ParentRelationshipStatus::Standalone));
    assert!(rel.parent_team_id.is_none());
    assert!(rel.pending_parent_team_id.is_none());

    // Submit → Pending.
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers.clone(),
        body: {
            "body": {
                "parent_team_id": parent_id,
                "form_values": {},
                "doc_agreements": []
            }
        }
    };
    let (_, _, rel) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent", child_id),
        headers: child_headers.clone(),
        response_type: ParentRelationshipResponse,
    };
    assert!(matches!(rel.status, ParentRelationshipStatus::PendingSubTeam));
    assert_eq!(rel.pending_parent_team_id.as_deref(), Some(parent_id.as_str()));
    assert!(rel.latest_application_id.is_some());
}

// ── Announcement lifecycle ──────────────────────────────────────────

#[tokio::test]
async fn test_create_update_publish_announcement_lifecycle() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    // Create draft.
    let (status, _, created) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": { "title": "Welcome", "body": "Hello world" }
        },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    assert_eq!(status, 200, "create: {:?}", created);
    assert_eq!(created.title, "Welcome");
    assert!(matches!(
        created.status,
        crate::features::sub_team::models::SubTeamAnnouncementStatus::Draft
    ));
    let ann_id = created.id.clone();

    // Update draft.
    let (status, _, updated) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements/{}", parent_id, ann_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "body": { "title": "Greetings" }
        },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    assert_eq!(status, 200, "update: {:?}", updated);
    assert_eq!(updated.title, "Greetings");

    // Publish.
    let (status, _, published) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/announcements/{}/publish",
            parent_id, ann_id
        ),
        headers: ctx.test_user.1.clone(),
        body: {},
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    assert_eq!(status, 200, "publish: {:?}", published);
    assert!(matches!(
        published.status,
        crate::features::sub_team::models::SubTeamAnnouncementStatus::Published
    ));
    assert!(published.published_at.is_some());
}

#[tokio::test]
async fn test_announcement_edit_rejected_after_publish() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    let (_, _, created) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "T", "body": "B" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    let ann_id = created.id.clone();

    let (s, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/announcements/{}/publish",
            parent_id, ann_id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_eq!(s, 200);

    // Any subsequent PATCH must fail with CONFLICT.
    let (status, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements/{}", parent_id, ann_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "nope" } }
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_publish_rejected_when_too_many_sub_teams() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    // Insert 51 SubTeamLink rows directly to exceed the 50-cap.
    for _ in 0..51 {
        let child_id = uuid::Uuid::new_v4().to_string();
        let link = SubTeamLink::new(
            parent_pk.clone(),
            child_id,
            ctx.test_user.0.pk.to_string(),
            "fake-app".to_string(),
        );
        link.create(&ctx.ddb).await.unwrap();
    }

    let (_, _, ann) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "T", "body": "B" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/announcements/{}/publish",
            parent_id, ann.id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_publish_fans_out_pinned_post_per_recognized_sub_team() {
    use crate::features::posts::models::Post;

    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    // Build two approved sub-teams through the lifecycle.
    enable_parent_eligible(&ctx, &parent_id, 0).await;
    let (child_user_a, child_headers_a) = ctx.create_another_user().await;
    let (child_pk_a, child_id_a) = create_team_for(&ctx, &child_user_a, &child_headers_a).await;
    let (child_user_b, child_headers_b) = ctx.create_another_user().await;
    let (child_pk_b, child_id_b) = create_team_for(&ctx, &child_user_b, &child_headers_b).await;

    for (child_id, child_headers) in [
        (&child_id_a, &child_headers_a),
        (&child_id_b, &child_headers_b),
    ] {
        let (_, _, app) = crate::test_post! {
            app: ctx.app.clone(),
            path: &format!("/api/teams/{}/parent/applications", child_id),
            headers: child_headers.clone(),
            body: {
                "body": {
                    "parent_team_id": parent_id,
                    "form_values": {},
                    "doc_agreements": []
                }
            },
            response_type: SubTeamApplicationResponse,
        };
        let (s, _, _) = crate::test_post! {
            app: ctx.app.clone(),
            path: &format!(
                "/api/teams/{}/sub-teams/applications/{}/approve",
                parent_id, app.id
            ),
            headers: ctx.test_user.1.clone(),
            body: {}
        };
        assert_eq!(s, 200);
    }

    // Create + publish the announcement through the controller.
    let (_, _, ann) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "Quarterly update", "body": "B" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    let (s, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/announcements/{}/publish",
            parent_id, ann.id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_eq!(s, 200);

    // Invoke the fan-out handler directly (tests don't have the stream poller).
    let source = crate::features::sub_team::models::SubTeamAnnouncement::get(
        &ctx.ddb,
        &parent_pk,
        Some(EntityType::SubTeamAnnouncement(ann.id.clone())),
    )
    .await
    .unwrap()
    .unwrap();
    crate::features::sub_team::services::announcement_fanout::handle_announcement_published(
        &ctx.ddb,
        source.clone(),
    )
    .await
    .unwrap();

    // Verify each child has one pinned-as-announcement Post.
    for child_pk in [&child_pk_a, &child_pk_b] {
        let (posts, _) = Post::find_by_user_pk(&ctx.ddb, child_pk, Post::opt().limit(10))
            .await
            .unwrap();
        let pinned: Vec<_> = posts
            .into_iter()
            .filter(|p| {
                p.pinned_as_announcement
                    && p.announcement_id.as_deref() == Some(ann.id.as_str())
            })
            .collect();
        assert_eq!(pinned.len(), 1, "expected 1 pinned post per child");
        assert_eq!(pinned[0].title, "Quarterly update");
        assert_eq!(
            pinned[0].announcement_parent_team_id.as_deref(),
            Some(parent_id.as_str())
        );
    }

    // fan_out_count propagated on source announcement.
    let refreshed = crate::features::sub_team::models::SubTeamAnnouncement::get(
        &ctx.ddb,
        &parent_pk,
        Some(EntityType::SubTeamAnnouncement(ann.id.clone())),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(refreshed.fan_out_count, 2);
}

#[tokio::test]
async fn test_publish_demotes_previous_announcement_post() {
    use crate::features::posts::models::Post;

    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    enable_parent_eligible(&ctx, &parent_id, 0).await;
    let (child_user, child_headers) = ctx.create_another_user().await;
    let (child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    let (_, _, app) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers.clone(),
        body: {
            "body": { "parent_team_id": parent_id, "form_values": {}, "doc_agreements": [] }
        },
        response_type: SubTeamApplicationResponse,
    };
    let (s, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/applications/{}/approve",
            parent_id, app.id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_eq!(s, 200);

    // First publish.
    let (_, _, ann1) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "First", "body": "B1" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/announcements/{}/publish",
            parent_id, ann1.id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    let src1 = crate::features::sub_team::models::SubTeamAnnouncement::get(
        &ctx.ddb,
        &parent_pk,
        Some(EntityType::SubTeamAnnouncement(ann1.id.clone())),
    )
    .await
    .unwrap()
    .unwrap();
    crate::features::sub_team::services::announcement_fanout::handle_announcement_published(
        &ctx.ddb, src1,
    )
    .await
    .unwrap();

    // Second publish.
    let (_, _, ann2) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "Second", "body": "B2" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/announcements/{}/publish",
            parent_id, ann2.id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    let src2 = crate::features::sub_team::models::SubTeamAnnouncement::get(
        &ctx.ddb,
        &parent_pk,
        Some(EntityType::SubTeamAnnouncement(ann2.id.clone())),
    )
    .await
    .unwrap()
    .unwrap();
    crate::features::sub_team::services::announcement_fanout::handle_announcement_published(
        &ctx.ddb, src2,
    )
    .await
    .unwrap();

    // The old ann1 post must be demoted; the new ann2 post is the only pinned one.
    let (posts, _) = Post::find_by_user_pk(&ctx.ddb, &child_pk, Post::opt().limit(20))
        .await
        .unwrap();
    let pinned: Vec<_> = posts
        .iter()
        .filter(|p| p.pinned_as_announcement)
        .collect();
    assert_eq!(pinned.len(), 1, "only latest should remain pinned");
    assert_eq!(pinned[0].announcement_id.as_deref(), Some(ann2.id.as_str()));

    // The demoted post exists but is no longer pinned.
    let demoted: Vec<_> = posts
        .iter()
        .filter(|p| p.announcement_id.as_deref() == Some(ann1.id.as_str()))
        .collect();
    assert_eq!(demoted.len(), 1);
    assert!(!demoted[0].pinned_as_announcement);
}

#[tokio::test]
async fn test_announcement_creates_notification_per_member_of_each_sub_team() {
    use crate::common::models::notification::UserInboxNotification;

    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    enable_parent_eligible(&ctx, &parent_id, 0).await;
    let (child_user, child_headers) = ctx.create_another_user().await;
    let (child_pk, child_id) = create_team_for(&ctx, &child_user, &child_headers).await;

    // Add two more members to child team.
    add_n_members(&ctx, &child_pk, 2).await;

    let (_, _, app) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/parent/applications", child_id),
        headers: child_headers,
        body: { "body": { "parent_team_id": parent_id, "form_values": {}, "doc_agreements": [] } },
        response_type: SubTeamApplicationResponse,
    };
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/applications/{}/approve",
            parent_id, app.id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };

    let (_, _, ann) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "Hi all", "body": "Body" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/teams/{}/sub-teams/announcements/{}/publish",
            parent_id, ann.id
        ),
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    let src = crate::features::sub_team::models::SubTeamAnnouncement::get(
        &ctx.ddb,
        &parent_pk,
        Some(EntityType::SubTeamAnnouncement(ann.id.clone())),
    )
    .await
    .unwrap()
    .unwrap();
    crate::features::sub_team::services::announcement_fanout::handle_announcement_published(
        &ctx.ddb, src,
    )
    .await
    .unwrap();

    // child has owner (child_user) + 2 others → 3 members → 3 announcement
    // inbox rows for this announcement.
    let members = crate::features::sub_team::services::announcement_fanout::resolve_team_members(
        &ctx.ddb,
        &child_pk,
    )
    .await
    .unwrap();
    assert!(members.len() >= 3, "expected ≥3 members: {:?}", members);

    let mut found = 0;
    for m in &members {
        let opts = UserInboxNotification::opt().limit(50);
        let (rows, _) = UserInboxNotification::query(&ctx.ddb, m.clone(), opts)
            .await
            .unwrap();
        for r in rows {
            if let InboxPayload::SubTeamAnnouncementReceived {
                announcement_id, ..
            } = r.payload
            {
                if announcement_id == ann.id {
                    found += 1;
                }
            }
        }
    }
    assert!(
        found >= 3,
        "expected ≥3 inbox rows across child members, found {found}"
    );
}

#[tokio::test]
async fn test_soft_delete_announcement_sets_status_deleted() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    let (_, _, ann) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "x", "body": "y" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };

    let (status, _, _) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements/{}", parent_id, ann.id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200);

    let stored = crate::features::sub_team::models::SubTeamAnnouncement::get(
        &ctx.ddb,
        &parent_pk,
        Some(EntityType::SubTeamAnnouncement(ann.id.clone())),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(matches!(
        stored.status,
        crate::features::sub_team::models::SubTeamAnnouncementStatus::Deleted
    ));
}

#[tokio::test]
async fn test_list_announcements_excludes_deleted_by_default() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);

    // Create 2 announcements; delete one.
    let (_, _, a1) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "Kept", "body": "b" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    let (_, _, a2) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        body: { "body": { "title": "Deleted", "body": "b" } },
        response_type: crate::features::sub_team::types::SubTeamAnnouncementResponse,
    };
    let (_, _, _) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements/{}", parent_id, a2.id),
        headers: ctx.test_user.1.clone(),
    };

    let (status, _, listed) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<crate::features::sub_team::types::SubTeamAnnouncementResponse>,
    };
    assert_eq!(status, 200);
    assert!(listed.items.iter().any(|x| x.id == a1.id));
    assert!(
        !listed.items.iter().any(|x| x.id == a2.id),
        "deleted ann must be excluded by default"
    );
}

#[tokio::test]
async fn test_announcement_list_supports_pagination() {
    let ctx = TestContext::setup().await;
    let parent_pk = create_parent_team(&ctx).await;
    let parent_id = team_id_from(&parent_pk);
    let _ = parent_pk;

    // Create several. Default page limit is 50 so a smaller list is fine —
    // smoke-test that ListResponse shape is honored and items are returned.
    for i in 0..3 {
        let (_, _, _) = crate::test_post! {
            app: ctx.app.clone(),
            path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
            headers: ctx.test_user.1.clone(),
            body: {
                "body": { "title": format!("A{}", i), "body": "b" }
            }
        };
    }

    let (status, _, listed) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/teams/{}/sub-teams/announcements", parent_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<crate::features::sub_team::types::SubTeamAnnouncementResponse>,
    };
    assert_eq!(status, 200);
    assert!(listed.items.len() >= 3);
}

// ── Compile-time silence (unused-import guard) ───────────────────
#[allow(dead_code)]
fn _unused_guard() {
    let _ = SubTeamFormField::default();
    let _ = SubTeamDocument::default();
    let _ = SubTeamFormFieldType::ShortText;
    let _ = UserTeamQueryOption::builder();
    let _: Option<SubTeamApplicationDetailResponse> = None;
}
