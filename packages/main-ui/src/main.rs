pub mod components;
pub mod config;
pub mod pages;
pub mod route;
pub mod services;
pub mod theme;

use dioxus::prelude::*;
use dioxus_popup::PopupService;
use route::Route;
use services::{politician_service::PoliticianService, user_service::UserService};
use theme::Theme;

fn main() {
    let conf = config::get();
    dioxus_logger::init(conf.log_level).expect("failed to init logger");
    tracing::debug!("config: {:?}", conf);
    rest_api::set_message(conf.domain.to_string());

    dioxus_aws::launch(app);
}

fn app() -> Element {
    Theme::init();
    UserService::init();
    PopupService::init();
    PoliticianService::init();

    rsx! {
        document::Title { "DemocraSee" }
        document::Meta {
            name: "title",
            content: "Driving Better Crypto Policies for South Korea's Ecosystem",
        }
        document::Meta {
            name: "description",
            content: "A platform empowering South Korea's crypto ecosystem by encouraging politicians to create supportive policies. Track their crypto stances, vote on legislation, propose DAO-driven improvements, and shape a thriving future for the industry and democracy.",
        }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }
        document::Link { id: "favicon", rel: "icon", href: asset!("/public/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/public/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.1/css/all.min.css",
        }
        document::Script { src: "https://cdn.tailwindcss.com/3.4.16" }

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
