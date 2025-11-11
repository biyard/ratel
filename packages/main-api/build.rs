use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

fn newest_match(pattern: &str) -> Option<PathBuf> {
    let mut best: Option<(SystemTime, PathBuf)> = None;
    for entry in glob::glob(pattern).ok()? {
        let path = entry.ok()?;
        let meta = fs::metadata(&path).ok()?;
        let mtime = meta.modified().ok()?;
        match &best {
            None => best = Some((mtime, path)),
            Some((best_time, _)) if mtime > *best_time => best = Some((mtime, path)),
            _ => {}
        }
    }
    best.map(|(_, p)| p)
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dist_dst = manifest_dir.join("dist");
    let assets_dir = dist_dst.join("assets");

    let web_build = env::var("WEB_BUILD").unwrap_or_default();
    let skip_web_build = env::var("CARGO_FEATURE_MODEL_ONLY").is_ok() || web_build == "ignore";
    if skip_web_build {
        println!("cargo:warning=Skipping web build because feature `model-only` is enabled");
        return;
    }
    if web_build != "false" || !fs::exists(&dist_dst).unwrap_or_default() {
        println!("cargo:rerun-if-env-changed=ENV");

        let workspace_root = manifest_dir
            .parent()
            .and_then(|p| p.parent())
            .unwrap_or(&manifest_dir);
        let web_dir = workspace_root.join("ts-packages/web");

        let status = Command::new("make")
            .arg("build")
            .current_dir(&web_dir)
            .status()
            .expect("failed to run `make build` for web");
        if !status.success() {
            panic!("web build failed with status: {status}");
        }

        let dist_src = web_dir.join("dist");
        let _ = fs::remove_dir_all(&dist_dst);
        copy_dir_all(&dist_src, &dist_dst).expect("failed to copy dist/");

        println!("cargo:rerun-if-changed={}", web_dir.display());
        println!("cargo:rerun-if-changed={}", assets_dir.display());
    }
    let css = newest_match(&format!("{}/index-*.css", assets_dir.display()))
        .expect("no index-*.css found");
    let js =
        newest_match(&format!("{}/index-*.js", assets_dir.display())).expect("no index-*.js found");

    println!("cargo:rerun-if-changed={}", css.display());
    println!("cargo:rerun-if-changed={}", js.display());

    println!(
        "cargo:rustc-env=WEB_INDEX_CSS={}",
        css.file_name().unwrap().to_string_lossy()
    );
    println!(
        "cargo:rustc-env=WEB_INDEX_JS={}",
        js.file_name().unwrap().to_string_lossy()
    );
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}
