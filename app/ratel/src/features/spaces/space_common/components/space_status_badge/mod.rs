use crate::features::spaces::space_common::*;

#[component]
pub fn SpaceStatusBadge(status: SpaceStatus) -> Element {
    let lang = use_language();

    let color = match status {
        SpaceStatus::Designing => BadgeColor::Purple,
        SpaceStatus::Open => BadgeColor::Blue,
        SpaceStatus::Ongoing => BadgeColor::Green,
        SpaceStatus::Processing => BadgeColor::Orange,
        SpaceStatus::Finished => BadgeColor::Grey,
    };

    rsx! {
        div { class: "whitespace-nowrap shrink-0",
            Badge { color, variant: BadgeVariant::Rounded, {status.translate(&lang())} }
        }
    }
}
