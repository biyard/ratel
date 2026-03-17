use crate::features::spaces::pages::apps::{views::main::i18n::AppMainTranslate, *};

#[component]
pub fn AppCard(
    app_type: SpaceAppType,
    #[props(optional)] header_action: Option<Element>,
    children: Option<Element>,
) -> Element {
    let lang = use_language();
    let tr: AppMainTranslate = use_translate();

    rsx! {
        SpaceCard { class: "flex overflow-hidden flex-col items-start rounded-lg !p-0",
            div { class: "flex flex-col gap-2.5 p-4 w-full",
                div { class: "flex gap-3 justify-between items-start w-full",
                    div { class: "flex justify-center items-center shrink-0 size-11 rounded-[10px] {app_type.class()}",
                        {app_type.icon()}
                    }
                    if let Some(header_action) = header_action {
                        div { class: "flex justify-center items-center shrink-0", {header_action} }
                    }
                }
                p { class: "text-sm font-bold sp-dash-font-raleway text-font-primary",
                    {app_type.translate(&lang())}
                }
                p { class: "h-8 text-xs font-medium leading-4 sp-dash-font-raleway text-font-body line-clamp-2",
                    {app_type.description(&lang())}
                }
            }

            if let Some(children) = children {
                div { class: "flex flex-col items-start py-3 px-4 w-full border-t border-web-card-divider",
                    {children}
                }
            }
        }
    }
}
