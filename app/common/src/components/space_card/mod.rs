use crate::*;

#[component]
pub fn SpaceCard(
    #[props(optional)] class: Option<String>,
    #[props(default = false)] fill_height: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let height_class = if fill_height {
        "h-full min-h-0"
    } else {
        "h-auto"
    };
    let base_class = format!(
        "{height_class} w-full rounded-2xl max-mobile:rounded-xl bg-web-card-bg p-[30px] max-tablet:p-5 max-mobile:p-4"
    );
    let class = match class {
        Some(extra) if !extra.is_empty() => format!("{} {}", base_class, extra),
        _ => base_class,
    };

    rsx! {
        div {
            class,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            ..attributes,
            {children}
        }
    }
}
