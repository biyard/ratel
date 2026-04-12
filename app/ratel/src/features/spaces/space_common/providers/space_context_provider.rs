use dioxus::fullstack::{Loader, Loading};

use crate::{
    features::spaces::space_common::{
        controllers::{get_user_role, SpaceResponse},
        hooks::*,
        *,
    },
    spaces::controllers::panel_requirements::PanelRequirementStatus,
};

#[derive(Clone, Copy, DioxusController)]
pub struct SpaceContextProvider {
    pub role: Loader<SpaceUserRole>,
    pub space: Loader<SpaceResponse>,
    pub current_role: Memo<SpaceUserRole>,
    pub panel_requirements: Loader<Vec<PanelRequirementStatus>>,
}

impl SpaceContextProvider {
    pub fn init(space_id: ReadSignal<SpacePartition>) -> crate::common::Result<Self, Loading> {
        let role = use_loader(move || async move { get_user_role(space_id()).await })?;
        let space = use_loader(move || async move { get_space(space_id()).await })?;
        let mut panel_requirements = use_loader(move || async move {
            crate::features::spaces::controllers::panel_requirements::get_panel_requirements(
                space_id(),
            )
            .await
        })?;

        let mut current_role = use_memo(move || {
            let space = space();
            let role = role();
            match (role, space.status) {
                (SpaceUserRole::Creator, Some(SpaceStatus::Ongoing)) => SpaceUserRole::Participant,
                _ => role,
            }
        });

        let srv = Self {
            role,
            space,
            current_role,
            panel_requirements,
        };
        debug!("Initialized SpaceContextProvider");
        use_context_provider(move || srv);

        Ok(srv)
    }

    pub fn toggle_role(&mut self) -> Result<()> {
        let current_role = (self.current_role)();
        let role = (self.role)();

        match (current_role, role) {
            (SpaceUserRole::Creator, SpaceUserRole::Creator) => {
                self.current_role.set(SpaceUserRole::Participant);
                Ok(())
            }
            (_, SpaceUserRole::Creator) => {
                self.current_role.set(SpaceUserRole::Creator);
                Ok(())
            }

            _ => Err(Error::UnauthorizedAccess),
        }
    }
}

pub fn use_space_context() -> SpaceContextProvider {
    use_context()
}
