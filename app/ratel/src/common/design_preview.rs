use crate::axum::{
    extract::Path as AxumPath,
    http::StatusCode,
    native_routing::get,
    response::{Html, IntoResponse},
    AxumRouter,
};
use std::path::Path;

/// Merge design preview routes into the app router.
/// - `GET /designs` — lists all `.html` files in the design directory
/// - `GET /designs/:file` — serves the raw HTML file
pub fn merge_design_routes(app: AxumRouter, design_dir: &Path) -> AxumRouter {
    let listing_dir = design_dir.to_path_buf();
    let serve_dir = design_dir.to_path_buf();

    let design_router = AxumRouter::new()
        .route(
            "/",
            get(move || async move { list_designs(&listing_dir) }),
        )
        .route(
            "/{file}",
            get(move |AxumPath(file): AxumPath<String>| async move {
                serve_design_file(&serve_dir, &file).await
            }),
        );

    tracing::info!("Design preview available at /designs");
    app.nest("/designs", design_router)
}

fn list_designs(dir: &Path) -> Html<String> {
    let mut entries: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".html") {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    entries.sort();

    let links: String = entries
        .iter()
        .map(|name| format!(r#"<li><a href="/designs/{name}">{name}</a></li>"#))
        .collect::<Vec<_>>()
        .join("\n        ");

    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Design Previews</title>
<style>
  body {{ font-family: system-ui, sans-serif; max-width: 600px; margin: 40px auto; padding: 0 20px; background: #0c0c1a; color: #f0f0f5; }}
  h1 {{ font-size: 24px; margin-bottom: 24px; }}
  ul {{ list-style: none; padding: 0; }}
  li {{ margin-bottom: 8px; }}
  a {{ color: #22d3ee; text-decoration: none; padding: 8px 12px; display: inline-block; border: 1px solid rgba(34,211,238,0.15); border-radius: 8px; transition: all 0.2s; }}
  a:hover {{ background: rgba(34,211,238,0.06); border-color: rgba(34,211,238,0.3); }}
</style>
</head>
<body>
  <h1>Design Previews ({count})</h1>
  <ul>
    {links}
  </ul>
</body>
</html>"#,
        count = entries.len(),
    ))
}

async fn serve_design_file(dir: &std::path::PathBuf, file: &str) -> impl IntoResponse {
    if file.contains("..") || file.contains('/') || file.contains('\\') {
        return (StatusCode::BAD_REQUEST, Html("Invalid file name".to_string()));
    }

    let path = dir.join(file);
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => (StatusCode::OK, Html(content)),
        Err(_) => (StatusCode::NOT_FOUND, Html("File not found".to_string())),
    }
}
