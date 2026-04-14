use crate::features::spaces::pages::apps::apps::panels::*;
use dioxus::fullstack::Loader;
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
pub enum PanelOption {
    University,
    Age,
    Gender,
}

impl PanelOption {
    pub fn label(self, tr: &AttributeGroupsTranslate) -> String {
        match self {
            Self::University => tr.university.to_string(),
            Self::Age => tr.age.to_string(),
            Self::Gender => tr.gender.to_string(),
        }
    }
}

pub fn is_selected_option(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> bool {
    panels.iter().any(|panel| {
        panel_attributes(panel)
            .into_iter()
            .any(|attribute| matches_option(option, &attribute))
    })
}

pub fn is_collective_option(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> bool {
    panels.iter().any(|panel| {
        panel_attributes(panel).into_iter().any(|attribute| {
            matches!(
                (option, &attribute),
                (
                    PanelOption::University,
                    PanelAttribute::CollectiveAttribute(CollectiveAttribute::University)
                ) | (
                    PanelOption::Age,
                    PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)
                ) | (
                    PanelOption::Gender,
                    PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender)
                )
            )
        })
    })
}

pub fn is_conditional_option(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> bool {
    panels.iter().any(|panel| {
        panel_attributes(panel).into_iter().any(|attribute| {
            matches!(
                (option, &attribute),
                (
                    PanelOption::Age,
                    PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))
                ) | (
                    PanelOption::Gender,
                    PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_))
                )
            )
        })
    })
}

pub fn option_keys(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> Vec<DeletePanelKey> {
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

pub fn panel_attributes(panel: &SpacePanelQuotaResponse) -> Vec<PanelAttribute> {
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

fn build_collective_groups(
    include_university: bool,
    include_age: bool,
    include_gender: bool,
) -> Vec<CreatePanelQuotaGroupRequest> {
    let mut groups = vec![];

    if include_university {
        groups.push(CreatePanelQuotaGroupRequest {
            attributes_vec: vec![PanelAttribute::CollectiveAttribute(
                CollectiveAttribute::University,
            )],
            quota: 0,
        });
    }

    if include_age {
        groups.push(CreatePanelQuotaGroupRequest {
            attributes_vec: vec![PanelAttribute::CollectiveAttribute(
                CollectiveAttribute::Age,
            )],
            quota: 0,
        });
    }

    if include_gender {
        groups.push(CreatePanelQuotaGroupRequest {
            attributes_vec: vec![PanelAttribute::CollectiveAttribute(
                CollectiveAttribute::Gender,
            )],
            quota: 0,
        });
    }

    groups
}

pub fn build_conditional_groups(
    option: PanelOption,
    total_quota: i64,
    include_gender_cross: bool,
) -> Vec<CreatePanelQuotaGroupRequest> {
    let mut groups = vec![];

    match option {
        PanelOption::Age => {
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

            if include_gender_cross {
                let quotas = split_quotas(total_quota, age_ranges.len() * 2);
                for (age_idx, age_range) in age_ranges.into_iter().enumerate() {
                    for (gender_idx, gender) in
                        [Gender::Male, Gender::Female].into_iter().enumerate()
                    {
                        groups.push(CreatePanelQuotaGroupRequest {
                            attributes_vec: vec![
                                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
                                    age_range,
                                )),
                                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(
                                    gender,
                                )),
                            ],
                            quota: quotas[age_idx * 2 + gender_idx],
                        });
                    }
                }
            } else {
                let quotas = split_quotas(total_quota, age_ranges.len());
                for (idx, age_range) in age_ranges.into_iter().enumerate() {
                    groups.push(CreatePanelQuotaGroupRequest {
                        attributes_vec: vec![PanelAttribute::VerifiableAttribute(
                            VerifiableAttribute::Age(age_range),
                        )],
                        quota: quotas[idx],
                    });
                }
            }
        }
        PanelOption::Gender => {
            let quotas = split_quotas(total_quota, 2);
            groups.push(CreatePanelQuotaGroupRequest {
                attributes_vec: vec![PanelAttribute::VerifiableAttribute(
                    VerifiableAttribute::Gender(Gender::Male),
                )],
                quota: quotas[0],
            });
            groups.push(CreatePanelQuotaGroupRequest {
                attributes_vec: vec![PanelAttribute::VerifiableAttribute(
                    VerifiableAttribute::Gender(Gender::Female),
                )],
                quota: quotas[1],
            });
        }
        PanelOption::University => {}
    }

    groups
}

pub async fn rebuild_panels(
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
    space_id: ReadSignal<SpacePartition>,
    panels: Vec<SpacePanelQuotaResponse>,
    current_quota: i64,
    panels_loader: Loader<Vec<SpacePanelQuotaResponse>>,
) -> Element {
    let tr: AttributeGroupsTranslate = use_translate();
    let mut toast = use_toast();
    let has_university = is_selected_option(PanelOption::University, &panels);
    let has_age = is_selected_option(PanelOption::Age, &panels);
    let has_gender = is_selected_option(PanelOption::Gender, &panels);
    let is_age_conditional = is_conditional_option(PanelOption::Age, &panels);
    let is_gender_conditional = is_conditional_option(PanelOption::Gender, &panels);

    let managed_keys = {
        let mut keys = option_keys(PanelOption::University, &panels);
        keys.extend(option_keys(PanelOption::Age, &panels));
        keys.extend(option_keys(PanelOption::Gender, &panels));
        let mut seen = HashSet::new();
        keys.retain(|key| seen.insert(key.panel_id.clone()));
        keys
    };

    let make_toggle = {
        move |option: PanelOption| {
            let keys = managed_keys.clone();
            let mut toast = toast;
            let mut panels_loader = panels_loader;

            let (next_uni, next_age, next_gender) = match option {
                PanelOption::University => (!has_university, has_age, has_gender),
                PanelOption::Age => (has_university, !has_age, has_gender),
                PanelOption::Gender => (has_university, has_age, !has_gender),
            };

            let next_age_conditional = if option == PanelOption::Age {
                false
            } else {
                is_age_conditional
            };
            let next_gender_conditional = if option == PanelOption::Gender {
                false
            } else {
                is_gender_conditional
            };

            move |_| {
                let keys = keys.clone();
                spawn(async move {
                    let mut groups = build_collective_groups(
                        next_uni,
                        next_age && !next_age_conditional,
                        next_gender && !next_gender_conditional,
                    );

                    if next_age && next_age_conditional {
                        groups.extend(build_conditional_groups(
                            PanelOption::Age,
                            current_quota,
                            next_gender_conditional,
                        ));
                    }
                    if next_gender && next_gender_conditional && !next_age_conditional {
                        groups.extend(build_conditional_groups(
                            PanelOption::Gender,
                            current_quota,
                            false,
                        ));
                    }

                    match rebuild_panels(space_id(), keys, groups).await {
                        Ok(_) => panels_loader.restart(),
                        Err(err) => {
                            error!("Failed to toggle panel: {:?}", err);
                            toast.error(err);
                        }
                    }
                });
            }
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
                    onclick: make_toggle(PanelOption::University),
                }
                AttributeButton {
                    label: PanelOption::Age.label(&tr),
                    selected: has_age,
                    onclick: make_toggle(PanelOption::Age),
                }
                AttributeButton {
                    label: PanelOption::Gender.label(&tr),
                    selected: has_gender,
                    onclick: make_toggle(PanelOption::Gender),
                }
            }
        }
    }
}
