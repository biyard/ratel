use super::*;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, DynamoEnum, Eq, PartialEq, Translate,
)]
pub enum SpaceAppType {
    #[default]
    #[translate(en = "General", ko = "스페이스 설정")]
    General,
    #[translate(en = "File", ko = "파일")]
    File,
    #[translate(en = "Panels", ko = "패널")]
    Panels,
    #[translate(en = "Incentive Pool", ko = "인센티브 풀")]
    IncentivePool,
}

impl SpaceAppType {
    pub fn is_default(&self) -> bool {
        matches!(self, SpaceAppType::General | SpaceAppType::File)
    }

    pub fn settings_path(&self, space_id: SpacePartition) -> Route {
        match self {
            SpaceAppType::General => Route::SpaceGeneralAppPage { space_id },
            SpaceAppType::File => Route::SpaceFileAppPage { space_id },
            SpaceAppType::Panels => Route::SpacePanelsAppPage { space_id },
            SpaceAppType::IncentivePool => Route::SpaceIncentivePoolAppPage { space_id },
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            SpaceAppType::General => "bg-green-500",
            SpaceAppType::IncentivePool => "bg-amber-500",
            SpaceAppType::File => "bg-violet-500",
            SpaceAppType::Panels => "bg-sky-500",
        }
    }

    pub fn icon(&self) -> Element {
        match self {
            SpaceAppType::General => rsx! {
                icons::settings::Settings2 {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:fill-black [&>circle]:stroke-black",
                }
            },
            SpaceAppType::IncentivePool => rsx! {
                icons::ratel::Chest {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:fill-none [&>path]:stroke-black",
                }
            },
            SpaceAppType::File => rsx! {
                icons::file::File {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:stroke-black",
                }
            },
            SpaceAppType::Panels => rsx! {
                icons::user::UserGroup {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:stroke-black [&>path]:fill-transparent",
                }
            },
        }
    }
}
