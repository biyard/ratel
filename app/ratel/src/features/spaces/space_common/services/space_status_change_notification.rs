use crate::common::models::space::SpaceStatusChangeEvent;
use crate::common::*;

/// Handle a space status transition by resolving the audience and creating
/// Notification rows to fan out via the existing SES pipeline.
pub async fn handle_space_status_change(event: SpaceStatusChangeEvent) -> Result<()> {
    tracing::info!(
        space_pk = %event.space_pk,
        old_status = ?event.old_status,
        new_status = ?event.new_status,
        "handle_space_status_change: received event",
    );

    // Implementation filled in by subsequent tasks.
    Ok(())
}
