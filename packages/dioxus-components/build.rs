use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let workspace_root = manifest_path.parent().unwrap().parent().unwrap();
    let assets_dir = manifest_path.join("assets");

    // Always rerun: TS sources are outside the Cargo package so directory-level
    // tracking doesn't catch file edits inside subdirectories.
    println!("cargo:rerun-if-changed=build.rs");

    // Run pnpm build from workspace root so hoisted binaries (tsx) are resolved
    let status = Command::new("pnpm")
        .args(["--filter", "@ratel/components", "build"])
        .env("ASSETS_DIR", &assets_dir)
        .current_dir(workspace_root)
        .status()
        .expect("failed to run pnpm build");

    assert!(status.success(), "pnpm build for @ratel/components failed");
}
