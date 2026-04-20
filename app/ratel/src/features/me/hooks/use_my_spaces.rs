use crate::features::spaces::space_common::controllers::*;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseMySpaces {
    pub my_spaces: Loader<ListResponse<HotSpaceResponse>>,
}

pub fn use_my_spaces() -> Result<UseMySpaces, Loading> {
    let user_ctx = crate::features::auth::hooks::use_user_context();

    let ctx: Option<UseMySpaces> = try_use_context();

    if let Some(ctx) = ctx {
        return Ok(ctx);
    }
    let my_spaces = use_loader(move || async move {
        if user_ctx().user.is_some() {
            list_my_home_spaces_handler(None).await
        } else {
            Ok(Default::default())
        }
    })?;

    Ok(use_context_provider(move || UseMySpaces { my_spaces }))
}
