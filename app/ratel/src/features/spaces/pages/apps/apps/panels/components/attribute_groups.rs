use crate::features::spaces::pages::apps::apps::panels::*;
use std::collections::HashSet;

translate! {
    AttributeGroupsTranslate;

    attribute_groups: {
        en: "Attribute groups",
        ko: "속성 그룹",
    },
    university: {
        en: "University",
        ko: "대학교",
    },
    gender: {
        en: "Gender",
        ko: "성별",
    },
    age: {
        en: "Age",
        ko: "나이",
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PanelOption {
    University,
    Age,
    Gender,
}

impl PanelOption {
    fn label(self, tr: &AttributeGroupsTranslate) -> String {
        match self {
            Self::University => tr.university.to_string(),
            Self::Age => tr.age.to_string(),
            Self::Gender => tr.gender.to_string(),
        }
    }
}

fn is_selected_option(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> bool {
    panels.iter().any(|panel| {
        panel_attributes(panel)
            .into_iter()
            .any(|attribute| matches_option(option, &attribute))
    })
}

fn option_keys(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> Vec<DeletePanelKey> {
    panels
        .iter()
        .filter(|panel| {
            panel_attributes(panel)
                .into_iter()
                .any(|attribute| matches_option(option, &attribute))
        })
        .map(|panel| DeletePanelKey {
            panel_id: panel.panel_id.clone(),
        })
        .collect()
}

fn panel_attributes(panel: &SpacePanelQuotaResponse) -> Vec<PanelAttribute> {
    if panel.attributes_vec.is_empty() && !matches!(panel.attributes, PanelAttribute::None) {
        vec![panel.attributes]
    } else {
        panel.attributes_vec.clone()
    }
}

fn matches_option(option: PanelOption, attribute: &PanelAttribute) -> bool {
    match (option, attribute) {
        (
            PanelOption::University,
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::University),
        ) => true,
        (PanelOption::Age, PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)) => true,
        (PanelOption::Age, PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))) => {
            true
        }
        (PanelOption::Gender, PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender)) => {
            true
        }
        (
            PanelOption::Gender,
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)),
        ) => true,
        _ => false,
    }
}

fn default_attributes(option: PanelOption, total_quota: i64) -> Vec<PanelAttributeWithQuota> {
    match option {
        PanelOption::University => {
            vec![PanelAttributeWithQuota::CollectiveAttribute(
                CollectiveAttribute::University,
            )]
        }
        PanelOption::Age => {
            let quotas = split_quotas(total_quota, 7);
            vec![
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Age(Age::Range {
                        inclusive_min: 0,
                        inclusive_max: 17,
                    }),
                    quota: quotas[0],
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Age(Age::Range {
                        inclusive_min: 18,
                        inclusive_max: 29,
                    }),
                    quota: quotas[1],
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Age(Age::Range {
                        inclusive_min: 30,
                        inclusive_max: 39,
                    }),
                    quota: quotas[2],
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Age(Age::Range {
                        inclusive_min: 40,
                        inclusive_max: 49,
                    }),
                    quota: quotas[3],
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Age(Age::Range {
                        inclusive_min: 50,
                        inclusive_max: 59,
                    }),
                    quota: quotas[4],
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Age(Age::Range {
                        inclusive_min: 60,
                        inclusive_max: 69,
                    }),
                    quota: quotas[5],
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Age(Age::Range {
                        inclusive_min: 70,
                        inclusive_max: u8::MAX,
                    }),
                    quota: quotas[6],
                }),
            ]
        }
        PanelOption::Gender => {
            let default_quota = (total_quota.max(0) + 1) / 2;
            vec![
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Gender(Gender::Male),
                    quota: default_quota,
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Gender(Gender::Female),
                    quota: default_quota,
                }),
            ]
        }
    }
}

fn split_quotas(total_quota: i64, count: usize) -> Vec<i64> {
    let total_quota = total_quota.max(0);
    let base = total_quota / count as i64;
    let remainder = total_quota % count as i64;

    (0..count)
        .map(|idx| {
            if idx < remainder as usize {
                base + 1
            } else {
                base
            }
        })
        .collect()
}

fn build_panel_groups(
    include_university: bool,
    include_age: bool,
    include_gender: bool,
    total_quota: i64,
) -> Vec<CreatePanelQuotaGroupRequest> {
    let mut groups = vec![];

    if include_university {
        groups.push(CreatePanelQuotaGroupRequest {
            attributes_vec: vec![PanelAttribute::CollectiveAttribute(
                CollectiveAttribute::University,
            )],
            quota: 1_000_000_000,
        });
    }

    if include_age && include_gender {
        let age_ranges = vec![
            Age::Range {
                inclusive_min: 0,
                inclusive_max: 17,
            },
            Age::Range {
                inclusive_min: 18,
                inclusive_max: 29,
            },
            Age::Range {
                inclusive_min: 30,
                inclusive_max: 39,
            },
            Age::Range {
                inclusive_min: 40,
                inclusive_max: 49,
            },
            Age::Range {
                inclusive_min: 50,
                inclusive_max: 59,
            },
            Age::Range {
                inclusive_min: 60,
                inclusive_max: 69,
            },
            Age::Range {
                inclusive_min: 70,
                inclusive_max: u8::MAX,
            },
        ];
        let quotas = split_quotas(total_quota, age_ranges.len() * 2);

        for (age_idx, age_range) in age_ranges.into_iter().enumerate() {
            for (gender_idx, gender) in [Gender::Male, Gender::Female].into_iter().enumerate() {
                groups.push(CreatePanelQuotaGroupRequest {
                    attributes_vec: vec![
                        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(age_range)),
                        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(gender)),
                    ],
                    quota: quotas[age_idx * 2 + gender_idx],
                });
            }
        }
    } else if include_age {
        groups.extend(
            default_attributes(PanelOption::Age, total_quota)
                .into_iter()
                .map(|attribute| {
                    let quota = attribute.quota();
                    CreatePanelQuotaGroupRequest {
                        attributes_vec: vec![attribute.into()],
                        quota,
                    }
                }),
        );
    } else if include_gender {
        groups.extend(
            default_attributes(PanelOption::Gender, total_quota)
                .into_iter()
                .map(|attribute| {
                    let quota = attribute.quota();
                    CreatePanelQuotaGroupRequest {
                        attributes_vec: vec![attribute.into()],
                        quota,
                    }
                }),
        );
    }

    groups
}

async fn rebuild_panels(
    space_id: SpacePartition,
    keys: Vec<DeletePanelKey>,
    groups: Vec<CreatePanelQuotaGroupRequest>,
) -> crate::common::Result<()> {
    if !keys.is_empty() {
        delete_panel_quotas(space_id.clone(), DeletePanelQuotaRequest { keys }).await?;
    }

    if !groups.is_empty() {
        create_panel_quotas(
            space_id,
            CreatePanelQuotaRequest {
                attributes: vec![],
                attributes_vec: groups,
            },
        )
        .await?;
    }

    Ok(())
}

#[component]
pub fn AttributeGroups(
    space_id: SpacePartition,
    panels: Vec<SpacePanelQuotaResponse>,
    current_quota: i64,
    panels_query_key: Vec<String>,
) -> Element {
    let tr: AttributeGroupsTranslate = use_translate();
    let mut toast = use_toast();
    let has_university = is_selected_option(PanelOption::University, &panels);
    let has_age = is_selected_option(PanelOption::Age, &panels);
    let has_gender = is_selected_option(PanelOption::Gender, &panels);
    let panels_query_key_for_university = panels_query_key.clone();
    let panels_query_key_for_age = panels_query_key.clone();
    let panels_query_key_for_gender = panels_query_key.clone();
    let managed_keys = {
        let mut keys = option_keys(PanelOption::University, &panels);
        keys.extend(option_keys(PanelOption::Age, &panels));
        keys.extend(option_keys(PanelOption::Gender, &panels));
        let mut seen = HashSet::new();
        keys.retain(|key| seen.insert(key.panel_id.clone()));
        keys
    };

    let on_toggle_university = {
        let space_id = space_id.clone();
        let keys = managed_keys.clone();
        move |_| {
            let space_id = space_id.clone();
            let panels_query_key = panels_query_key_for_university.clone();
            let keys = keys.clone();
            let mut toast = toast;
            let next_university = !has_university;
            let next_age = has_age;
            let next_gender = has_gender;
            spawn(async move {
                let result = rebuild_panels(
                    space_id.clone(),
                    keys,
                    build_panel_groups(next_university, next_age, next_gender, current_quota),
                )
                .await;

                match result {
                    Ok(_) => invalidate_query(&panels_query_key),
                    Err(err) => {
                        error!("Failed to toggle university panel: {:?}", err);
                        toast.error(err);
                    }
                }
            });
        }
    };
    let on_toggle_age = {
        let space_id = space_id.clone();
        let keys = managed_keys.clone();
        move |_| {
            let space_id = space_id.clone();
            let panels_query_key = panels_query_key_for_age.clone();
            let keys = keys.clone();
            let mut toast = toast;
            let next_university = has_university;
            let next_age = !has_age;
            let next_gender = has_gender;
            spawn(async move {
                let result = rebuild_panels(
                    space_id.clone(),
                    keys,
                    build_panel_groups(next_university, next_age, next_gender, current_quota),
                )
                .await;

                match result {
                    Ok(_) => invalidate_query(&panels_query_key),
                    Err(err) => {
                        error!("Failed to toggle age panels: {:?}", err);
                        toast.error(err);
                    }
                }
            });
        }
    };
    let on_toggle_gender = {
        let space_id = space_id.clone();
        let keys = managed_keys.clone();
        move |_| {
            let space_id = space_id.clone();
            let panels_query_key = panels_query_key_for_gender.clone();
            let keys = keys.clone();
            let mut toast = toast;
            let next_university = has_university;
            let next_age = has_age;
            let next_gender = !has_gender;
            spawn(async move {
                let result = rebuild_panels(
                    space_id.clone(),
                    keys,
                    build_panel_groups(next_university, next_age, next_gender, current_quota),
                )
                .await;

                match result {
                    Ok(_) => invalidate_query(&panels_query_key),
                    Err(err) => {
                        error!("Failed to toggle gender panels: {:?}", err);
                        toast.error(err);
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "flex items-center gap-5 min-w-0 flex-1",
            div { class: "text-sm font-medium text-text-primary whitespace-nowrap",
                {tr.attribute_groups}
            }
            div { class: "flex flex-wrap items-center gap-2 min-w-0",
                AttributeButton {
                    label: PanelOption::University.label(&tr),
                    selected: has_university,
                    onclick: on_toggle_university,
                }
                AttributeButton {
                    label: PanelOption::Age.label(&tr),
                    selected: has_age,
                    onclick: on_toggle_age,
                }
                AttributeButton {
                    label: PanelOption::Gender.label(&tr),
                    selected: has_gender,
                    onclick: on_toggle_gender,
                }
            }
        }
    }
}
