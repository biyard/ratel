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
pub fn PanelPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let panels_query_key = panels_key(&space_id());
    let panels_loader = use_query(&panels_query_key, { move || list_panels(space_id()) })?;

    let panels: Vec<SpacePanelQuotaResponse> = panels_loader.read().clone();
    let space = use_space();

    rsx! {
        div { class: "flex w-full flex-col gap-5 max-w-[1024px]",
            div { class: "flex flex-wrap items-center gap-5 min-w-0",
                TotalQuotas { space_id, quota: space().quota }
                AttributeGroups {
                    space_id,
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
pub fn CreatorPage(space_id: ReadSignal<SpacePartition>) -> Element {
    rsx! {
        PanelPage { space_id }
    }
}
