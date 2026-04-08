#!/usr/bin/env node
/**
 * build-tokens.js
 * Reads all token JSON files from ../assets/ and generates:
 *   - ../assets/tokens.css   (CSS custom properties, light + dark)
 *   - token-manifest.json    (flat manifest used by other tooling)
 *
 * Run: node scripts/build-tokens.js
 */

const fs   = require("fs");
const path = require("path");

const VARIANTS_DIR = path.resolve(__dirname, "../assets");
const OUT_CSS      = path.resolve(__dirname, "../assets/tokens.css");
const OUT_JSON     = path.resolve(__dirname, "../assets/token-manifest.json");

// ─── helpers ──────────────────────────────────────────────────────────────────

function readJson(rel) {
  const abs = path.join(VARIANTS_DIR, rel);
  if (!fs.existsSync(abs)) return null;
  return JSON.parse(fs.readFileSync(abs, "utf8"));
}

function hexOrRgba(token) {
  const v = token["$value"];
  if (typeof v === "string") return v; // alias reference like "{generic.primary}"
  if (typeof v === "object" && v.hex) {
    const a = v.alpha !== undefined ? v.alpha : 1;
    if (Math.abs(a - 1) < 0.001) return v.hex;
    // Return rgba()
    const r = Math.round(v.components[0] * 255);
    const g = Math.round(v.components[1] * 255);
    const b = Math.round(v.components[2] * 255);
    return `rgba(${r},${g},${b},${a.toFixed(3)})`;
  }
  return String(v);
}

/**
 * Flatten a nested token object into { "group.subgroup.key": tokenObj }
 * Stops recursing when it hits a "$value" key (leaf node).
 */
function flatten(obj, prefix = "") {
  const out = {};
  for (const [k, v] of Object.entries(obj)) {
    if (k.startsWith("$")) continue;
    if (typeof v === "object" && v !== null && !("$value" in v)) {
      Object.assign(out, flatten(v, prefix ? `${prefix}.${k}` : k));
    } else if (typeof v === "object" && v !== null && "$value" in v) {
      out[prefix ? `${prefix}.${k}` : k] = v;
    }
  }
  return out;
}

/** Convert a token path like "font.body" → "--ratel-color-font-body" */
function colorVar(path) {
  return "--ratel-color-" + path.replace(/\./g, "-").replace(/\s+/g, "-").toLowerCase();
}

/** px value from number token */
function px(n) {
  if (n === 9999) return "9999px";
  return Number.isInteger(n) ? `${n}px` : `${parseFloat(n.toFixed(4))}px`;
}

// ─── Load source files ─────────────────────────────────────────────────────────

const primitiveColor = readJson("primitive/primitive-color.json");
const radius         = readJson("primitive/radius.json");
const stroke         = readJson("primitive/stroke.json");
const tokens         = readJson("primitive/token.json");  // spacing scale
const typography     = readJson("ratel-brand/typography.json");
const lightTokens    = readJson("ratel-brand/color/Light.tokens.json");
const darkTokens     = readJson("ratel-brand/color/Dark.tokens.json");

// ─── Build: radius ─────────────────────────────────────────────────────────────

function buildRadius() {
  const lines = ["  /* === PRIMITIVE: Radius === */"];
  for (const [k, v] of Object.entries(radius)) {
    if (k.startsWith("$")) continue;
    // Only emit the main (non-directional) tokens to avoid duplication
    if (k.startsWith("rounded-") && !k.match(/rounded-[tblrse]/)) {
      const val = typeof v === "object" ? v["$value"] : v;
      lines.push(`  --ratel-radius-${k.replace("rounded-", "")}: ${px(val)};`);
    }
  }
  return lines.join("\n");
}

// ─── Build: stroke ─────────────────────────────────────────────────────────────

function buildStroke() {
  const lines = ["  /* === PRIMITIVE: Stroke === */"];
  for (const [k, v] of Object.entries(stroke)) {
    if (k.startsWith("$")) continue;
    const val  = typeof v === "object" ? v["$value"] : v;
    const name = k.replace("stroke-", "").replace(",", "p").replace(".", "p");
    lines.push(`  --ratel-stroke-${name}: ${px(val)};`);
  }
  return lines.join("\n");
}

// ─── Build: spacing ────────────────────────────────────────────────────────────

function buildSpacing() {
  const lines = ["  /* === PRIMITIVE: Spacing Scale === */"];
  const skip  = new Set(["-0,8", "-0,4", "0,4", "0,5", "0,75", "0,8", "1,25",
                         "1,5", "1,6", "1,75", "2,25", "2,5", "2,75"]);
  for (const [k, v] of Object.entries(tokens)) {
    if (k.startsWith("$") || skip.has(k)) continue;
    const val = typeof v === "object" ? v["$value"] : v;
    if (typeof val === "number" && val >= 0) {
      lines.push(`  --ratel-space-${k}: ${px(val)};`);
    }
  }
  return lines.join("\n");
}

// ─── Build: typography ─────────────────────────────────────────────────────────

function buildTypography() {
  const lines = ["  /* === TYPOGRAPHY === */",
                 "  --ratel-font-family: 'Raleway', sans-serif;",
                 ""];

  // Font weights
  lines.push("  /* Weights */");
  const weights = typography["Weights"] || {};
  for (const [name, v] of Object.entries(weights)) {
    const val = typeof v === "object" ? v["$value"] : v;
    lines.push(`  --ratel-font-weight-${name.toLowerCase()}: ${val};`);
  }
  lines.push("");

  // Type scale groups: Title, Heading, Label, Body
  const scaleGroups = ["Title", "Heading", "Label", "Body"];
  for (const group of scaleGroups) {
    const g = typography[group];
    if (!g) continue;
    lines.push(`  /* ${group} */`);
    for (const [variant, props] of Object.entries(g)) {
      if (typeof props !== "object" || !("fontSize" in props)) continue;
      const slug = `${group.toLowerCase()}-${variant.toLowerCase().replace(/\s+/g, "-")}`;
      const size = props.fontSize?.["$value"];
      const lh   = props.lineHeight?.["$value"];
      const ls   = props.letterSpacing?.["$value"];
      if (size !== undefined) lines.push(`  --ratel-text-${slug}-size: ${px(size)};`);
      if (lh   !== undefined) lines.push(`  --ratel-text-${slug}-lh: ${px(lh)};`);
      if (ls   !== undefined) lines.push(`  --ratel-text-${slug}-ls: ${parseFloat(ls.toFixed(4))}px;`);
    }
    lines.push("");
  }

  return lines.join("\n");
}

// ─── Alias resolution ─────────────────────────────────────────────────────────

/**
 * Flatten the primitive color file into a lookup keyed by "group/name" path.
 * e.g. primitiveColor["neutral"]["400"] → key "neutral/400"
 */
function flattenPrimitive(obj, prefix = "") {
  const out = {};
  for (const [k, v] of Object.entries(obj)) {
    if (k.startsWith("$")) continue;
    if (typeof v === "object" && v !== null && !("$value" in v)) {
      Object.assign(out, flattenPrimitive(v, prefix ? `${prefix}/${k}` : k));
    } else if (typeof v === "object" && v !== null && "$value" in v) {
      out[prefix ? `${prefix}/${k}` : k] = v;
    }
  }
  return out;
}

const primitiveFlat = flattenPrimitive(primitiveColor);

/**
 * Resolve a Figma alias like "{generic.primary}" by looking up the referenced
 * path in the semantic flat-map first (priority 1: ratel-brand), then in the
 * primitive flat-map (priority 2, slash-delimited).
 *
 * Returns a CSS value string, or null if unresolvable.
 */
function resolveAlias(ref, semanticFlat, visited = new Set()) {
  // ref is the inner content of {…}, e.g. "generic.primary"
  if (visited.has(ref)) {
    console.warn(`  ⚠ circular alias: {${ref}}`);
    return null;
  }
  visited.add(ref);

  // Priority 1 — look in the same theme's semantic tokens (dot-path)
  const semToken = semanticFlat[ref];
  if (semToken) {
    const v = semToken["$value"];
    if (typeof v === "string" && v.startsWith("{")) {
      return resolveAlias(v.slice(1, -1), semanticFlat, visited);
    }
    return hexOrRgba(semToken);
  }

  // Priority 2 — look in primitive colors (slash-path)
  // Figma sometimes uses "neutral/400" style refs; also try converting "." → "/"
  const slashRef  = ref.replace(/\./g, "/");
  const primToken = primitiveFlat[ref] || primitiveFlat[slashRef];
  if (primToken) {
    const v = primToken["$value"];
    if (typeof v === "string" && v.startsWith("{")) {
      return resolveAlias(v.slice(1, -1), semanticFlat, visited);
    }
    return hexOrRgba(primToken);
  }

  return null; // unresolvable
}

// ─── Build: semantic colors (light or dark) ────────────────────────────────────

function buildSemanticColors(src, themeName) {
  const flat       = flatten(src);
  const lines      = [];
  const unresolved = [];

  for (const [path, token] of Object.entries(flat)) {
    if (token["$type"] !== "color") continue;
    const varName = colorVar(path);
    const rawVal  = token["$value"];

    let val;
    if (typeof rawVal === "string" && rawVal.startsWith("{")) {
      // Alias — resolve via priority chain
      val = resolveAlias(rawVal.slice(1, -1), flat);
      if (!val) {
        unresolved.push(`${path} -> ${rawVal}`);
        continue;
      }
    } else {
      val = hexOrRgba(token);
    }

    lines.push(`  ${varName}: ${val};`);
  }

  if (unresolved.length > 0) {
    console.warn(`\n⚠  ${themeName}: ${unresolved.length} unresolvable alias(es) — skipped:`);
    unresolved.forEach(u => console.warn(`   ${u}`));
  }

  return lines.join("\n");
}

// ─── Assemble CSS ──────────────────────────────────────────────────────────────

const css = `/*
 * tokens.css — GENERATED by scripts/build-tokens.js
 * Source of truth: variants/
 * DO NOT edit this file manually — run: node scripts/build-tokens.js
 */

/* ============================================================
   ROOT: Primitive tokens + Dark semantic colors (default theme)
   Dark is the default. Light theme overrides below via [data-theme="light"].
   ============================================================ */
:root {
${buildRadius()}

${buildStroke()}

${buildSpacing()}

${buildTypography()}

  /* Semantic colors — dark (default) */
${buildSemanticColors(darkTokens, "dark")}
}

/* ============================================================
   LIGHT THEME OVERRIDES
   ============================================================ */
[data-theme="light"] {
  /* Semantic colors — light */
${buildSemanticColors(lightTokens, "light")}
}
`;

// ─── Write outputs ─────────────────────────────────────────────────────────────

fs.mkdirSync(path.dirname(OUT_CSS), { recursive: true });
fs.writeFileSync(OUT_CSS, css, "utf8");
console.log(`✓ tokens.css written to ${OUT_CSS}`);

// Also write a flat JSON manifest for tooling use
const manifest = {
  radius:     {},
  stroke:     {},
  spacing:    {},
  typography: { family: "Raleway", weights: {}, scale: {} },
  light:      flatten(lightTokens),
  dark:       flatten(darkTokens),
};
for (const [k, v] of Object.entries(radius)) {
  if (!k.startsWith("$") && k.startsWith("rounded-") && !k.match(/rounded-[tblrse]/)) {
    manifest.radius[k] = typeof v === "object" ? v["$value"] : v;
  }
}
for (const [k, v] of Object.entries(stroke)) {
  if (!k.startsWith("$")) manifest.stroke[k] = typeof v === "object" ? v["$value"] : v;
}
fs.writeFileSync(OUT_JSON, JSON.stringify(manifest, null, 2), "utf8");
console.log(`✓ token-manifest.json written to ${OUT_JSON}`);
