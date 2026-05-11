//! Controller for the child-side "apply to be a sub-team" form.
//!
//! Route: `/:username/sub-teams/apply` — `:username` is the **parent**
//! team being applied to. The page provides that team's id via
//! `use_context_provider`, which we read as `parent_team_id` so the
//! apply-context loader fires immediately on mount and renders the
//! form fields the parent admin configured.
//!
//! Phase-2 additions:
//!   - `my_teams` loader: lists every team the current user belongs
//!     to so the apply page can render a real "Pick your team"
//!     dropdown (mockup `team-dropdown`) instead of a placeholder.
//!   - `applicant_team_id`: signal driven by the picker. Defaults to
//!     the first admin team the user owns. The submit handler routes
//!     through this team's pk (path param) — server enforces admin
//!     role.
//!   - `handle_save_draft`: writes the current form-values + doc
//!     agreements to a backend `SubTeamApplicationDraft` row keyed by
//!     `(applicant_team, parent_team)`, so the applicant can leave
//!     and resume.

use crate::common::contexts::TeamItem;
use crate::features::social::controllers::list_admin_teams_handler;
use crate::features::sub_team::controllers::{
    delete_sub_team_application_draft_handler, get_sub_team_apply_context_handler,
    get_sub_team_application_draft_handler, list_child_applications_handler,
    save_sub_team_application_draft_handler, submit_application_handler,
};
use crate::features::sub_team::models::SubTeamApplicationStatus;
use crate::features::sub_team::types::{
    ApplyContextResponse, DocAgreementInput, SaveApplicationDraftRequest,
    SubmitApplicationRequest,
};
use crate::*;
use std::collections::HashMap;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamApply {
    /// Applicant (child) team — driven by the team-picker dropdown.
    pub applicant_team_id: Signal<TeamPartition>,
    /// Read-only mirror used by callers that don't care which team
    /// the user picked (e.g. submit path).
    pub team_id: ReadSignal<TeamPartition>,
    /// Parent team being applied to — seeded from the route's
    /// `:username` context.
    pub parent_team_id: Signal<String>,
    pub apply_context: Loader<ApplyContextResponse>,
    /// Teams the current user belongs to (membership rows).
    pub my_teams: Loader<ListResponse<TeamItem>>,
    /// Form values keyed by `field_id`. Populated/edited by the UI.
    pub form_values: Signal<HashMap<String, serde_json::Value>>,
    /// Doc ids the user has explicitly agreed to in the current session.
    pub agreed_doc_ids: Signal<HashMap<String, String>>,

    /// Writes the current `(form_values, agreed_doc_ids)` to a draft
    /// row keyed by `(applicant_team_id, parent_team_id)`.
    pub handle_save_draft: Action<(), ()>,
}

impl UseSubTeamApply {
    /// Submits the application as the picked applicant team. On success
    /// the draft row is cleared and the new application's id is returned
    /// so the caller can navigate to the status page.
    pub async fn submit(self) -> crate::common::Result<String> {
        let parent = (self.parent_team_id)();
        let values = self.form_values.read().clone();
        let agreements: Vec<DocAgreementInput> = self
            .agreed_doc_ids
            .read()
            .iter()
            .map(|(doc_id, body_hash)| DocAgreementInput {
                doc_id: doc_id.clone(),
                body_hash: body_hash.clone(),
            })
            .collect();
        let req = SubmitApplicationRequest {
            parent_team_id: parent.clone(),
            form_values: values,
            doc_agreements: agreements,
        };
        let applicant: TeamPartition = (self.team_id)();
        let resp = submit_application_handler(applicant.clone(), req).await?;
        // Clear the draft after a successful submit — the row's purpose
        // was to hold the unfinished state. Failure is non-fatal: we
        // still return Ok so the caller navigates to status.
        let _ = delete_sub_team_application_draft_handler(applicant, parent).await;
        Ok(resp.id)
    }
}

#[track_caller]
pub fn use_sub_team_apply() -> std::result::Result<UseSubTeamApply, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamApply>() {
        return Ok(ctx);
    }

    // Route-provided context is the PARENT team. The applicant team
    // is determined by the team-picker; we seed it with the parent's
    // id so the structure compiles before the picker chooses one.
    let parent_id: TeamPartition = use_context();
    let parent_id_str = parent_id.0.clone();
    let mut applicant_team_id: Signal<TeamPartition> =
        use_signal(|| parent_id.clone());
    let team_id_signal: ReadSignal<TeamPartition> = applicant_team_id.into();
    let parent_team_id: Signal<String> = use_signal(|| parent_id_str);

    let apply_context = use_loader(move || {
        let parent = parent_team_id();
        async move {
            if parent.is_empty() {
                return Ok(ApplyContextResponse::default());
            }
            get_sub_team_apply_context_handler(TeamPartition(parent)).await
        }
    })?;

    let my_teams = use_loader(move || async move {
        list_admin_teams_handler(None).await
    })?;

    // Auto-pick the first team for the user — the API already
    // filters to admin/owner roles server-side, so any item is
    // submit-eligible. Runs in a `use_effect` (not inline) so it
    // only fires once after the loader resolves and avoids
    // re-render loops.
    {
        let parent_for_effect = parent_id.clone();
        use_effect(move || {
            let teams = my_teams();
            if let Some(t) = teams.items.first() {
                if let Ok(pk) = t.pk.parse::<TeamPartition>() {
                    let current = applicant_team_id();
                    // Only seed if applicant hasn't been picked yet —
                    // i.e. still empty or still equal to the parent
                    // (the structural placeholder).
                    if current.0.is_empty() || current == parent_for_effect {
                        applicant_team_id.set(pk);
                    }
                }
            }
        });
    }

    let form_values: Signal<HashMap<String, serde_json::Value>> = use_signal(HashMap::new);
    let agreed_doc_ids: Signal<HashMap<String, String>> = use_signal(HashMap::new);

    // Two-step hydration (each one-shot via its own guard so the
    // effect can't re-trigger itself through implicit subscriptions):
    //   1. Existing in-flight application for this (applicant, parent)
    //      — wins when the applicant is "Edit and resubmit"-ing a
    //      Returned application or just revisiting a Pending one.
    //   2. Draft row — used only when no in-flight application exists
    //      (drafts are deleted on successful submit, so seeing both
    //      together is rare; if it happens, draft is older noise and
    //      the application is the source of truth).
    let mut hydrated: Signal<bool> = use_signal(|| false);
    let apply_context_for_hydrate = apply_context;
    {
        let mut form_values_w = form_values;
        let mut agreed = agreed_doc_ids;
        use_effect(move || {
            if hydrated() {
                return;
            }
            let applicant = applicant_team_id();
            let parent = parent_team_id();
            if applicant.0.is_empty() || parent.is_empty() || applicant.0 == parent {
                return;
            }
            hydrated.set(true);
            let parent_cloned = parent.clone();
            // Snapshot the required docs so the hydration spawn can
            // pre-fill `agreed_doc_ids` from the body hashes the
            // applicant agreed to at first submission. The user
            // doesn't need to re-agree on resubmit since the doc
            // body hasn't changed (server enforces the hash check).
            let required_docs: Vec<(String, String)> = apply_context_for_hydrate
                .read()
                .required_docs
                .iter()
                .filter(|d| d.required)
                .map(|d| (d.id.clone(), d.body_hash.clone()))
                .collect();
            spawn(async move {
                // Step 1 — try in-flight application first.
                let in_flight = list_child_applications_handler(applicant.clone(), None)
                    .await
                    .ok()
                    .and_then(|page| {
                        page.items.into_iter().find(|a| {
                            a.parent_team_id == parent_cloned
                                && matches!(
                                    a.status,
                                    SubTeamApplicationStatus::Pending
                                        | SubTeamApplicationStatus::Returned,
                                )
                        })
                    });
                if let Some(app) = in_flight {
                    form_values_w.set(app.form_values);
                    // Pre-agree all currently-required docs (the user
                    // already agreed at first submission). Pulled from
                    // apply_context so we use the latest hashes.
                    agreed.set(required_docs.into_iter().collect());
                    return;
                }
                // Step 2 — fall back to draft hydration.
                if let Ok(Some(draft)) =
                    get_sub_team_application_draft_handler(applicant, parent_cloned).await
                {
                    form_values_w.set(draft.form_values);
                    agreed.set(
                        draft
                            .doc_agreements
                            .into_iter()
                            .map(|a| (a.doc_id, a.body_hash))
                            .collect(),
                    );
                }
            });
        });
    }

    // Submit lives as `UseSubTeamApply::submit` (async fn) so the page
    // can `await` it and navigate to the status page on Ok. See impl
    // block above. `handle_save_draft` stays an Action because it's
    // fired-and-forgot from the debounce effect.
    let handle_save_draft = use_action(move || async move {
        let parent = parent_team_id();
        let applicant = applicant_team_id();
        if parent.is_empty() || applicant.0.is_empty() || applicant.0 == parent {
            // Nothing to write — the applicant hasn't been chosen yet
            // or still mirrors the parent (initial placeholder state).
            return Ok::<(), crate::common::Error>(());
        }
        let values = form_values.read().clone();
        let agreements: Vec<DocAgreementInput> = agreed_doc_ids
            .read()
            .iter()
            .map(|(doc_id, body_hash)| DocAgreementInput {
                doc_id: doc_id.clone(),
                body_hash: body_hash.clone(),
            })
            .collect();
        save_sub_team_application_draft_handler(
            applicant,
            SaveApplicationDraftRequest {
                parent_team_id: parent,
                form_values: values,
                doc_agreements: agreements,
            },
        )
        .await?;
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamApply {
        applicant_team_id,
        team_id: team_id_signal,
        parent_team_id,
        apply_context,
        my_teams,
        form_values,
        agreed_doc_ids,
        handle_save_draft,
    }))
}
