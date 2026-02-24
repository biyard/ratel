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
    std::fs::copy(
        js_dir.join("dist/main.js"),
        assets_dir.join("ratel-app-shell.js"),
    )
    .expect("failed to copy dist/main.js to assets/ratel-app-shell.js");

    let setting_asset_src = manifest_path
        .join("../socials/users/pages/setting/assets/ratel-user-setting.js");
    let setting_asset_dst = assets_dir.join("ratel-user-setting.js");
    if setting_asset_src.exists() {
        std::fs::copy(&setting_asset_src, &setting_asset_dst)
            .expect("failed to copy ratel-user-setting.js into app-shell assets");
        println!("cargo:rerun-if-changed={}", setting_asset_src.display());
    } else {
        println!(
            "cargo:warning=ratel-user-setting.js not found at {}",
            setting_asset_src.display()
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", js_dir.join("src").display());
}
