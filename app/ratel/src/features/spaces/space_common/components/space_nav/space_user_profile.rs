use crate::common::components::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};
use crate::features::spaces::space_common::*;

#[component]
pub fn SpaceUserProfile(
    image: String,
    display_name: String,
    user_role: SpaceUserRole,
    real_role: SpaceUserRole,
    on_role_change: EventHandler<SpaceUserRole>,
) -> Element {
    let lang = use_language();
    let lang = lang();

    let is_admin = real_role.is_admin();

    rsx! {
        div { id: "space-user-profile", class: "p-4 w-full",
            div { class: "flex flex-row gap-2.5",
                img {
                    src: image,
                    class: "object-cover w-12 h-12 rounded-full shrink-0",
                }
                div { class: "flex flex-col gap-1",
                    span { class: "text-sm font-medium text-text", {display_name} }
                    if is_admin {
                        ContextMenu {
                            ContextMenuTrigger {
                                span { class: "text-xs cursor-context-menu text-text-secondary",
                                    {user_role.translate(&lang)}
                                }
                            }
                            ContextMenuContent {
                                for (idx , (role , value)) in [
                                    (SpaceUserRole::Creator, "admin"),
                                    (SpaceUserRole::Viewer, "viewer"),
                                    (SpaceUserRole::Candidate, "candidate"),
                                    (SpaceUserRole::Participant, "participant"),
                                ]
                                    .into_iter()
                                    .enumerate()
                                {
                                    ContextMenuItem {
                                        key: "{value}",
                                        value,
                                        index: idx,
                                        on_select: move |_| on_role_change.call(role),
                                        {role.translate(&lang)}
                                    }
                                }
                            }
                        }
                    } else {
                        span { class: "text-xs text-text-secondary", {user_role.translate(&lang)} }
                    }
                }
            }
        }
    }
}
