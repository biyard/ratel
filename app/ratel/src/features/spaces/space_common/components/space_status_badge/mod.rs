use crate::features::spaces::space_common::*;

#[component]
pub fn SpaceStatusBadge(status: SpaceStatus) -> Element {
    let lang = use_language();

    rsx! {
        Badge { color: BadgeColor::Green, variant: BadgeVariant::Rounded, {status.translate(&lang())} }
    }
}
