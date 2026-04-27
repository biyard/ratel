use crate::common::hooks::{InfiniteQuery, use_infinite_query};
use crate::features::spaces::pages::apps::apps::general::controllers::*;
use crate::features::spaces::space_common::controllers::{UpdateSpaceRequest, update_space};
use crate::features::spaces::space_common::hooks::use_space;
use crate::*;

const INVITATION_PAGE_SIZE: i32 = 20;

/// Controller hook for the Space General Settings page (arena view).
///
/// Bundles:
/// - Auxiliary loaders (`admins`, `invitations`) that only this page
///   needs.
/// - Every mutation action a setting card can fire.
///
/// The shared `SpaceResponse` loader is accessed separately via
/// `use_space()` — each action captures it so success handlers can run
/// `space.with_mut(...)` for optimistic UI without a refetch. List
/// loaders are restarted via `.restart()` / `.refresh()` instead.
///
/// Error handling: every action `match`es on the server result and
/// fires a toast for either branch, then always returns `Ok(())`. That
/// keeps `Action::pending()` meaningful (reflects in-flight requests
/// only) and means components don't need to read `.error()` — the toast
/// is the user-visible signal.
#[derive(Clone, Copy)]
pub struct UseSpaceGeneralSettings {
    pub space_id: ReadSignal<SpacePartition>,
    pub admins: Loader<Vec<SpaceAdminListItem>>,
    pub invitations:
        InfiniteQuery<String, SpaceInvitationListItem, ListResponse<SpaceInvitationListItem>>,

    pub update_logo: Action<(String,), ()>,
    pub update_start_time: Action<(i64,), ()>,
    pub update_visibility: Action<(SpaceVisibility,), ()>,
    pub update_anonymous: Action<(bool,), ()>,
    pub update_join_anytime: Action<(bool,), ()>,
    pub send_invitations: Action<(Vec<String>,), ()>,
    pub delete_invitation: Action<(UserPartition,), ()>,
    pub add_admins: Action<(Vec<String>,), ()>,
    pub remove_admin: Action<(UserPartition,), ()>,
    pub delete_space_action: Action<(), ()>,
}

#[track_caller]
pub fn use_space_general_settings(
    space_id: ReadSignal<SpacePartition>,
) -> std::result::Result<UseSpaceGeneralSettings, RenderError> {
    if let Some(ctx) = try_use_context::<UseSpaceGeneralSettings>() {
        return Ok(ctx);
    }

    let tr: crate::features::spaces::pages::apps::apps::general::GeneralTranslate = use_translate();
    let mut toast = use_toast();
    let nav = use_navigator();

    let mut space = use_space();

    let mut admins = use_loader(move || async move {
        let space_id = space_id();
        list_space_admins(space_id).await
    })?;

    let mut invitations = use_infinite_query(move |bookmark| {
        list_space_invitations(space_id(), bookmark, Some(INVITATION_PAGE_SIZE))
    })?;

    // ─── Mutations ───────────────────────────────────────

    let update_logo = use_action(move |logo: String| async move {
        match update_space(
            space_id(),
            UpdateSpaceRequest::Logo { logo: logo.clone() },
        )
        .await
        {
            Ok(_) => {
                space.with_mut(|s| s.logo = logo);
                toast.info(tr.logo_updated_successfully);
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let update_start_time = use_action(move |started_at: i64| async move {
        match update_space(
            space_id(),
            UpdateSpaceRequest::StartTime {
                started_at: Some(started_at),
            },
        )
        .await
        {
            Ok(_) => {
                space.with_mut(|s| s.started_at = Some(started_at));
                toast.info(tr.start_time_updated_successfully);
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let update_visibility = use_action(move |visibility: SpaceVisibility| async move {
        match update_space(
            space_id(),
            UpdateSpaceRequest::Visibility {
                visibility: visibility.clone(),
            },
        )
        .await
        {
            Ok(_) => {
                space.with_mut(|s| s.visibility = visibility);
                toast.info(tr.visibility_updated_successfully);
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let update_anonymous = use_action(move |anonymous_participation: bool| async move {
        match update_space(
            space_id(),
            UpdateSpaceRequest::Anonymous {
                anonymous_participation,
            },
        )
        .await
        {
            Ok(_) => {
                space.with_mut(|s| s.anonymous_participation = anonymous_participation);
                toast.info(tr.anonymous_updated_successfully);
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let update_join_anytime = use_action(move |join_anytime: bool| async move {
        match update_space(
            space_id(),
            UpdateSpaceRequest::JoinAnytime { join_anytime },
        )
        .await
        {
            Ok(_) => {
                space.with_mut(|s| s.join_anytime = join_anytime);
                toast.info(tr.join_anytime_updated_successfully);
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let send_invitations = use_action(move |emails: Vec<String>| async move {
        if emails.is_empty() {
            return Ok::<(), crate::common::Error>(());
        }
        match invite_space_participants(space_id(), InviteSpaceParticipantsRequest { emails })
            .await
        {
            Ok(_) => {
                invitations.refresh();
                toast.info(tr.participants_invited_successfully);
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let delete_invitation = use_action(move |user_id: UserPartition| async move {
        match delete_space_invitation(space_id(), user_id).await {
            Ok(_) => {
                invitations.refresh();
                toast.info(tr.invitation_deleted_successfully);
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let add_admins = use_action(move |targets: Vec<String>| async move {
        if targets.is_empty() {
            return Ok::<(), crate::common::Error>(());
        }
        let mut last_err: Option<crate::common::Error> = None;
        for target in targets {
            if let Err(err) = add_space_admin(space_id(), AddSpaceAdminRequest { target }).await {
                last_err = Some(err);
                break;
            }
        }
        match last_err {
            Some(err) => {
                toast.error(err);
            }
            None => {
                admins.restart();
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let remove_admin = use_action(move |user_id: UserPartition| async move {
        match remove_space_admin(space_id(), user_id).await {
            Ok(_) => {
                admins.restart();
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let delete_space_action = use_action(move || async move {
        match delete_space(space_id()).await {
            Ok(_) => {
                toast.info(tr.space_deleted_successfully);
                nav.push(Route::Index {});
            }
            Err(err) => {
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSpaceGeneralSettings {
        space_id,
        admins,
        invitations,
        update_logo,
        update_start_time,
        update_visibility,
        update_anonymous,
        update_join_anytime,
        send_invitations,
        delete_invitation,
        add_admins,
        remove_admin,
        delete_space_action,
    }))
}
