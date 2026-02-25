use crate::*;

#[component]
pub fn AllAppsPage(space_id: SpacePartition) -> Element {
    // FIXME: Use space_id when space-scoped data is added.
    let _ = space_id;
    let mut installed = use_signal(|| false);

    rsx! {
        div { class: "grid grid-cols-3 gap-5 content-start items-start w-full max-tablet:grid-cols-2 max-mobile:grid-cols-1",
            div { class: "flex flex-col items-start w-full gap-[10px] rounded-t-[16px] bg-card p-[15px]",
                div { class: "flex justify-center items-center w-10 h-10 bg-violet-500 rounded-[10px]",
                    icons::ratel::Chest {
                        width: "24",
                        height: "24",
                        class: "text-font-primary [&>path]:fill-none [&>path]:stroke-current",
                    }
                }
                div { class: "flex flex-col items-start self-stretch gap-[6px]",
                    p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                        "Incentive Pool"
                    }
                    p { class: "font-medium leading-4 sp-dash-font-raleway text-[12px] tracking-[0] text-card-meta",
                        "The official server for the here. Welcome to our channel, Traveler! This is the ..."
                    }
                }
                if installed() {
                    button {
                        class: "flex flex-col justify-center items-center self-stretch px-5 py-3 w-full font-bold leading-5 border gap-[10px] rounded-[10px] border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text sp-dash-font-raleway text-[17px] tracking-[-0.18px]",
                        onclick: move |_| {
                            installed.set(false);
                        },
                        "Uninstall"
                    }
                } else {
                    button {
                        class: "flex flex-col justify-center items-center self-stretch px-5 py-3 w-full font-bold leading-5 gap-[10px] rounded-[10px] bg-btn-primary-bg text-btn-primary-text sp-dash-font-raleway text-[17px] tracking-[-0.18px]",
                        onclick: move |_| {
                            installed.set(true);
                        },
                        "Install"
                    }
                }
            }
        }
    }
}

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role =
        use_loader(move || async move { Ok::<SpaceUserRole, Error>(SpaceUserRole::Creator) })?;

    if role() == SpaceUserRole::Creator {
        rsx! {
            AllAppsPage { space_id }
        }
    } else {
        rsx! {
            div { class: "flex justify-center items-center w-full h-full text-font-primary",
                "No permission"
            }
        }
    }
}
