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
    assert!(status.success(), "npm install failed");

    let build_cmd = match std::env::var("ENV").as_deref() {
        Ok("dev") | Ok("local") => "build-dev",
        _ => "build",
    };

    let status = Command::new("npm")
        .args(["run", build_cmd])
        .current_dir(&js_dir)
        .status()
        .expect("failed to run npm build");
    assert!(status.success(), "npm build failed");

    std::fs::create_dir_all(&assets_dir).expect("failed to create assets directory");
    std::fs::copy(
        js_dir.join("dist/main.js"),
        assets_dir.join("ratel-my-follower.js"),
    )
    .expect("failed to copy dist/main.js to assets/ratel-my-follower.js");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", js_dir.join("src").display());
}
