pub mod home_page;

use crate::AppState;
use crate::error::Error;
use axum::Router;
use axum::native_routing::get_service;
use bdk::prelude::*;
use home_page::home_page_handler;
use tower_http::services::ServeDir;

pub fn route(app_state: AppState) -> Result<Router, Error> {
    Ok(Router::new()
        .native_route("/", axum::native_routing::get(home_page_handler))
        .fallback(home_page_handler)
        .with_state(app_state)
        .nest_service("/assets", get_service(ServeDir::new("dist/assets")))
        .nest_service(
            "/favicon.ico",
            get_service(ServeDir::new("dist/favicon.ico")),
        )
        .nest_service("/images", get_service(ServeDir::new("dist/images")))
        .nest_service("/animations", get_service(ServeDir::new("dist/animations")))
        .nest_service("/documents", get_service(ServeDir::new("dist/documents")))
        .nest_service("/logos", get_service(ServeDir::new("dist/logos")))
        .nest_service("/sounds", get_service(ServeDir::new("dist/sounds")))
        .nest_service("/videos", get_service(ServeDir::new("dist/videos")))
        .nest_service(
            "/tailwind.css",
            get_service(ServeDir::new("dist/tailwind.css")),
        )
        .nest_service("/main.css", get_service(ServeDir::new("dist/main.css"))))
}
