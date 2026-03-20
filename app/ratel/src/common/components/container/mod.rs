use crate::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContainerProps {
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
    bottom_sheet: Option<Element>,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let base = attributes!(div {
        class: "flex flex-col h-screen"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        div {..merged,
            div { class: "flex overflow-y-auto flex-1 justify-center items-start w-full scrollbar-none",
                {props.children}
            }
            if let Some(bottom_sheet) = props.bottom_sheet {
                Card {
                    variant: CardVariant::Filled,
                    shape: CardShape::Squere,
                    class: "flex flex-row justify-between items-center w-full h-20",
                    {bottom_sheet}
                }
            }
        }
    }
}
