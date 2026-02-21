use super::*;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    let tr: HomeTranslate = use_translate();
    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col gap-5 items-start w-full",
            h3 { "{tr.action}" }

            div { class: "flex flex-col gap-5 justify-center items-center py-5 px-4 w-full border border-dashed rounded-[12px] bg-sp-act-card-bg border-sp-act-card-stroke",
                ActionIcon {}
                span { "{tr.description}" }
                Button {
                    style: ButtonStyle::Secondary,
                    onclick: move |_| {
                        nav.push(Route::NewActionPage {
                            space_id: space_id.clone(),
                        });
                    },
                    "{tr.btn_create_action}"
                }
            }

        }
    }
}

#[component]
pub fn ActionIcon() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "64",
            view_box: "0 0 64 64",
            width: "64",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M52.904 28.5663L28.7681 54.119C27.9617 54.9724 26.8859 55.5217 25.7218 55.6744C24.5577 55.8271 23.3766 55.5738 22.3775 54.9572C21.3783 54.3407 20.6223 53.3985 20.2368 52.2896C19.8513 51.1806 19.8598 49.9727 20.261 48.8693L24.0939 38.3256H15.1663C14.2996 38.3256 13.446 38.1132 12.6804 37.707C11.9148 37.3007 11.2604 36.713 10.7745 35.9952C10.2886 35.2775 9.98603 34.4516 9.89326 33.5898C9.80049 32.728 9.92035 31.8567 10.2423 31.052L17.5887 12.6875C18.1103 11.3883 19.0083 10.2747 20.1674 9.48975C21.3266 8.70477 22.694 8.28425 24.0939 8.28223H36.7438C39.8051 8.28223 41.9207 11.3403 40.8487 14.2055L38.325 20.9321H49.6119C50.4961 20.932 51.3609 21.191 52.0996 21.6771C52.8383 22.1631 53.4184 22.8549 53.7682 23.667C54.1181 24.4791 54.2224 25.3758 54.0682 26.2465C53.9141 27.1172 53.5114 27.9237 52.904 28.5663Z",
                stroke: "#737373",
                stroke_width: "4",
            }
        }
    }
}
