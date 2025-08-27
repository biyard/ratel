#! /bin/bash

corepack enable
corepack prepare pnpm@10.15.0 --activate
pnpm -w install
pnpm -C ts-packages/web dev
