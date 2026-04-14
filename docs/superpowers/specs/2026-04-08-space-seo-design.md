# Space/Post SEO Meta Design

**Date**: 2026-04-08  
**Status**: Approved

## Problem

Space and Post pages show Ratel's global SEO (title, description, image) instead of page-specific content when shared on Slack, Twitter, etc.

**Root cause**: Dioxus SSR renders parent components first. The global `SeoMeta` in `app.rs` emits `og:title`, `og:description`, `og:image` tags before `SpaceLayout`'s `SeoMeta`. Social media crawlers use the **first** occurrence of each OG tag, so the Ratel global metadata always wins.

Additionally, `SpaceLayout`'s `SeoMeta` is missing the `image` prop, and `PostDetail` has no `SeoMeta` at all.

## Solution

### 1. Remove global `SeoMeta` from `app.rs`
- Eliminates the first-occurrence problem
- Each page/layout is responsible for its own SEO

### 2. Add default Ratel `SeoMeta` to `Index` (homepage)
- Move the Ratel branding SEO to the homepage only
- Preserves SEO for the main landing page

### 3. Fix `SpaceLayout` `SeoMeta`
- Add `image` prop: space logo, fallback to Ratel logo (`https://metadata.ratel.foundation/logos/logo-symbol.png`)
- Title: `space.title`
- Description: `space.description()` (HTML-stripped content)

### 4. Add `SeoMeta` to `PostDetail`
- Title: post title
- Description: HTML-stripped `html_contents` (first ~200 chars)
- Image: first URL from `post.urls` if available, otherwise empty (no image)

## Files Changed

| File | Change |
|------|--------|
| `app/ratel/src/app.rs` | Remove global `SeoMeta` |
| `app/ratel/src/views/index/mod.rs` | Add Ratel default `SeoMeta` |
| `app/ratel/src/features/spaces/layout.rs` | Add `image` prop to `SeoMeta` |
| `app/ratel/src/features/posts/views/post_detail/mod.rs` | Add `SeoMeta` with post data |

## OG/Twitter Coverage

The `SeoMeta` component already handles:
- `og:title`, `og:description`, `og:image`, `og:type`, `og:url`
- `twitter:card`, `twitter:title`, `twitter:description`, `twitter:image`

No changes to `SeoMeta` component needed.
