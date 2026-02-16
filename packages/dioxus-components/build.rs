use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let workspace_root = manifest_path.parent().unwrap().parent().unwrap();
    let assets_dir = manifest_path.join("assets");

    // No cargo:rerun-if-changed directives: Cargo reruns this build script
    // on every build by default when none are specified.
    let status = Command::new("pnpm")
        .args(["--filter", "@ratel/components", "build"])
        .env("ASSETS_DIR", &assets_dir)
        .current_dir(workspace_root)
        .status()
        .expect("failed to run pnpm build");

    assert!(status.success(), "pnpm build for @ratel/components failed");
}
