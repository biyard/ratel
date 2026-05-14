//! `UseFactFoldAdminHeadlines` — drive the admin headline list page.
//!
//! Loader-based (one page of ≤50 rows is enough for v1; pagination
//! lands when the lifetime queue actually exceeds it). Mutations go
//! through async fn methods on the controller per
//! `conventions/hooks-and-actions.md` — components await them and
//! pick their own UX (toast / nav / row removal).

use crate::features::fact_or_fold::{
    CreateHeadlineRequest, HeadlineResponse, HeadlineStatus, PublishHeadlineRequest,
    create_headline_handler, delete_headline_handler, get_settings_handler,
    list_headlines_handler, publish_headline_handler,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldAdminHeadlines {
    /// Active filter — drives the loader's status query param.
    pub status_filter: Signal<Option<HeadlineStatus>>,
    /// Headlines for the current filter (server-paginated; we render
    /// the first page and let the user re-filter to slice).
    pub headlines: Loader<Vec<HeadlineResponse>>,
    /// Queue alarm threshold (admin-tunable). Read once at load time
    /// so filter / page transitions don't re-fetch.
    pub queue_alert_threshold_days: Signal<i32>,
}

impl UseFactFoldAdminHeadlines {
    /// Create a draft / scheduled headline. The freshly inserted row
    /// doesn't always show up at the top of the next list page since
    /// we sort by `created_at desc` server-side, so we restart the
    /// loader to make sure the new row shows up immediately.
    pub async fn create(
        &mut self,
        req: CreateHeadlineRequest,
    ) -> crate::common::Result<HeadlineResponse> {
        let res = create_headline_handler(req).await?;
        self.headlines.restart();
        Ok(res)
    }

    pub async fn publish_now(
        &mut self,
        headline_id: crate::FactFoldHeadlineEntityType,
    ) -> crate::common::Result<HeadlineResponse> {
        let res = publish_headline_handler(headline_id, PublishHeadlineRequest { scheduled_at: None })
            .await?;
        self.headlines.restart();
        Ok(res)
    }

    pub async fn delete(
        &mut self,
        headline_id: crate::FactFoldHeadlineEntityType,
    ) -> crate::common::Result<HeadlineResponse> {
        let res = delete_headline_handler(headline_id).await?;
        self.headlines.restart();
        Ok(res)
    }
}

pub fn use_fact_fold_admin_headlines_provider()
-> std::result::Result<UseFactFoldAdminHeadlines, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldAdminHeadlines>() {
        return Ok(ctx);
    }

    let status_filter: Signal<Option<HeadlineStatus>> = use_signal(|| None);
    let queue_alert_threshold_days = use_signal(|| 5);

    let headlines = use_loader(move || {
        let status = status_filter();
        async move {
            let resp = list_headlines_handler(None, status).await?;
            Ok::<Vec<HeadlineResponse>, crate::common::Error>(resp.items)
        }
    })?;

    // Pull the alert threshold once. Failure is non-fatal — we keep
    // the default in the signal — so we ignore the error to keep the
    // page renderable even if the singleton row is missing.
    let mut alert_signal = queue_alert_threshold_days;
    spawn(async move {
        if let Ok(s) = get_settings_handler().await {
            alert_signal.set(s.queue_low_alert_days);
        }
    });

    Ok(use_context_provider(|| UseFactFoldAdminHeadlines {
        status_filter,
        headlines,
        queue_alert_threshold_days,
    }))
}

#[track_caller]
pub fn use_fact_fold_admin_headlines() -> UseFactFoldAdminHeadlines {
    use_context::<UseFactFoldAdminHeadlines>()
}
