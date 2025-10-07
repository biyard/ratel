#! /bin/bash

corepack enable
corepack prepare pnpm@10.15.0 --activate
pnpm install
pnpm -C ts-packages/www dev
