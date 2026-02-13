use crate::{
    controllers::user::{get_user, login, LoginRequest},
    *,
};
use dioxus::fullstack::Form;
use dioxus::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[component]
pub fn LoginTest() -> Element {
    let mut user = use_loader(move || get_user())?;

    let mut login = use_action(login);

    let mut email = use_signal(|| String::default());
    let mut password = use_signal(|| String::default());

    let submit_client = move |_evt: MouseEvent| async move {
        // Write the client
        login
            .call(Form(LoginRequest {
                email: email.read().clone(),
                password: password.read().clone(),
            }))
            .await;

        user.restart();
    };

    rsx! {
        div { class: "flex flex-col w-full",
            input {
                class: "border border-white",
                r#type: "text",
                id: "email",
                name: "email",
                oninput: move |ev| {
                    email.set(ev.data().value());
                },
            }
            label { "email" }
            input {
                class: "border border-white",
                r#type: "password",
                id: "password",
                name: "password",
                oninput: move |ev| {
                    password.set(ev.data().value());
                },
            }
            label { "password" }
            button {
                onclick: submit_client,
                class: "border border-white bg-primary py-5 px-10",
                "Login"
            }
        }

        {
            if let Some(user) = user.read().as_ref() {
                rsx! {
                    div { "{user.display_name}" }
                }
            } else {
                rsx! {
                    div { "유저를 찾을 수 없습니다" }
                }
            }
        }
    }
}
