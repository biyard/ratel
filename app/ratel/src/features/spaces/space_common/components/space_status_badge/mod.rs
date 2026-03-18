use crate::features::spaces::space_common::*;

#[component]
pub fn SpaceStatusBadge(status: SpaceStatus) -> Element {
    let lang = use_language();

    let color = match status {
        SpaceStatus::InProgress => BadgeColor::Blue,
        SpaceStatus::Started => BadgeColor::Green,
        SpaceStatus::Waiting => BadgeColor::Orange,
        SpaceStatus::Finished => BadgeColor::Grey,
    };

    rsx! {
        Badge { color, variant: BadgeVariant::Rounded, {status.translate(&lang())} }
    }
}
