use crate::features::auth::{context::*, *};
use crate::features::membership::controllers::get_membership_handler;
use crate::features::membership::models::UserMembershipResponse;

pub fn use_user_membership() -> Option<UserMembershipResponse> {
    let ctx = use_context::<Context>();
    let mut user_ctx = ctx.user_context;

    let _loader = use_resource(move || async move {
        let has_user = user_ctx.read().user.is_some();
        let has_membership = user_ctx.read().membership.is_some();

        if has_user && !has_membership {
            if let Ok(resp) = get_membership_handler().await {
                user_ctx.write().membership = Some(resp);
            }
        }
    });

    let membership = user_ctx.read().membership.clone();
    membership
}
