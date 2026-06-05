#! /bin/bash

# Inside docker run -it -v $(HOME)/.cargo/bin:/root/.cargo/bin -v $(pwd):/app -w /app public.ecr.aws/sam/build-provided.al2023
source app/ratel/env.sh

# install platform 
dnf install -y openssl-devel pkgconf-pkg-config gcc

# install cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source ~/.cargo/env

rustup toolchain install stable
rustup target add wasm32-unknown-unknown

# `--locked` honors the Cargo.lock published with dioxus-cli, avoiding the
# clean-cache `cargo install` fallback that resolves `git2 0.21` and fails
# with E0599 (auth-git2 0.5.8 calls a removed API). Same fix already applied
# to the workflow `cargo binstall dioxus-cli --locked` calls (commit 41c41f228);
# this docker-internal build path was missed, so the lambda build silently
# failed and left `target/dx/app-shell/release/web/public` uncreated.
cargo install dioxus-cli --locked

# Install node.js
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm

nvm install --lts

# Pre-install JS deps before `dx build` to avoid a race condition.
# `app/ratel/build.rs` runs `npm install` in `app/ratel/js/` on every build,
# and Cargo runs it twice in parallel (once per target: client(wasm) + server(lambda))
# because `app-shell` is built for two targets in a single `dx build` invocation.
# When both processes start with an empty `node_modules`, they clobber each other
# (e.g. ENOTEMPTY on rmdir of partially-written package dirs, missing files like
# schema-utils/dist/index.js). Pre-installing here leaves `node_modules` in a
# lock-consistent state, so the two parallel installs from build.rs find nothing
# to do and exit cleanly.
npm i
npm install --prefix app/ratel/js

cd app/ratel

# Web wasm: `web,fullstack` is the only correct feature set for the
# regular SSR + hydrated dev/prod build. See app/ratel/Cargo.toml [features]
# comment — `fullstack` is intentionally excluded from `web` so that the
# Tauri Android shell's `--features tauri-web --platform web --fullstack false`
# build (driven by `app/ratel-tauri/Makefile`'s dx-build target) doesn't
# transitively enable `dioxus-web/hydrate`. Regular web builds (this script)
# must add `fullstack` explicitly. Without it, the by-macros `#[get]/#[post]`
# expansion's `cfg(not(tauri-web))` arm collapses to a non-functional stub,
# which is what made dev wasm fall back to baking `MOBILE_API_URL` (the CI
# runner's LAN IP) into reqwest calls and trip mixed-content errors.
#
# `--no-default-features` mirrors every other build target in app/ratel/Makefile
# (build, build-testing, build-arm, build-tauri). Without it, the default
# `mobile` feature bleeds reqwest + dioxus/mobile code paths into the wasm
# client bundle.
dx build --release --no-default-features --debug-symbols false @client --features web,fullstack --platform web @server --features server,lambda --platform server

cd ../..

chown -R $HOST_UID:$HOST_GID target
chown -R $HOST_UID:$HOST_GID /root/.cargo
