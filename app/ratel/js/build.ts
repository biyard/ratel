import { build } from "vite";
import { getConfigs } from "./vite.config.ts";

for (const config of getConfigs()) {
  await build(config);
}
