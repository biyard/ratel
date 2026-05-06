#!/usr/bin/env bash
# make-media.sh — convert Playwright .webm recordings into docs media.
#
# Usage:
#   tests/docs/make-media.sh <chapter> <scene> [--spec-glob <pattern>]
#
# Args:
#   <chapter>   Chapter folder under docs/static/media/, e.g. "getting-started"
#   <scene>     Scene name, e.g. "signup". Output files will be named
#               docs/static/media/<chapter>/<scene>.{mp4,gif}
#
# Optional flags:
#   --spec-glob <pattern>   Glob (relative to test-results/docs/) used to find
#                           the source .webm. Defaults to a glob built from
#                           <chapter>-<scene>: "*<chapter>-<scene>*/video.webm".
#                           If multiple matches exist, the most recently
#                           modified one wins.
#
# Examples:
#   tests/docs/make-media.sh getting-started signup
#   tests/docs/make-media.sh teams create-team \
#       --spec-glob "*teams-create-team*/video.webm"
#
# Output:
#   docs/static/media/<chapter>/<scene>.mp4   (h264, web-friendly, faststart)
#   docs/static/media/<chapter>/<scene>.gif   (15 fps, 720px wide, looping)
#
# Requirements:
#   - ffmpeg on PATH
#   - A prior `npx playwright test --project=Docs ...` run that produced a
#     .webm file under playwright/test-results/docs/.
#
# This script never starts a server, never runs Playwright, and never
# touches app/ratel/. It is purely a post-processing helper.

set -euo pipefail

# Resolve repo paths relative to this script so the helper works from any cwd.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLAYWRIGHT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
REPO_ROOT="$(cd "$PLAYWRIGHT_DIR/.." && pwd)"
RESULTS_DIR="$PLAYWRIGHT_DIR/test-results/docs"
MEDIA_ROOT="$REPO_ROOT/docs/static/media"

usage() {
  sed -n '2,33p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
  exit "${1:-1}"
}

if [[ $# -lt 2 ]]; then
  usage 1
fi

CHAPTER="$1"
SCENE="$2"
shift 2

SPEC_GLOB=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --spec-glob)
      SPEC_GLOB="$2"
      shift 2
      ;;
    -h | --help)
      usage 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage 1
      ;;
  esac
done

if ! command -v ffmpeg >/dev/null 2>&1; then
  echo "error: ffmpeg not found on PATH" >&2
  exit 1
fi

if [[ ! -d "$RESULTS_DIR" ]]; then
  echo "error: no Playwright results dir at $RESULTS_DIR" >&2
  echo "       Run \`npx playwright test --project=Docs ...\` first." >&2
  exit 1
fi

# Default glob: any test-results subdir mentioning "<chapter>-<scene>".
if [[ -z "$SPEC_GLOB" ]]; then
  SPEC_GLOB="*${CHAPTER}-${SCENE}*/video.webm"
fi

# Find the most-recently-modified .webm matching the glob.
# Use bash globstar so ** descends into nested test-output dirs.
shopt -s globstar nullglob
candidates=( "$RESULTS_DIR"/$SPEC_GLOB "$RESULTS_DIR"/**/$SPEC_GLOB )
shopt -u globstar nullglob

if [[ ${#candidates[@]} -eq 0 ]]; then
  echo "error: no .webm matched glob '$SPEC_GLOB' under $RESULTS_DIR" >&2
  echo "       Hint: list candidates with \`ls $RESULTS_DIR\`" >&2
  exit 1
fi

# Pick the newest match.
SRC=""
SRC_MTIME=0
for cand in "${candidates[@]}"; do
  [[ -f "$cand" ]] || continue
  mtime=$(stat -c %Y "$cand")
  if (( mtime > SRC_MTIME )); then
    SRC="$cand"
    SRC_MTIME=$mtime
  fi
done

if [[ -z "$SRC" ]]; then
  echo "error: no usable .webm found" >&2
  exit 1
fi

OUT_DIR="$MEDIA_ROOT/$CHAPTER"
mkdir -p "$OUT_DIR"
OUT_MP4="$OUT_DIR/$SCENE.mp4"
OUT_GIF="$OUT_DIR/$SCENE.gif"

echo "==> source webm: $SRC"
echo "==> chapter:     $CHAPTER"
echo "==> scene:       $SCENE"
echo "==> output mp4:  $OUT_MP4"
echo "==> output gif:  $OUT_GIF"

# --- mp4 (h264, yuv420p, faststart for in-browser <video> streaming) -------
echo "==> encoding mp4 ..."
ffmpeg -y -hide_banner -loglevel error \
  -i "$SRC" \
  -movflags +faststart \
  -pix_fmt yuv420p \
  -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" \
  -c:v libx264 \
  -preset slow \
  -crf 23 \
  -an \
  "$OUT_MP4"

# --- gif (15 fps, 720px wide, palette-optimized, looping) ------------------
# Two-pass palette generation keeps file size + colors balanced; without it
# 15-second screencaps balloon to 30+ MB.
PALETTE="$(mktemp -t ratel-docs-palette.XXXXXX.png)"
trap 'rm -f "$PALETTE"' EXIT

echo "==> generating palette ..."
ffmpeg -y -hide_banner -loglevel error \
  -i "$SRC" \
  -vf "fps=15,scale=720:-1:flags=lanczos,palettegen=stats_mode=diff" \
  "$PALETTE"

echo "==> encoding gif ..."
ffmpeg -y -hide_banner -loglevel error \
  -i "$SRC" \
  -i "$PALETTE" \
  -lavfi "fps=15,scale=720:-1:flags=lanczos[v];[v][1:v]paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle" \
  -loop 0 \
  "$OUT_GIF"

mp4_size=$(du -h "$OUT_MP4" | cut -f1)
gif_size=$(du -h "$OUT_GIF" | cut -f1)
echo "==> done: mp4=${mp4_size}, gif=${gif_size}"
echo
echo "Embed:"
echo "  <video src=\"/media/$CHAPTER/$SCENE.mp4\" controls width=\"720\" muted />"
echo "  <img   src=\"/media/$CHAPTER/$SCENE.gif\" alt=\"$CHAPTER $SCENE\" width=\"720\" />"
