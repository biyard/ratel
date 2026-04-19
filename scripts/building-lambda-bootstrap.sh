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

npm i

cd app/ratel

dx build --release --debug-symbols false @client --features web --platform web @server --features server,lambda --platform server
