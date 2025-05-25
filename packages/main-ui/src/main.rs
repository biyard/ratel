pub mod components;
pub mod config;
pub mod dto;
pub mod pages;
pub mod route;
pub mod services;
pub mod theme;
pub mod utils;

use bdk::prelude::*;
use by_components::{effects::HoverEffects, responsive::Responsive};
use dioxus_oauth::prelude::FirebaseProvider;
use dioxus_popup::PopupService;
use route::Route;

#[cfg(feature = "web")]
use services::anonymouse_service::*;
use services::{user_service::UserService, vote_service::VoteService};
use theme::Theme;

#[cfg(target_os = "ios")]
fn redirect_logs_to_file() {
    use std::fs::OpenOptions;
    use std::os::unix::io::IntoRawFd;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/tmp/ios_rust_log.txt")
        .unwrap();

    let fd = file.into_raw_fd();
    unsafe {
        libc::dup2(fd, libc::STDOUT_FILENO);
        libc::dup2(fd, libc::STDERR_FILENO);
    }
}

fn main() {
    let conf = config::get();
    #[cfg(target_os = "ios")]
    redirect_logs_to_file();
    dioxus_logger::init(conf.log_level).expect("failed to init logger");
    tracing::debug!("config: {:?}", conf);
    rest_api::set_message(conf.domain.to_string());

    #[cfg(feature = "mobile")]
    {
        dioxus_aws::launch(app);
    }

    #[cfg(feature = "web")]
    {
        dioxus_aws::launch(app);
    }

    #[cfg(feature = "server")]
    {
        let app = dioxus_aws::new(app).route(
            "/sitemap.xml",
            dioxus_aws::axum::routing::get(api::sitemap_xml),
        );
        dioxus_aws::serve(app);
    }
}

#[allow(dead_code)]
fn app() -> Element {
    Theme::init();
    #[cfg(feature = "web")]
    AnonymouseService::init();
    UserService::init();
    PopupService::init();
    VoteService::init();
    let conf = config::get();

    let css = include_str!("../public/theme.css");
    let custom = include_str!("../public/custom.css");

    rsx! {
        btracing::ToastTracing {}
        HoverEffects {}
        FirebaseProvider {
            api_key: conf.firebase.api_key.clone(),
            auth_domain: conf.firebase.auth_domain.clone(),
            project_id: conf.firebase.project_id.clone(),
            storage_bucket: conf.firebase.storage_bucket.clone(),
            messaging_sender_id: conf.firebase.messaging_sender_id.clone(),
            app_id: conf.firebase.app_id.clone(),
            measurement_id: conf.firebase.measurement_id.clone(),
        }
        document::Link {
            href: asset!("/public/logos/favicon-96x96.png"),
            r#type: "image/png",
            rel: "icon",
            sizes: "96x96",
        }
        document::Link {
            href: asset!("/public/logos/favicon.svg"),
            r#type: "image/svg+xml",
            rel: "icon",
        }
        document::Link { href: asset!("/public/logos/favicon.ico"), rel: "shortcut icon" }
        document::Link {
            href: asset!("/public/logos/apple-touch-icon.png"),
            rel: "apple-touch-icon",
            sizes: "180x180",
        }

        document::Link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        document::Link {
            crossorigin: "false",
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
        }
        document::Style { href: "https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Raleway:ital,wght@0,100..900;1,100..900&display=swap" }
        document::Style { href: asset!("/public/main.css") }
        document::Style { href: asset!("/public/tailwind.css") }

        document::Script { src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" }
        document::Script {
            src: "https://unpkg.com/@dotlottie/player-component@2.7.12/dist/dotlottie-player.mjs",
            r#type: "module",
        }
        document::Script { src: "https://d3js.org/d3.v7.min.js" }
        document::Style { r#type: "text/tailwindcss", "{css} {custom}" }

        // document::Script { r#type: "module", src: asset!("/public/dep.js"), defer: true }

        Responsive { desktop: 1200.0, tablet: 900.0, Router::<Route> {} }
    }
}

#[cfg(feature = "server")]
mod api {
    use bdk::prelude::*;
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

    pub async fn sitemap_xml() -> dioxus_aws::axum::response::Response {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(option_env!("DATABASE_URL").unwrap_or("postgres://localhost:5432/ratel"))
            .await
            .unwrap();

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

        let changefreq = "daily";
        let priority = 1.0;

        for lang in ["en"] {
            xml.push_str(&format!(
            "<url><loc>https://ratel.foundation/{lang}</loc><lastmod>{today}</lastmod><changefreq>{changefreq}</changefreq><priority>{priority}</priority></url>",
        ));
        }

        let priority = 0.8;

        let assembly_members = dto::AssemblyMember::query_builder()
            .query()
            .map(dto::AssemblyMember::from)
            .fetch_all(&pool)
            .await
            .unwrap();
        let ids = assembly_members.iter().map(|m| m.id).collect::<Vec<_>>();

        for lang in ["en"] {
            for id in ids.iter() {
                xml.push_str(&format!(
                    "<url><loc>https://ratel.foundation/{lang}/politicians/{id}</loc><lastmod>{today}</lastmod><changefreq>{changefreq}</changefreq><priority>{priority}</priority></url>",
        ));
            }
        }

        let changefreq = "monthly";

        for lang in ["en"] {
            xml.push_str(&format!(
                "<url><loc>https://ratel.foundation/{lang}/politicians</loc><lastmod>{today}</lastmod><changefreq>{changefreq}</changefreq><priority>{priority}</priority></url>",
        ));
        }

        let priority = 0.3;

        for lang in ["en"] {
            xml.push_str(&format!(
                "<url><loc>https://ratel.foundation/{lang}/privacy-policy</loc><lastmod>{today}</lastmod><changefreq>{changefreq}</changefreq><priority>{priority}</priority></url>",
        ));
        }

        xml.push_str("</urlset>");

        dioxus_aws::axum::response::IntoResponse::into_response((
            by_axum::axum::http::StatusCode::OK,
            [("Content-Type", "application/xml")],
            xml,
        ))
    }
}
