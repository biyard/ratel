import { readdirSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";
import type { InlineConfig } from "vite";

const __dirname = dirname(fileURLToPath(import.meta.url));

const ASSETS_DIR = process.env.ASSETS_DIR;

if (!ASSETS_DIR) {
  throw new Error("ASSETS_DIR is not defined");
}

const components = readdirSync(resolve(__dirname, "src"), {
  withFileTypes: true,
})
  .filter((d) => d.isDirectory())
  .map((d) => d.name);

export function getConfigs(): InlineConfig[] {
  return components.map((name) => ({
    configFile: false,
    build: {
      lib: {
        entry: resolve(__dirname, `src/${name}/index.ts`),
        name: name.replace(/-./g, (m) => m[1].toUpperCase()),
        formats: ["iife"] as const,
        fileName: () => `${name}.js`,
      },
      outDir: ASSETS_DIR,
      emptyOutDir: false,
      minify: "esbuild" as const,
    },
  }));
}
