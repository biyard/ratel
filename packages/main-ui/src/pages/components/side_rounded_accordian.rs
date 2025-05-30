use bdk::prelude::{
    by_components::icons::arrows::{ShapeArrowDown, ShapeArrowUp},
    *,
};

#[component]
pub fn SideRoundedAccordian(icon: Element, title: String, children: Element) -> Element {
    let mut is_clicked = use_signal(|| true);

    rsx! {
        div { class: "flex flex-col px-16 py-20 rounded-[10px] bg-footer gap-20",
            div { class: "flex flex-col w-218 gap-20",
                button {
                    class: "cursor-pointer flex flex-row  justify-between items-center",
                    onclick: move |_| {
                        is_clicked.set(!is_clicked());
                    },
                    div { class: "flex flex-row w-full justify-start items-center gap-4",
                        {icon}
                        div { class: "font-bold text-sm/16 text-neutral-500", {title} }
                    }

                    if is_clicked() {
                        div { class: "flex flex-row w-fit h-fit",
                            ShapeArrowDown {
                                class: "[&>path]:stroke-white [&>path]:fill-white",
                                size: 14,
                                fill: "white",
                            }
                        }
                    } else {
                        div { class: "flex flex-row w-fit h-fit",
                            ShapeArrowUp {
                                class: "[&>path]:stroke-white [&>path]:fill-white",
                                size: 14,
                                fill: "white",
                            }
                        }
                    }
                }

                div {
                    class: "flex flex-col aria-hidden:hidden",
                    "aria-hidden": !is_clicked(),
                    {children}
                }
            }
        }
    }
}
