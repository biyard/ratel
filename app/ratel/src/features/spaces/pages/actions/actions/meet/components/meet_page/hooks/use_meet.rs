use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};
use crate::features::spaces::pages::actions::types::SpaceActionStatus;

#[derive(Clone, Copy)]
pub struct UseMeet {
    pub space_id: ReadSignal<SpacePartition>,
    pub meet_id: ReadSignal<SpaceMeetEntityType>,
    pub meet: Loader<MeetResponse>,
    pub update_mode: Action<(MeetMode,), ()>,
    pub update_start_time: Action<(i64,), ()>,
    pub update_duration: Action<(i32,), ()>,
    pub publish: Action<(), ()>,
}

#[track_caller]
pub fn use_meet(
    space_id: ReadSignal<SpacePartition>,
    meet_id: ReadSignal<SpaceMeetEntityType>,
) -> std::result::Result<UseMeet, RenderError> {
    if let Some(ctx) = try_use_context::<UseMeet>() {
        return Ok(ctx);
    }

    let mut meet =
        use_loader(move || async move { get_meet(space_id(), meet_id()).await })?;

    let update_mode = use_action(move |mode: MeetMode| async move {
        update_meet(space_id(), meet_id(), UpdateMeetRequest::Mode { mode }).await?;
        meet.restart();
        Ok::<(), crate::common::Error>(())
    });

    let update_start_time = use_action(move |start_time: i64| async move {
        update_meet(
            space_id(),
            meet_id(),
            UpdateMeetRequest::StartTime { start_time },
        )
        .await?;
        meet.restart();
        Ok::<(), crate::common::Error>(())
    });

    let update_duration = use_action(move |duration_min: i32| async move {
        update_meet(
            space_id(),
            meet_id(),
            UpdateMeetRequest::DurationMin { duration_min },
        )
        .await?;
        meet.restart();
        Ok::<(), crate::common::Error>(())
    });

    let publish = use_action(move || async move {
        let current = meet();
        let mode = current.mode.clone();
        if mode == MeetMode::Instant {
            let now = crate::common::utils::time::get_now_timestamp_millis();
            update_meet(
                space_id(),
                meet_id(),
                UpdateMeetRequest::StartTime { start_time: now },
            )
            .await?;
        }
        let action_id = meet_id().to_string();
        update_space_action(
            space_id(),
            action_id,
            UpdateSpaceActionRequest::Status {
                status: SpaceActionStatus::Ongoing,
            },
        )
        .await?;
        meet.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseMeet {
        space_id,
        meet_id,
        meet,
        update_mode,
        update_start_time,
        update_duration,
        publish,
    }))
}
