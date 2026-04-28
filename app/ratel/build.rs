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

    let status = Command::new("npm")
        .args(["run", "build"])
        .env("ASSETS_DIR", &assets_dir)
        .current_dir(&js_dir)
        .status()
        .expect("failed to run npm build");
    assert!(status.success(), "npm build for ratel/common failed");

    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rerun-if-changed={}", js_dir.join("src").display());
}
