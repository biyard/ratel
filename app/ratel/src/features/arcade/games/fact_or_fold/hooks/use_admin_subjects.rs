//! `UseFactFoldAdminSubjects` — drive the admin subject list page.
//!
//! Loader-based (one page of ≤50 rows is enough for v1; pagination
//! lands when the lifetime queue actually exceeds it). Mutations go
//! through async fn methods on the controller per
//! `conventions/hooks-and-actions.md` — components await them and
//! pick their own UX (toast / nav / row removal).
//!
//! Loader-resolution convention: `subjects()` returns
//! `Result<Loader<...>, Loading>` so consumers resolve at use time.

use crate::features::arcade::games::fact_or_fold::{
    CreateSubjectRequest, SubjectResponse, SubjectStatus, PublishSubjectRequest,
    create_subject_handler, delete_subject_handler, get_settings_handler,
    list_subjects_handler, publish_subject_handler,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldAdminSubjects {
    /// Active filter — drives the loader's status query param.
    pub status_filter: Signal<Option<SubjectStatus>>,
    /// Bump to force the subjects loader to re-fetch.
    pub subjects_refresh: Signal<u64>,
    /// Queue alarm threshold (admin-tunable). Read once at load time
    /// so filter / page transitions don't re-fetch.
    pub queue_alert_threshold_days: Signal<i32>,
}

impl UseFactFoldAdminSubjects {
    pub fn subjects(&self) -> std::result::Result<Loader<Vec<SubjectResponse>>, Loading> {
        let status_filter = self.status_filter;
        let refresh = self.subjects_refresh;
        use_loader(move || {
            let _ = refresh();
            let status = status_filter();
            async move {
                let resp = list_subjects_handler(None, status).await?;
                Ok::<Vec<SubjectResponse>, crate::common::Error>(resp.items)
            }
        })
    }

    /// Create a draft / scheduled subject. The freshly inserted row
    /// doesn't always show up at the top of the next list page since
    /// we sort by `created_at desc` server-side, so we bump the
    /// refresh signal to make sure the new row shows up immediately.
    pub async fn create(
        &mut self,
        req: CreateSubjectRequest,
    ) -> crate::common::Result<SubjectResponse> {
        let res = create_subject_handler(req).await?;
        self.subjects_refresh.with_mut(|n| *n += 1);
        Ok(res)
    }

    pub async fn publish_now(
        &mut self,
        subject_id: crate::FactFoldSubjectEntityType,
    ) -> crate::common::Result<SubjectResponse> {
        let res = publish_subject_handler(
            subject_id,
            PublishSubjectRequest {
                scheduled_at: None,
                expires_at: None,
            },
        )
        .await?;
        self.subjects_refresh.with_mut(|n| *n += 1);
        Ok(res)
    }

    pub async fn delete(
        &mut self,
        subject_id: crate::FactFoldSubjectEntityType,
    ) -> crate::common::Result<SubjectResponse> {
        let res = delete_subject_handler(subject_id).await?;
        self.subjects_refresh.with_mut(|n| *n += 1);
        Ok(res)
    }
}

pub fn use_fact_fold_admin_subjects_provider()
-> std::result::Result<UseFactFoldAdminSubjects, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldAdminSubjects>() {
        return Ok(ctx);
    }

    let status_filter: Signal<Option<SubjectStatus>> = use_signal(|| None);
    let queue_alert_threshold_days = use_signal(|| 5);
    let subjects_refresh = use_signal(|| 0u64);

    // Pull the alert threshold once. Failure is non-fatal — we keep
    // the default in the signal — so we ignore the error to keep the
    // page renderable even if the singleton row is missing.
    let mut alert_signal = queue_alert_threshold_days;
    spawn(async move {
        if let Ok(s) = get_settings_handler().await {
            alert_signal.set(s.queue_low_alert_days);
        }
    });

    Ok(use_context_provider(|| UseFactFoldAdminSubjects {
        status_filter,
        subjects_refresh,
        queue_alert_threshold_days,
    }))
}

#[track_caller]
pub fn use_fact_fold_admin_subjects() -> UseFactFoldAdminSubjects {
    use_context::<UseFactFoldAdminSubjects>()
}
