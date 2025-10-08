// backend-crate/build.rs
use std::{env, path::PathBuf, process::Command};

fn main() {
    if env::var("WEB_BUILD").unwrap_or_else(|_| "false".into()) != "true" {
        println!("cargo:warning=Skipping web build (WEB_BUILD!=1)");
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap_or(&manifest_dir);
    let web_dir = workspace_root.join("ts-packages/web");

    let env_name = env::var("ENV").unwrap_or_else(|_| "dev".into());

    let status = Command::new("make")
        .arg("build")
        .current_dir(&web_dir)
        .env("ENV", &env_name)
        .status()
        .expect("failed to spawn `make build` for web");

    if !status.success() {
        panic!("web build failed with status: {status}");
    }

    println!("cargo:rerun-if-changed={}", web_dir.display());
}
