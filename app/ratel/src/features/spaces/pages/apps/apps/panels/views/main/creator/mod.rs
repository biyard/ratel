use super::*;

const PANELS_QUERY_KEY: &str = "Panels";

fn panels_key(space_id: &SpacePartition) -> Vec<String> {
    vec![
        "Space".to_string(),
        space_id.to_string(),
        PANELS_QUERY_KEY.to_string(),
    ]
}

#[component]
pub fn PanelPage(space_id: SpacePartition) -> Element {
    let panels_query_key = panels_key(&space_id);
    let panels_loader = use_query(&panels_query_key, {
        let space_id = space_id.clone();
        move || list_panels(space_id.clone())
    })?;

    let panels: Vec<SpacePanelQuotaResponse> = panels_loader.read().clone();
    let space = use_space();

    rsx! {
        div { class: "flex w-full flex-col gap-5 max-w-[1024px]",
            div { class: "flex flex-wrap items-center gap-5 min-w-0",
                TotalQuotas { space_id: space_id.clone(), quota: space().quota }
                AttributeGroups {
                    space_id: space_id.clone(),
                    panels: panels.clone(),
                    current_quota: space().quota,
                    panels_query_key: panels_query_key.clone(),
                }
            }
            PanelsTable { space_id, panels, panels_query_key }
        }
    }
}

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    rsx! {
        PanelPage { space_id }
    }
}
