use crate::common::*;

#[component]
pub fn PremiumSwitch(
    active: bool,
    on_toggle: EventHandler<MouseEvent>,
    label: String,
    #[props(default)] disabled: bool,
) -> Element {
    let membership = crate::features::auth::hooks::use_user_membership();
    let is_paid = membership.as_ref().map_or(false, |m| m.is_paid());

    rsx! {
        if is_paid {
            Switch {
                active,
                on_toggle,
                label,
                disabled,
            }
        } else {
            PremiumUnlockButton {}
        }
    }
}

/// Unlock button for non-paid users. Uses raw button element to match
/// the existing UnlockButton pattern in reward_setting.rs.
#[component]
fn PremiumUnlockButton() -> Element {
    let tr: PremiumUnlockTranslate = use_translate();
    let nav = use_navigator();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let username = user_ctx
        .read()
        .user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();

    rsx! {
        button {
            r#type: "button",
            class: "inline-flex gap-1.5 items-center py-1.5 px-3 text-xs font-semibold rounded-full transition-all bg-primary/10 text-primary hover:bg-primary/20",
            onclick: move |_| {
                let username = username.clone();
                nav.push(format!("/{username}/memberships"));
            },
            icons::security::Lock1 { width: "14", height: "14", class: "[&>path]:stroke-current" }
            {tr.unlock}
        }
    }
}

translate! {
    PremiumUnlockTranslate;

    unlock: {
        en: "Unlock",
        ko: "잠금 해제",
    },
}
