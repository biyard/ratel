use dioxus::fullstack::{Loader, Loading};

use crate::{
    controllers::{SpaceResponse, get_user_role},
    hooks::*,
    *,
};

#[derive(Clone, Copy)]
pub struct SpaceContextProvider {
    pub role: Loader<SpaceUserRole>,
    pub space: Loader<SpaceResponse>,
    pub current_role: Signal<SpaceUserRole>,
}

impl SpaceContextProvider {
    pub fn init(space_id: &SpacePartition) -> common::Result<Self, Loading> {
        let v = space_id.clone();
        let role = use_loader(move || {
            let space_id = v.clone();
            async move { get_user_role(space_id.clone()).await }
        })?;
        let space = use_space_query(space_id)?;
        let current_role = use_signal(|| role());

        let srv = Self {
            role,
            space,
            current_role,
        };
        use_context_provider(move || srv);

        Ok(srv)
    }

    pub fn toggle_role(&mut self) -> Result<()> {
        let current_role = (self.current_role)();
        let role = (self.role)();

        match (current_role, role) {
            (SpaceUserRole::Viewer, SpaceUserRole::Creator) => {
                self.current_role.set(SpaceUserRole::Creator);
                Ok(())
            }
            (SpaceUserRole::Creator, SpaceUserRole::Creator) => {
                self.current_role.set(SpaceUserRole::Viewer);
                Ok(())
            }
            _ => Err(Error::UnauthorizedAccess),
        }
    }
}

pub fn use_space_context() -> SpaceContextProvider {
    use_context()
}
