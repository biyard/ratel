use dioxus_animations::use_animated;

#[component]
fn AnimatedCard(content: Element) -> Element {
    let (ref node, _controller) = use_animated("fade-in slide-up");
    
    rsx! {
        div { ref: node, class: "transition-all duration-500", content }
    }
}
