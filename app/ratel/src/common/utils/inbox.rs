#[cfg(feature = "server")]
use crate::common::*;
#[cfg(feature = "server")]
use crate::common::models::notification::{InboxDedupMarker, UserInboxNotification};

/// Write an inbox row. Non-fatal: logs `error!` on DynamoDB failure and returns Ok.
#[cfg(feature = "server")]
pub async fn create_inbox_row(
    recipient_pk: Partition,
    payload: InboxPayload,
) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let row = UserInboxNotification::new(recipient_pk.clone(), payload);
    if let Err(e) = row.create(cli).await {
        crate::error!(
            "create_inbox_row: failed for user={}: {e}",
            recipient_pk
        );
    }
    Ok(())
}

/// Idempotent inbox row creator. Uses `InboxDedupMarker` as a 7-day lock
/// keyed on `(recipient, kind, source_id)`. If the marker already exists,
/// the inbox row is skipped.
#[cfg(feature = "server")]
pub async fn create_inbox_row_once(
    recipient_pk: Partition,
    payload: InboxPayload,
    source_id: &str,
) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let kind = payload.kind();
    let marker = InboxDedupMarker::new(recipient_pk.clone(), kind, source_id);

    match InboxDedupMarker::get(cli, &marker.pk, Some(marker.sk.clone())).await {
        Ok(Some(_)) => {
            tracing::debug!(
                "create_inbox_row_once: skipped duplicate for user={} kind={:?} source={}",
                recipient_pk, kind, source_id
            );
            return Ok(());
        }
        Ok(None) => {}
        Err(e) => {
            crate::error!(
                "create_inbox_row_once: dedup lookup failed, proceeding anyway: {e}"
            );
        }
    }

    let row = UserInboxNotification::new(recipient_pk.clone(), payload);
    if let Err(e) = row.create(cli).await {
        crate::error!(
            "create_inbox_row_once: row create failed for user={}: {e}",
            recipient_pk
        );
        return Ok(());
    }

    if let Err(e) = marker.create(cli).await {
        crate::error!(
            "create_inbox_row_once: marker create failed (row created, dedup broken): {e}"
        );
    }
    Ok(())
}
