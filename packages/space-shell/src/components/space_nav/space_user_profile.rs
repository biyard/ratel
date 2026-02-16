use crate::*;

#[component]
pub fn SpaceUserProfile(image: String, display_name: String, user_role: SpaceUserRole) -> Element {
    let lang = use_language();

    rsx! {
        div { id: "space-user-profile", class: "p-4 w-full",
            div { class: "flex flex-row gap-2.5",
                img {
                    src: image,
                    class: "object-cover w-12 h-12 rounded-full shrink-0",
                }
                div { class: "flex flex-col gap-1",
                    span { class: "text-sm font-medium text-text", {display_name} }
                    span { class: "text-xs text-text-secondary", {user_role.translate(&lang)} }
                }
            }
        }
    }
}
