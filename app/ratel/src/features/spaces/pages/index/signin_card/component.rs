use crate::features::auth::LoginModal;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::providers::use_space_context;

#[component]
pub fn SigninCard(
    space_id: ReadSignal<SpacePartition>,
    participants: String,
    remaining: String,
    rewards: String,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let mut popup = use_popup();
    let ctx = use_space_context();

    rsx! {
        div { class: "participate-card", "data-testid": "card-signin",
            span { class: "participate-card__heading", "{tr.welcome_heading}" }
            p { class: "participate-card__desc", "{tr.welcome_desc}" }
            div { class: "participate-card__stats",
                div { class: "stat",
                    span { class: "stat__value", "{participants}" }
                    span { class: "stat__label", "{tr.participants}" }
                }
                if ctx.space().quota > 0 {
                    div { class: "stat",
                        span { class: "stat__value", "{remaining}" }
                        span { class: "stat__label", "{tr.remaining}" }
                    }
                }
                div { class: "stat",
                    span { class: "stat__value", "{rewards}" }
                    span { class: "stat__label", "{tr.rewards}" }
                }
            }
            button {
                class: "cta-signin",
                "data-testid": "btn-signin",
                onclick: move |_| {
                    let mut ctx = ctx;
                    let cb = Callback::new(move |_| {
                        ctx.restart();
                    });
                    popup.open(rsx! {
                        LoginModal { on_success: cb }
                    }).with_title(tr.login_title);
                },
                "{tr.sign_in}"
            }
        }
    }
}
