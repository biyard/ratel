use crate::*;

#[component]
pub fn SuspenseBoundary(children: Element) -> Element {
    rsx! {
        dioxus::prelude::SuspenseBoundary {
            fallback: |_ctx: SuspenseContext| {
                // if let Some(el) = ctx.suspended_nodes() {
                //     debug!("Suspended nodes: {el:#?}");
                //     return rsx! {
                //         {el}
                //     };
                // }
                rsx! {}
            },
            {children}
        }
    }
}
