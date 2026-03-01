use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR missing");
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

    fs::create_dir_all(&assets_dir).expect("failed to create assets directory");
    fs::copy(
        js_dir.join("dist/main.js"),
        assets_dir.join("space-incentive-pool.js"),
    )
    .expect("failed to copy dist/main.js to assets/space-incentive-pool.js");

    let repo_root = manifest_path
        .ancestors()
        .nth(4)
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_path.to_path_buf());
    let artifact_src =
        repo_root.join("ts-packages/web/src/contracts/artifacts/SpaceIncentive.json");
    let artifact_dst = assets_dir.join("space-incentive-artifact.json");

    if artifact_src.exists() {
        fs::copy(&artifact_src, &artifact_dst).unwrap_or_else(|err| {
            panic!(
                "failed to copy artifact from {} to {}: {err}",
                artifact_src.display(),
                artifact_dst.display()
            )
        });
    } else {
        println!(
            "cargo:warning=SpaceIncentive artifact not found at {}",
            artifact_src.display()
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", js_dir.join("src").display());
    println!(
        "cargo:rerun-if-changed={}",
        js_dir.join("package.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        js_dir.join("package-lock.json").display()
    );
    println!("cargo:rerun-if-changed={}", artifact_src.display());
}
