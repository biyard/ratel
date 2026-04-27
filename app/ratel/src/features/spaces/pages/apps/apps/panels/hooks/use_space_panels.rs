use crate::features::spaces::pages::apps::apps::panels::controllers::*;
use crate::features::spaces::pages::apps::apps::panels::*;
use crate::features::spaces::space_common::controllers::{UpdateSpaceRequest, update_space};
use crate::features::spaces::space_common::hooks::use_space;
use crate::*;
use std::collections::HashSet;

// ── Panel option enum + pure logic helpers ───────────────────────────
//
// These are derived-state helpers from the original
// `components/attribute_groups.rs`. They are pure functions over the
// current panel list; moved into the hook module so both the hook and
// view sections can reach them without cross-referencing components.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelOption {
    University,
    Age,
    Gender,
}

pub fn panel_attributes(panel: &SpacePanelQuotaResponse) -> Vec<PanelAttribute> {
    if panel.attributes_vec.is_empty() && !matches!(panel.attributes, PanelAttribute::None) {
        vec![panel.attributes]
    } else {
        panel.attributes_vec.clone()
    }
}

fn matches_option(option: PanelOption, attribute: &PanelAttribute) -> bool {
    matches!(
        (option, attribute),
        (
            PanelOption::University,
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::University)
        ) | (
            PanelOption::Age,
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)
        ) | (
            PanelOption::Age,
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))
        ) | (
            PanelOption::Gender,
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender)
        ) | (
            PanelOption::Gender,
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_))
        )
    )
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
            attributes_vec: vec![PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)],
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

fn build_conditional_groups(
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

// ── Controller hook ───────────────────────────────────────────────────
//
// Bundles every mutation the panel settings arena needs. The shared
// `SpaceResponse` loader is still accessed via `use_space()` — this hook
// only layers panel-specific state on top.
//
// Every action updates the panel list optimistically by calling
// `panels.restart()` on success. Errors surface as toasts; the action
// itself always returns `Ok(())` so `.pending()` accurately reflects the
// in-flight request instead of a failed prior call.

#[derive(Clone, Copy)]
pub struct UseSpacePanels {
    pub space_id: ReadSignal<SpacePartition>,
    pub panels: Loader<Vec<SpacePanelQuotaResponse>>,

    pub update_total_quota: Action<(i64,), ()>,
    pub toggle_attribute: Action<(PanelOption,), ()>,
    pub move_to_conditional: Action<(PanelOption,), ()>,
    pub update_row_quota: Action<(SpacePanelAttributeEntityType, i64), ()>,
    pub delete_row: Action<(SpacePanelAttributeEntityType,), ()>,
}

#[track_caller]
pub fn use_space_panels(
    space_id: ReadSignal<SpacePartition>,
) -> std::result::Result<UseSpacePanels, RenderError> {
    if let Some(ctx) = try_use_context::<UseSpacePanels>() {
        return Ok(ctx);
    }

    let mut toast = use_toast();
    let mut space = use_space();

    let mut panels = use_loader(move || async move { list_panels(space_id()).await })?;

    // ── Total quota (lives on Space, not on panel entity) ────────────
    let update_total_quota = use_action(move |next_quota: i64| async move {
        match update_space(space_id(), UpdateSpaceRequest::Quota { quotas: next_quota }).await {
            Ok(_) => {
                space.restart();
                panels.restart();
            }
            Err(err) => {
                crate::error!("Failed to update panel total quota: {:?}", err);
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    // ── Attribute group toggle (University / Age / Gender) ───────────
    let toggle_attribute = use_action(move |option: PanelOption| async move {
        let current = panels.read().clone();
        let current_quota = space().quota;

        let has_university = is_selected_option(PanelOption::University, &current);
        let has_age = is_selected_option(PanelOption::Age, &current);
        let has_gender = is_selected_option(PanelOption::Gender, &current);
        let is_age_conditional = is_conditional_option(PanelOption::Age, &current);
        let is_gender_conditional = is_conditional_option(PanelOption::Gender, &current);

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

        let mut keys = option_keys(PanelOption::University, &current);
        keys.extend(option_keys(PanelOption::Age, &current));
        keys.extend(option_keys(PanelOption::Gender, &current));
        let mut seen = HashSet::new();
        keys.retain(|key| seen.insert(key.panel_id.clone()));

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
            Ok(_) => panels.restart(),
            Err(err) => {
                crate::error!("Failed to toggle panel: {:?}", err);
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    // ── Move a collective option into the conditional table ──────────
    let move_to_conditional = use_action(move |option: PanelOption| async move {
        let current = panels.read().clone();
        let current_quota = space().quota;

        let is_age_conditional = is_conditional_option(PanelOption::Age, &current);
        let is_gender_conditional = is_conditional_option(PanelOption::Gender, &current);

        let will_age_be_conditional = option == PanelOption::Age || is_age_conditional;
        let will_gender_be_conditional = option == PanelOption::Gender || is_gender_conditional;

        let mut keys = option_keys(option, &current);
        if will_age_be_conditional && will_gender_be_conditional {
            let other = if option == PanelOption::Age {
                PanelOption::Gender
            } else {
                PanelOption::Age
            };
            keys.extend(option_keys(other, &current));
            let mut seen = HashSet::new();
            keys.retain(|k| seen.insert(k.panel_id.clone()));
        }

        let groups = if will_age_be_conditional && will_gender_be_conditional {
            build_conditional_groups(PanelOption::Age, current_quota, true)
        } else {
            build_conditional_groups(option, current_quota, false)
        };

        match rebuild_panels(space_id(), keys, groups).await {
            Ok(_) => panels.restart(),
            Err(err) => {
                crate::error!("Failed to move to conditional: {:?}", err);
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    // ── Per-row conditional quota update ─────────────────────────────
    let update_row_quota = use_action(
        move |panel_id: SpacePanelAttributeEntityType, quota: i64| async move {
            match update_panel_quota(
                space_id(),
                UpdatePanelQuotaRequest { panel_id, quota },
            )
            .await
            {
                Ok(_) => panels.restart(),
                Err(err) => {
                    crate::error!("Failed to update panel row quota: {:?}", err);
                    toast.error(err);
                }
            }
            Ok::<(), crate::common::Error>(())
        },
    );

    // ── Per-row delete ───────────────────────────────────────────────
    let delete_row = use_action(move |panel_id: SpacePanelAttributeEntityType| async move {
        let keys = vec![DeletePanelKey { panel_id }];
        match delete_panel_quotas(space_id(), DeletePanelQuotaRequest { keys }).await {
            Ok(_) => panels.restart(),
            Err(err) => {
                crate::error!("Failed to delete panel row: {:?}", err);
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSpacePanels {
        space_id,
        panels,
        update_total_quota,
        toggle_attribute,
        move_to_conditional,
        update_row_quota,
        delete_row,
    }))
}
