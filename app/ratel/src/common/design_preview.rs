use crate::axum::{
    extract::Path as AxumPath,
    http::StatusCode,
    native_routing::get,
    response::{Html, IntoResponse, Response},
    AxumRouter,
};
use std::path::{Path, PathBuf};

/// Merge design preview routes into the app router.
/// - `GET /designs`             — lists top-level directories + root HTML files
/// - `GET /designs/{*path}`     — directory listing or HTML file serving
pub fn merge_design_routes(app: AxumRouter, design_dir: &Path) -> AxumRouter {
    let root = design_dir.to_path_buf();
    let root_index = root.clone();
    let root_catch = root.clone();

    let design_router = AxumRouter::new()
        .route(
            "/",
            get(move || {
                let root = root_index.clone();
                async move { list_path(&root, &root).into_response() }
            }),
        )
        .route(
            "/{*path}",
            get(move |AxumPath(path): AxumPath<String>| {
                let root = root_catch.clone();
                async move { resolve_path(&root, &path).await }
            }),
        );

    tracing::info!("Design preview available at /designs");
    app.nest("/designs", design_router)
}

async fn resolve_path(root: &Path, relative: &str) -> Response {
    if relative.contains("..") || relative.starts_with('/') {
        return (StatusCode::BAD_REQUEST, Html("Invalid path".to_string())).into_response();
    }

    let target = root.join(relative);

    let canonical_root = match std::fs::canonicalize(root) {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Design root not found".to_string()),
            )
                .into_response()
        }
    };
    let canonical_target = match std::fs::canonicalize(&target) {
        Ok(p) => p,
        Err(_) => return (StatusCode::NOT_FOUND, Html("Not found".to_string())).into_response(),
    };
    if !canonical_target.starts_with(&canonical_root) {
        return (StatusCode::BAD_REQUEST, Html("Invalid path".to_string())).into_response();
    }

    if canonical_target.is_dir() {
        list_path(&canonical_target, &canonical_root).into_response()
    } else if canonical_target.extension().and_then(|e| e.to_str()) == Some("html") {
        match tokio::fs::read_to_string(&canonical_target).await {
            Ok(content) => (StatusCode::OK, Html(content)).into_response(),
            Err(_) => {
                (StatusCode::NOT_FOUND, Html("File not found".to_string())).into_response()
            }
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Html("Only HTML files are served".to_string()),
        )
            .into_response()
    }
}

fn list_path(dir: &Path, root: &Path) -> Html<String> {
    let mut dirs: Vec<(String, usize)> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();
            if path.is_dir() {
                let count = count_html_recursive(&path);
                dirs.push((name, count));
            } else if name.ends_with(".html") {
                files.push(name);
            }
        }
    }
    dirs.sort_by(|a, b| a.0.cmp(&b.0));
    files.sort();

    let relative = dir.strip_prefix(root).unwrap_or(Path::new(""));
    let relative_str = relative.to_string_lossy().to_string();
    let base_url = if relative_str.is_empty() {
        "/designs".to_string()
    } else {
        format!("/designs/{}", relative_str)
    };

    let breadcrumbs = build_breadcrumbs(&relative_str);

    let dir_items: String = dirs
        .iter()
        .map(|(name, count)| {
            let href = format!("{}/{}", base_url.trim_end_matches('/'), name);
            let plural = if *count == 1 { "" } else { "s" };
            format!(
                r#"<a class="card card--dir" href="{href}">
  <span class="card__icon">
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
  </span>
  <span class="card__body">
    <span class="card__name">{name}</span>
    <span class="card__sub">{count} file{plural}</span>
  </span>
  <svg class="card__chev" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
</a>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let file_items: String = files
        .iter()
        .map(|name| {
            let href = format!("{}/{}", base_url.trim_end_matches('/'), name);
            let label = name.trim_end_matches(".html");
            format!(
                r#"<a class="card card--file" href="{href}">
  <span class="card__icon card__icon--file">
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>
  </span>
  <span class="card__body">
    <span class="card__name">{label}</span>
    <span class="card__sub">{name}</span>
  </span>
  <svg class="card__chev" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"/></svg>
</a>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let sections = {
        let mut out = String::new();
        if !dirs.is_empty() {
            out.push_str(&format!(
                r#"<section class="group">
<h2 class="group__title">Directories <span class="group__count">{}</span></h2>
<div class="grid">{}</div>
</section>"#,
                dirs.len(),
                dir_items
            ));
        }
        if !files.is_empty() {
            out.push_str(&format!(
                r#"<section class="group">
<h2 class="group__title">Files <span class="group__count">{}</span></h2>
<div class="grid">{}</div>
</section>"#,
                files.len(),
                file_items
            ));
        }
        if dirs.is_empty() && files.is_empty() {
            out.push_str(
                r#"<section class="group"><p class="empty">No HTML files or directories here yet.</p></section>"#,
            );
        }
        out
    };

    let title = if relative_str.is_empty() {
        "Design Previews".to_string()
    } else {
        format!("/{}", relative_str)
    };

    let summary = format!(
        "{} director{} · {} file{}",
        dirs.len(),
        if dirs.len() == 1 { "y" } else { "ies" },
        files.len(),
        if files.len() == 1 { "" } else { "s" },
    );

    Html(format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title} — Ratel Designs</title>
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link href="https://fonts.googleapis.com/css2?family=Orbitron:wght@500;600;700&family=Outfit:wght@300;400;500;600;700&display=swap" rel="stylesheet" />
<style>
*,*::before,*::after{{margin:0;padding:0;box-sizing:border-box}}
:root{{
  --bg-void:#06060e;--bg-glass:rgba(12,12,26,0.65);--bg-glass-hover:rgba(20,20,40,0.80);
  --border-subtle:rgba(255,255,255,0.06);--border-strong:rgba(255,255,255,0.12);
  --text-primary:#f0f0f5;--text-muted:#8888a8;--text-dim:#55556a;
  --accent-gold:#fcb300;--accent-teal:#6eedd8;
  --font-display:'Orbitron',sans-serif;--font-body:'Outfit',sans-serif;
}}
html,body{{min-height:100%;background:var(--bg-void);color:var(--text-primary);font-family:var(--font-body);font-size:14px;line-height:1.5}}
body{{padding:40px 24px 80px;overflow-x:hidden}}
body::before{{content:'';position:fixed;inset:0;background:radial-gradient(ellipse 80% 60% at 30% 15%,rgba(252,179,0,0.05) 0%,transparent 55%),radial-gradient(ellipse 60% 60% at 80% 85%,rgba(110,237,216,0.04) 0%,transparent 60%);z-index:-1;pointer-events:none}}

.page{{max-width:920px;margin:0 auto}}
.header{{display:flex;flex-direction:column;gap:10px;padding-bottom:22px;border-bottom:1px solid var(--border-subtle);margin-bottom:28px}}
.eyebrow{{font-family:var(--font-display);font-size:10px;font-weight:600;letter-spacing:0.18em;text-transform:uppercase;color:var(--text-dim)}}
.eyebrow a{{color:var(--text-muted);text-decoration:none}}
.eyebrow a:hover{{color:var(--accent-gold)}}
.eyebrow-sep{{margin:0 8px;color:var(--text-dim)}}
.title{{font-family:var(--font-display);font-size:26px;font-weight:700;letter-spacing:0.03em;color:var(--text-primary)}}
.title strong{{background:linear-gradient(135deg,var(--accent-gold),var(--accent-teal));-webkit-background-clip:text;background-clip:text;-webkit-text-fill-color:transparent;font-weight:700}}
.sub{{font-size:13px;color:var(--text-muted);letter-spacing:0.02em}}

.group{{display:flex;flex-direction:column;gap:14px;margin-bottom:32px}}
.group__title{{font-family:var(--font-display);font-size:11px;font-weight:700;letter-spacing:0.16em;text-transform:uppercase;color:var(--text-muted);display:flex;align-items:center;gap:10px}}
.group__title::before{{content:'';width:4px;height:14px;border-radius:2px;background:linear-gradient(180deg,var(--accent-gold),transparent)}}
.group__count{{font-family:var(--font-display);font-size:10px;font-weight:600;color:var(--text-dim);padding:2px 8px;border-radius:100px;background:rgba(255,255,255,0.04);border:1px solid var(--border-subtle)}}

.grid{{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:12px}}

.card{{display:flex;align-items:center;gap:12px;padding:14px 16px;border-radius:12px;background:var(--bg-glass);backdrop-filter:blur(16px);border:1px solid var(--border-subtle);color:var(--text-primary);text-decoration:none;transition:all 0.2s ease}}
.card:hover{{border-color:rgba(252,179,0,0.25);background:var(--bg-glass-hover);transform:translateY(-1px)}}
.card__icon{{width:38px;height:38px;border-radius:10px;background:rgba(252,179,0,0.08);border:1px solid rgba(252,179,0,0.18);display:flex;align-items:center;justify-content:center;color:var(--accent-gold);flex-shrink:0}}
.card__icon--file{{background:rgba(110,237,216,0.06);border-color:rgba(110,237,216,0.18);color:var(--accent-teal)}}
.card__body{{flex:1;min-width:0;display:flex;flex-direction:column;gap:2px}}
.card__name{{font-weight:600;font-size:14px;color:var(--text-primary);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}}
.card__sub{{font-family:var(--font-display);font-size:9.5px;font-weight:600;letter-spacing:0.12em;text-transform:uppercase;color:var(--text-dim)}}
.card__chev{{color:var(--text-dim);transition:all 0.2s}}
.card:hover .card__chev{{color:var(--accent-gold);transform:translateX(2px)}}

.empty{{color:var(--text-dim);font-style:italic;padding:24px;text-align:center;border-radius:12px;border:1px dashed var(--border-strong);background:rgba(255,255,255,0.01)}}
</style>
</head>
<body>
  <div class="page">
    <header class="header">
      <div class="eyebrow">{breadcrumbs}</div>
      <h1 class="title"><strong>{title}</strong></h1>
      <div class="sub">{summary}</div>
    </header>
    {sections}
  </div>
</body>
</html>"##,
    ))
}

fn build_breadcrumbs(relative: &str) -> String {
    let mut parts = vec![r#"<a href="/designs">designs</a>"#.to_string()];
    if !relative.is_empty() {
        let mut acc = PathBuf::from("/designs");
        for segment in relative.split('/').filter(|s| !s.is_empty()) {
            acc.push(segment);
            parts.push(format!(
                r#"<a href="{}">{}</a>"#,
                acc.display(),
                segment
            ));
        }
    }
    parts.join(r#"<span class="eyebrow-sep">/</span>"#)
}

fn count_html_recursive(dir: &Path) -> usize {
    let mut count = 0;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                count += count_html_recursive(&path);
            } else if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "html")
                .unwrap_or(false)
            {
                count += 1;
            }
        }
    }
    count
}
