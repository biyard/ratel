use super::*;

#[component]
pub fn SpaceActionsLayout(space_id: ReadSignal<SpacePartition>) -> Element {
    let role = use_space_role();
    let ctx = Context::init()?;

    if !role().is_admin() {
        return rsx! {
            SuspenseBoundary { Outlet::<Route> {} }
        };
    }

    rsx! {
        div {
            id: "space-actions-layout",
            class: "flex flex-col gap-5 w-full h-full grow",
            Fragment {
                h1 { {ctx.title()} }
                div { class: "flex flex-col gap-10",
                    Tabs { tabs: ctx.tabs() }
                    SuspenseBoundary { Outlet::<Route> {} }
                }
            }
        }
    }
}

#[component]
pub fn Tabs(tabs: Vec<SpaceActionSettingTab>) -> Element {
    rsx! {
        div { id: "layout", class: "flex flex-row gap-5",
            for tab in tabs {
                Link { to: tab.target.clone(), {tab.label} }
            }
        }
    }
}
