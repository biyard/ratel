use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let js_dir = manifest_path.join("js");
    let assets_dir = manifest_path.join("assets");

    let status = Command::new("npm")
        .args(["install"])
        .current_dir(&js_dir)
        .status()
        .expect("failed to run npm install");
    assert!(status.success(), "npm install for ratel/common failed");

    let build_cmd = match std::env::var("ENV").as_deref() {
        Ok("dev") | Ok("local") => "build-dev",
        _ => "build",
    };

    let status = Command::new("npm")
        .args(["run", build_cmd])
        .current_dir(&js_dir)
        .status()
        .expect("failed to run npm build");
    assert!(status.success(), "npm build for ratel/common failed");

    std::fs::create_dir_all(&assets_dir).expect("failed to create assets directory");

    // Copy the main entry bundle.
    std::fs::copy(
        js_dir.join("dist/main.js"),
        assets_dir.join("ratel-app-shell.js"),
    )
    .expect("failed to copy dist/main.js to assets/ratel-app-shell.js");

    // Copy any chunk files produced by webpack code splitting.
    let dist_dir = js_dir.join("dist");
    if let Ok(entries) = std::fs::read_dir(&dist_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            // Copy chunk files (ratel-chunk-*.js) alongside main bundle.
            if name_str.starts_with("ratel-chunk-") && name_str.ends_with(".js") {
                let dest = assets_dir.join(&*name_str);
                std::fs::copy(entry.path(), &dest).unwrap_or_else(|e| {
                    panic!("failed to copy chunk {} to assets: {}", name_str, e)
                });
            }
        }
    }

    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rerun-if-changed={}", js_dir.join("src").display());
}
