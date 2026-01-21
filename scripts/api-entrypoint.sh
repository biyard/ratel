#! /bin/bash

apt update && apt install -y curl jq build-essential cmake pkg-config libssl-dev
curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_co
nvm install --lts
nvm use --lts
npm i -g npm@latest
npm i -g pnpm

cd packages/main-api
cargo install cargo-binstall
cargo binstall --no-confirm cargo-watch

make run
