use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            h1 { "Ratel Auth" }
            p { "Authentication service" }
        }
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        div {
            h1 { "Login" }
            p { "Login page placeholder" }
        }
    }
}

#[component]
fn Signup() -> Element {
    rsx! {
        div {
            h1 { "Signup" }
            p { "Signup page placeholder" }
        }
    }
}
