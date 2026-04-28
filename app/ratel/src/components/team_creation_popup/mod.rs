use crate::features::posts::types::TeamGroupPermissions;
use crate::features::social::controllers::{
    create_team_handler, get_user_teams_handler, CreateTeamRequest,
};
use crate::*;

#[component]
pub fn TeamCreationPopup() -> Element {
    let mut popup = use_popup();
    let mut team_ctx = use_team_context();
    let nav = use_navigator();
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut submitting = use_signal(|| false);
    rsx! {
        crate::common::TeamCreationForm {
            submitting: submitting(),
            error_message: error_msg.read().clone(),
            on_cancel: move |_| {
                popup.close();
            },
            on_submit: move |payload| async move {
                submitting.set(true);
                error_msg.set(None);

                match team_ctx.create_team(payload).await {
                    Ok(response) => {
                        team_ctx.select_team(&response.pk);
                        popup.close();
                        nav.push(Route::TeamHome {
                            username: response.username,
                        });
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("{}", e)));
                    }
                }
                submitting.set(false);
            },
        }
    }
}
