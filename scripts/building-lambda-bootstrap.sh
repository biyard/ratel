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

cargo install dioxus-cli

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

dx build --release --debug-symbols false @client --features web --platform web @server --features server,lambda --platform server

cd ../..

chown -R $HOST_UID:$HOST_GID target
chown -R $HOST_UID:$HOST_GID /root/.cargo
chown -R $HOST_UID:$HOST_GID .build
