pub mod config;
pub mod layouts;
pub mod pages;
pub mod route;

use dioxus::prelude::*;
use route::Route;

fn main() {
    let conf = config::get();
    dioxus_logger::init(conf.log_level).expect("failed to init logger");

    dioxus_aws::launch(app);
}

fn app() -> Element {
    rsx! {
        head {
            title {""}
            meta {
                name: "description",
                content: ""
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1.0"
            }
            link {
                id: "favicon",
                rel: "icon",
                href: asset!("/public/favicon.ico"),
            }
            link {
                rel: "stylesheet",
                href: asset!("/public/main.css"),
            }
            link {
                rel: "stylesheet",
                href: asset!("/public/tailwind.css")
            }
            script {
                src: "https://cdn.tailwindcss.com/3.4.16",
            }
            link {
                rel: "stylesheet",
                href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.1/css/all.min.css",
            }
        }

        Router::<Route> {}
    }
}

#[cfg(feature = "server")]
mod api {
    use dioxus::fullstack::prelude::*;
    use server_fn::codec::{GetUrl, Json};

    #[server(endpoint = "/version", input=GetUrl, output=Json)]
    pub async fn version() -> Result<String, ServerFnError> {
        Ok(match option_env!("VERSION") {
            Some(version) => match option_env!("COMMIT") {
                Some(commit) => format!("{}-{}", version, commit),
                None => format!("{}", version),
            },
            None => match option_env!("DATE") {
                Some(date) => date.to_string(),
                None => "unknown".to_string(),
            },
        }
        .to_string())
    }
}
