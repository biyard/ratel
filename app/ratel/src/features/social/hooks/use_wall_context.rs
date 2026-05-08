use crate::*;
use features::social::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseWallContext {
    pub data: Loader<Wall>,
}

impl UseWallContext {
    pub fn is_user(&self) -> bool {
        matches!(self.data(), Wall::User { .. })
    }

    pub fn is_team(&self) -> bool {
        matches!(self.data(), Wall::Team { .. })
    }
}

#[track_caller]
pub fn use_wall_context() -> UseWallContext {
    use_context()
}

#[track_caller]
pub fn use_wall_context_provider(
    username: ReadSignal<String>,
) -> Result<UseWallContext, Loading> {
    let data = use_loader(move || async move { get_wall_by_username(username()).await })?;

    let ctx = use_context_provider(move || UseWallContext { data });

    Ok(ctx)
}
