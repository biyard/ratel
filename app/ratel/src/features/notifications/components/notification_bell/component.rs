use crate::common::*;
use crate::features::notifications::hooks::use_unread_count;

#[component]
pub fn NotificationBell(onclick: EventHandler<()>, #[props(default)] class: String) -> Element {
    let count = use_unread_count()();
    let label = if count >= 100 {
        "99+".to_string()
    } else {
        count.to_string()
    };

    rsx! {
        button {
            class: "notification-bell {class}",
            "aria-label": "Notifications",
            "data-testid": "notification-bell",
            onclick: move |_| onclick.call(()),
            // Inline outline bell matching the arena HUD aesthetic: same
            // stroke-width/linecap/linejoin as `home`/`publish`/`start`/etc
            // in `arena_topbar/component.rs`, and `stroke: currentColor`
            // so the existing `.hud-btn svg { color: … }` rule handles
            // default + hover + pressed states identically to its siblings.
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" }
                path { d: "M13.73 21a2 2 0 0 1-3.46 0" }
            }
            if count > 0 {
                span {
                    class: "notification-bell__badge",
                    "data-testid": "notification-bell-badge",
                    "{label}"
                }
            }
        }
    }
}
