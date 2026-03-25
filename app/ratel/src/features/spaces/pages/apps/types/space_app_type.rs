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
    #[translate(en = "Analyzes", ko = "분석")]
    Analyzes,
    #[translate(en = "Panels", ko = "패널")]
    Panels,
    #[cfg(feature = "beta")]
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
            SpaceAppType::Analyzes => Route::SpaceAnalyzesAppPage { space_id },
            SpaceAppType::Panels => Route::SpacePanelsAppPage { space_id },
            #[cfg(feature = "beta")]
            SpaceAppType::IncentivePool => Route::SpaceIncentivePoolAppPage { space_id },
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            SpaceAppType::General => "bg-green-500",
            SpaceAppType::Analyzes => "bg-cyan-500",
            #[cfg(feature = "beta")]
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
                    class: "text-white [&>path]:fill-current [&>circle]:stroke-current",
                }
            },
            SpaceAppType::Analyzes => rsx! {
                icons::layouts::Apps {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:stroke-black",
                }
            },
            #[cfg(feature = "beta")]
            SpaceAppType::IncentivePool => rsx! {
                icons::ratel::Chest {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:fill-none [&>path]:stroke-current",
                }
            },
            SpaceAppType::File => rsx! {
                icons::file::File {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:stroke-current",
                }
            },
            SpaceAppType::Panels => rsx! {
                icons::user::UserGroup {
                    width: "24",
                    height: "24",
                    class: "text-white [&>path]:stroke-current [&>path]:fill-transparent",
                }
            },
        }
    }

    pub fn description(&self, lang: &Language) -> String {
        let tr = SpaceAppTypeTranslate::new(lang);

        match self {
            #[cfg(feature = "beta")]
            SpaceAppType::IncentivePool => tr.app_description_incentive_pool,
            SpaceAppType::File => tr.app_description_file,
            SpaceAppType::Analyzes => tr.app_description_analyzes,
            SpaceAppType::Panels => tr.app_description_panels,
            SpaceAppType::General => tr.app_description_general,
        }
        .to_string()
    }
}

translate! {
    SpaceAppTypeTranslate;

    app_description_incentive_pool: {
        en: "An incentive granted by the space creator, and granted upon finising according to the incentive rules",
        ko: "스페이스 생성자가 부여하는 인센티브로 종료 시 인센티브 규칙에 따라 부여됩니다.",
    },
    app_description_file: {
        en: "Manage and organize files shared in your space.",
        ko: "스페이스에서 공유되는 파일을 관리하고 정리하세요.",
    },
    app_description_panels: {
        en: "Configure panel participation rules",
        ko: "패널 참여 조건을 설정하세요.",
    },
    app_description_general: {
        en: "Settings (Admin)",
        ko: "설정(관리자)",
    },
    app_description_analyzes: {
        en: "Check out the poll results.",
        ko: "설문조사 결과를 확인하세요.",
    },
}
