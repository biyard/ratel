---
name: Space Layout Responsive Implementation
description: YouTube-style responsive layout for Space pages - bottom nav on mobile, sidebar on desktop
type: project
---

Implemented responsive Space layout with YouTube-style behavior (2026-03-14):

- Desktop (>900px tablet breakpoint): 250px sidebar (SpaceNav) + main content -- unchanged
- Mobile/Tablet (<900px): Bottom navigation bar (SpaceBottomNav) replaces sidebar, content fills width

**Key files changed:**
- `app/ratel/src/features/spaces/layout.rs` -- added SpaceBottomNav, responsive padding for content area
- `app/ratel/src/features/spaces/space_common/components/space_bottom_nav.rs` -- new component: fixed bottom bar with icon+label nav items
- `app/ratel/src/features/spaces/space_common/components/space_top/mod.rs` -- responsive header: icon-only buttons on mobile, reduced padding/gaps
- `app/ratel/src/features/spaces/space_common/components/mod.rs` -- registered space_bottom_nav module

**Why:** Space pages had no navigation below tablet breakpoint since sidebar was hidden via `hidden tablet:flex`, leaving mobile users stranded.

**How to apply:** Use `tablet:hidden` to show bottom nav only on mobile/tablet, use `max-mobile:hidden` on button labels for icon-only mode on small screens.
