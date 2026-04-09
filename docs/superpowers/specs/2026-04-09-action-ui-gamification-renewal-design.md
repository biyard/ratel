# Space Action UI — Gamification Renewal

**Status:** Draft design · ready for implementation planning
**Date:** 2026-04-09
**Branch:** `renewal/new-action-ui`
**Scope:** Space-scoped action UI + global player profile + site-wide primitives refresh

---

## 1. Vision

The Space Action page becomes a **Quest Map** — a multi-chapter branching tree of actions where every node shows the XP it pays out upfront, users build combos by clearing nodes in sequence, the headline number is *the XP they could earn right now*, and finishing a quest fires an animated reward moment with combo/streak/level updates. The dungeon vibe starts at the **space entry** (`/spaces/{id}`), not just the action page.

**User journey (participant, 1,200-person Space):**

1. Opens `/spaces/{id}` → redirected to `/spaces/{id}/actions` (the Quest Map).
2. Persistent **Dungeon Hero band** at the top of every space sub-page shows Space title (stylized), party stats, your XP HUD (level + progress + streak + combo chip), and a leaderboard rail with your rank.
3. Past chapters collapse into a compact "✓ Passed" strip. The active chapter expands with its internal DAG.
4. Clicking an active quest tile opens the **Quest Briefing** overlay with full XP math (`base × participants × combo × streak = total`), rules, prerequisites, and unlock consequences.
5. The user plays the quest in the restyled action player (poll/quiz/discussion/follow).
6. On submit, the **Completion Overlay** animates through XP burst → combo/streak bump → level-up (if triggered) → unlock reveal.
7. The user navigates to `/spaces/{id}/leaderboard` (per-space full page with podium) or `/me/profile` (global player profile across all spaces).

**Dopamine loop:** *see the quest map → see the stakes upfront → clear nodes with growing combo → big animated payout → number on profile goes up.*

---

## 2. Scope

### In V1

| # | Screen | Location |
|---|---|---|
| 1 | **Dungeon Hero band** (persistent) | `SpaceLayout` — every space sub-page |
| 2 | **Quest Map** (action list, quest tree) | `/spaces/{id}/actions` (participant) |
| 3 | **Chapter Editor** (creator) | `/spaces/{id}/actions` (creator, edit mode) |
| 4 | **Quest Briefing** overlay | modal over the Quest Map |
| 5 | **Action Player** (restyled) | `/spaces/{id}/actions/{poll,quiz,discussion,follow}/{id}` |
| 6 | **Completion Overlay** | modal after action submit |
| 7 | **Per-space Leaderboard** | `/spaces/{id}/leaderboard` (NEW) |
| 8 | **Global Player Profile** | `/me/profile` (NEW) |
| 9 | **Site-wide primitives refresh** | `common::components::*` |

### Out of V1 (future expansion)

- Achievements collection
- Seasons / leagues
- Cross-space skill tree (Voter / Debater / Scholar / Networker)
- Global cross-space leaderboard
- Retroactive creator earnings backfill

---

## 3. Design Decisions Locked

| Dimension | Decision |
|---|---|
| **Gamification depth** | Full RPG Arena (cross-space identity) + Quest Map shape from Quest Journey |
| **Visual tone** | Arcade Bold rendered with glass + multi-stop gradients + semi-3D depth. Cinematic, not flat-neon. |
| **Theme** | Dark-first, light theme optional (existing `data-theme` attribute) |
| **Default landing** | `/spaces/{id}` redirects to `/spaces/{id}/actions` (was `/dashboard`) |
| **Dashboard fate** | Kept as creator-leaning analytics page (Option α) |
| **Quest ordering** | Custom serial chapters (author-editable) + DAG within each chapter |
| **Chapter model** | Author defines `name`, `actor_role`, `completion_benefit`; chapters serialize; default seeded on new spaces |
| **Role transitions** | Forward only (Viewer → Candidate → Participant). No downgrades. |
| **Past chapters** | Collapsed into expandable "✓ Passed" strip |
| **XP formula** | `XP = base_P × participants_snapshot × combo × streak` |
| **Participant definition** | Users with `SpaceUserRole::Participant` role only (no Viewers, no Candidates) |
| **Participant snapshot timing** | Locked at the moment the user submits — never recomputed retroactively |
| **Points (P) mapping** | `P` (existing `total_point`) stays as raw award; `XP` is derived. Both tracked separately. |
| **Combo** | In-space consecutive clears. ×2 at 3 clears, ×3 at 5. Breaks at 24h inactivity in the space. |
| **Streak** | Global daily counter. ×1.05 at 3 days, ×1.15 at 7, ×1.5 at 30. Resets on missed day. |
| **Level curve** | `level = floor(sqrt(total_xp / 1000)) + 1` |
| **Class labels** | None. Just "Level N · Citizen · Tier X" where tier = bucket of levels. |
| **Creator fee** | **10% additive** (minted, not cut from user). Credited to space author. |
| **Creator recipient** | One author per space — a `User` OR a `Team` (polymorphic). Team-internal distribution is out of scope. |
| **Creator earnings retroactive** | Skip. Start tracking from the moment the new system ships. |
| **Mobile** | Quest Map collapses to vertical chapter accordion with stacked quest cards. |
| **Accessibility** | All animations skippable; `prefers-reduced-motion` respected; hero frames render statically if JS disabled. |

---

## 4. Custom Chapter Model

Chapters are the backbone of the Quest Map. A space has N ordered chapters. Each chapter has:

- **Name** (e.g., "Proving yours", "Debate")
- **Optional description**
- **Actor role** — which `SpaceUserRole` can submit actions in this chapter
- **Completion benefit** — what happens when the user finishes every action in the chapter
- **Serial order** — chapters progress in order; chapter N+1 is gated by chapter N even when both have the same `actor_role`

**Completion benefit enum:**

```rust
enum ChapterBenefit {
    RoleUpgradeTo(SpaceUserRole),      // e.g., Ch0 Viewer → Candidate
    RoleUpgradeAndXp(SpaceUserRole),   // e.g., Ch1 Candidate → Participant + XP
    XpOnly,                            // Ch2+ pure XP
}
```

**Example (user-provided reference):**

| # | Name | Actor | Benefit |
|---|---|---|---|
| 0 | Proving yours | `Viewer` | `RoleUpgradeTo(Candidate)` |
| 1 | Qualify | `Candidate` | `RoleUpgradeAndXp(Participant)` |
| 2 | Debate | `Participant` | `XpOnly` |
| 3 | Final Poll | `Participant` | `XpOnly` |

**Default seeding:** new spaces get two default chapters — "Qualify" (Candidate → Participant+XP) and "Participate" (Participant → XpOnly). Creators can add/rename/reorder later.

**Migration:** existing spaces are seeded with those same two defaults; existing actions are assigned by their `prerequisite: bool` flag (`true` → Qualify, `false` → Participate).

**Rules:**

1. **Chapters are strictly serial.** Chapter N+1 requires all actions in Chapters 0..N to be complete by the user (even when `actor_role` matches).
2. **Chapters gate by role.** A Viewer sees Chapter 0 active, others labeled "requires Candidate". A Candidate sees Chapter 0 as "✓ already past", Chapter 1 active. Participants see Chapters 0 and 1 as "✓ already past", Chapter 2 active.
3. **Past chapters collapse** into a single strip at the top of the Quest Map; click to expand for review.
4. **Chapter completion** = all actions in the chapter are done (not just terminal leaves of the DAG).
5. **On chapter completion**, the server awards the `completion_benefit`:
   - For `RoleUpgradeTo(R)` / `RoleUpgradeAndXp(R)`, the user's role in that space is upgraded (write to `SpacePanelParticipant` or equivalent).
   - Next chapter unlocks for that user.

**Inside a chapter**, actions form a DAG via `depends_on: Vec<ActionId>`. Multiple parents/children allowed. Cross-chapter dependencies are forbidden. Cycle detection runs server-side on every update.

---

## 5. XP Formula

```text
XP_earned = base_P × participants_snapshot × combo_multiplier × streak_multiplier
```

Where:

- `base_P` = the action's `total_point` field (existing, unchanged)
- `participants_snapshot` = the count of `SpaceUserRole::Participant` users in the space at the moment of submission — locked permanently
- `combo_multiplier` = `1.0` for 0-2 consecutive clears, `2.0` for 3-4, `3.0` for 5+ (within the same space, in-chapter streak)
- `streak_multiplier` = `1.0` for 0-2 days, `1.05` for 3-6, `1.15` for 7-29, `1.5` for 30+ (global daily)

**Level curve:** `level = floor(sqrt(total_xp / 1000)) + 1`
- Level 1: 0 XP
- Level 5: ~16,000 XP
- Level 10: ~81,000 XP
- Level 20: ~361,000 XP

**Creator fee (additive):**

```text
creator_xp      = XP_earned × 0.10    # minted, does not reduce user XP
creator_points  = base_P    × 0.10    # same
```

Credited to `SpaceCreatorEarnings.recipient` — either a `User(id)` or `Team(id)`. For `User` recipients, the creator share also flows into their `UserGlobalXp` aggregate (so creators level up from their own spaces).

---

## 6. Data Model

### New entities

```rust
// DynamoEntity derive on all models

SpaceChapter {
    pk: Partition::Space(space_id),
    sk: EntityType::SpaceChapter(chapter_id),
    order: u32,
    name: String,
    description: Option<String>,
    actor_role: SpaceUserRole,
    completion_benefit: ChapterBenefit,
    created_at: i64,
    updated_at: i64,
}

enum ChapterBenefit {
    RoleUpgradeTo(SpaceUserRole),
    RoleUpgradeAndXp(SpaceUserRole),
    XpOnly,
}

SpaceXpLedgerEntry {
    pk: Partition::Space(space_id),
    sk: EntityType::XpLedger { user_id, timestamp },
    action_id: String,
    base_points: i64,
    participants_snapshot: u32,
    combo_multiplier: f32,
    streak_multiplier: f32,
    xp_earned: i64,
    is_creator_share: bool,
}

UserGlobalXp {
    pk: Partition::User(user_id),
    sk: EntityType::GlobalXp,
    total_xp: i64,
    total_points: i64,
    level: u32,
    spaces_entered: u32,
    spaces_cleared: u32,
    quests_cleared: u32,
    last_updated: i64,
}

UserStreak {
    pk: Partition::User(user_id),
    sk: EntityType::Streak,
    current_streak: u32,
    last_active_date: String,           // YYYY-MM-DD in user's timezone
    longest_streak: u32,
}

UserSpaceCombo {
    pk: Partition::Space(space_id),
    sk: EntityType::Combo(user_id),
    current_streak_in_space: u32,
    combo_multiplier: f32,
    last_completion_at: i64,
}

SpaceCreatorEarnings {
    pk: Partition::Space(space_id),
    sk: EntityType::CreatorEarnings,
    recipient: CreatorRecipient,
    total_xp: i64,
    total_points: i64,
    last_updated: i64,
}

enum CreatorRecipient {
    User(UserPartition),
    Team(TeamPartition),
}
```

### Changed entity

```rust
SpaceAction {
    // existing fields preserved: title, description, started_at, ended_at,
    //   prerequisite, total_point, total_score, action_type, ...
    chapter_id: SpaceChapterEntityType,   // NEW — required; denormalized hint
    depends_on: Vec<SpaceActionId>,       // NEW — DAG parents (same chapter only)
    // prerequisite: bool stays as denormalized hint; chapter_id is source of truth
}
```

### Typed errors

```rust
// features/spaces/pages/actions/gamification/types/error.rs
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum GamificationError {
    #[error("chapter must have at least one action before deletion")]
    #[translate(en = "Chapter still has actions — move or delete them first",
                ko = "챕터에 퀘스트가 남아 있습니다. 먼저 이동하거나 삭제하세요")]
    ChapterNotEmpty,

    #[error("dependency cycle detected")]
    #[translate(en = "That would create a dependency loop",
                ko = "의존 관계가 순환됩니다")]
    CycleDetected,

    #[error("cross-chapter dependency forbidden")]
    #[translate(en = "Dependencies must stay within a single chapter",
                ko = "의존 관계는 같은 챕터 안에서만 설정할 수 있습니다")]
    CrossChapterDependency,

    #[error("action locked: prerequisites not met")]
    #[translate(en = "Complete the prerequisite quests first",
                ko = "선행 퀘스트를 먼저 완료하세요")]
    ActionLocked,

    #[error("prior chapter incomplete")]
    #[translate(en = "Finish the previous chapter first",
                ko = "이전 챕터를 먼저 완료하세요")]
    PriorChapterIncomplete,

    #[error("role mismatch for chapter actor")]
    #[translate(en = "You're not at the right role for this chapter",
                ko = "이 챕터를 진행할 수 있는 역할이 아닙니다")]
    RoleMismatch,
}

// Register in common::Error via #[from] + #[translate(from)]
```

---

## 7. Access Control Rewrite

`features/spaces/pages/actions/access.rs` — `can_execute_space_action` is rewritten to replace the `prerequisite: bool` check with chapter + DAG logic:

```rust
pub fn can_execute_space_action(
    role: SpaceUserRole,
    chapter: &SpaceChapter,
    action_deps_met: bool,            // all `depends_on` parents complete
    prior_chapters_complete: bool,    // chapters 0..chapter.order all complete
    status: Option<SpaceStatus>,
    join_anytime: bool,
) -> bool {
    let role_matches = role == chapter.actor_role || role == SpaceUserRole::Creator;

    let status_ok = matches!(status, Some(SpaceStatus::Ongoing))
        || (join_anytime && matches!(status, Some(SpaceStatus::Open)));

    role_matches
        && action_deps_met
        && prior_chapters_complete
        && status_ok
}
```

End-of-space read-only behavior (`SpaceStatus::Processing | Finished`) is preserved — `status_ok` fails automatically.

---

## 8. Server Functions

### New endpoints

```text
// Chapter CRUD (creator role required)
POST   /api/spaces/:space_id/chapters                    create_chapter
GET    /api/spaces/:space_id/chapters                    list_chapters
PATCH  /api/spaces/:space_id/chapters/:chapter_id        update_chapter
DELETE /api/spaces/:space_id/chapters/:chapter_id        delete_chapter
POST   /api/spaces/:space_id/chapters/reorder            reorder_chapters

// Quest map + progress (participant view)
GET    /api/spaces/:space_id/quest-map                   get_quest_map
    → { chapters: Vec<ChapterView>, current_user_state: UserQuestState }

// Leaderboard (extends existing get_ranking_handler)
GET    /api/spaces/:space_id/leaderboard
    ?chapter_id=<optional>
    &window=<all | week | month>
    &bookmark=<optional>

// Global profile
GET    /api/me/profile                                   get_my_global_profile
    → { level, total_xp, streak, combo_armed_in: Vec<Space>, stats,
        creator_earnings, space_standings }
```

### Changed endpoints

`PATCH /api/spaces/:space_id/actions/:action_id` extends `UpdateSpaceActionRequest` with:

```rust
enum UpdateSpaceActionRequest {
    // existing variants preserved
    Prerequisite { prerequisite: bool },
    ChapterId { chapter_id: SpaceChapterEntityType },      // NEW
    DependsOn { depends_on: Vec<SpaceActionEntityType> },  // NEW
}
```

Both new variants run server-side validation:
- `chapter_id` — must exist in the space
- `depends_on` — cycle detection + all targets must be in the same chapter

### XP computation service

`features/spaces/pages/actions/gamification/services/award_xp.rs`:

```rust
pub async fn award_xp(
    cli: &DynamoClient,
    user_id: UserPartition,
    space_id: SpacePartition,
    action_id: SpaceActionEntityType,
) -> Result<XpGainResponse> {
    // 1. Read action, chapter, base_P
    // 2. Snapshot Participant count in the space
    // 3. Read + increment UserSpaceCombo → new combo_multiplier
    // 4. Read + update UserStreak → new streak_multiplier
    // 5. XP = base_P × participants × combo × streak
    // 6. Write SpaceXpLedgerEntry (append-only)
    // 7. Update UserGlobalXp aggregate (+XP, +P, recompute level)
    // 8. Check chapter completion → if all actions done, apply completion_benefit:
    //    - RoleUpgradeTo: write to SpacePanelParticipant, flip role
    //    - XP already added via the action itself
    // 9. Compute + write 10% creator share to SpaceCreatorEarnings
    //    - For User recipient, also update their UserGlobalXp
    // 10. Compute newly unlocked actions (DAG children whose other parents are done)
    // 11. Return XpGainResponse {
    //         xp_earned, combo, streak, old_level, new_level,
    //         unlocked_actions, role_upgraded, chapter_completed
    //     }
}
```

Called from existing endpoints: `respond_poll`, `respond_quiz`, `add_comment` (on first comment per user per action), `follow_user` (on first follow per action). Each endpoint returns `XpGainResponse` in the success body so the client can play the completion animation.

---

## 9. Routes & Component Map

### Route changes

```rust
// features/spaces/route.rs
#[nest("/spaces")]
    #[nest("/:space_id")]
        #[layout(SpaceLayout)]                    // ← renders DungeonHero persistently
            #[route("/dashboard/:..rest")]   SpaceDashboardPage { ... }
            #[route("/overview/:..rest")]    Overview { ... }
            #[route("/actions/:..rest")]     Actions { ... }
            #[route("/leaderboard")]         SpaceLeaderboardPage    // NEW
            #[route("/report/:..rest")]      Report { ... }
            #[route("/apps/:..rest")]        Apps { ... }
            #[redirect("/", … → Actions { rest: vec![] })]   // CHANGED
        #[end_layout]
    #[end_nest]
#[end_nest]

// App-level (in app/ratel/src/route.rs)
#[route("/me/profile")]   GlobalPlayerProfilePage                   // NEW
```

### New feature module structure

```
features/spaces/pages/actions/gamification/
├── mod.rs
├── i18n.rs
├── types/
│   ├── chapter.rs                # SpaceChapter, ChapterBenefit
│   ├── xp_gain.rs                # XpGainResponse, XpBreakdown
│   └── error.rs                  # GamificationError
├── models/
│   ├── space_chapter.rs
│   ├── space_xp_ledger.rs
│   ├── user_global_xp.rs
│   ├── user_streak.rs
│   ├── user_space_combo.rs
│   └── space_creator_earnings.rs
├── controllers/
│   ├── chapters/
│   │   ├── create_chapter.rs
│   │   ├── list_chapters.rs
│   │   ├── update_chapter.rs
│   │   ├── delete_chapter.rs
│   │   └── reorder_chapters.rs
│   ├── quest_map/get_quest_map.rs
│   ├── leaderboard/get_leaderboard.rs
│   └── profile/get_my_global_profile.rs
├── services/
│   ├── award_xp.rs
│   ├── check_chapter_complete.rs
│   ├── validate_dag.rs
│   └── apply_chapter_benefit.rs
├── components/
│   ├── dungeon_hero/
│   │   ├── mod.rs
│   │   ├── xp_hud.rs
│   │   └── leaderboard_rail.rs
│   ├── quest_map/
│   │   ├── mod.rs
│   │   ├── chapter_section.rs
│   │   ├── quest_node.rs
│   │   ├── dag_canvas.rs
│   │   └── collapsed_past_strip.rs
│   ├── quest_briefing/mod.rs
│   ├── completion_overlay/
│   │   ├── mod.rs
│   │   ├── xp_gain_animation.rs
│   │   ├── level_up_scene.rs
│   │   └── unlock_reveal.rs
│   ├── chapter_editor/
│   │   ├── mod.rs
│   │   ├── chapter_row.rs
│   │   ├── chapter_expanded.rs
│   │   └── dag_editor_canvas.rs
│   └── creator_earnings_card/mod.rs
└── hooks/
    ├── use_quest_map.rs
    ├── use_xp_hud.rs
    └── use_completion_flow.rs
```

### Touched existing files

| File | Change |
|---|---|
| `features/spaces/route.rs` | Flip `/` redirect from `Dashboard` → `Actions`; register `SpaceLeaderboardPage`; register `GlobalPlayerProfilePage` at app level |
| `features/spaces/layout.rs` (`SpaceLayout`) | Wrap content in `DungeonHero`; load XP HUD + leaderboard rail once at layout level |
| `features/spaces/pages/actions/access.rs` | Rewrite `can_execute_space_action` to use chapter + DAG + prior-chapters logic |
| `features/spaces/pages/actions/components/action_card/mod.rs` | **Retired** — replaced by `quest_node.rs` |
| `features/spaces/pages/actions/views/main/participant_page/mod.rs` | Render `QuestMap` instead of card grid |
| `features/spaces/pages/actions/views/main/creator_page/mod.rs` | Render `ChapterEditor` instead of card grid |
| `features/spaces/pages/actions/actions/poll/controllers/respond_poll.rs` | Call `award_xp`, return `XpGainResponse` |
| `features/spaces/pages/actions/actions/quiz/controllers/respond_quiz.rs` | Same |
| `features/spaces/pages/actions/actions/discussion/controllers/comments/add_comment.rs` | Call `award_xp` on first comment per user per action |
| `features/spaces/pages/actions/actions/follow/controllers/follow_user.rs` | Call `award_xp` on first follow per action |
| `features/spaces/pages/actions/actions/*/views/main/*/content.rs` | After submit, drive `CompletionOverlay` from `XpGainResponse` instead of redirecting |
| `features/activity/components/ranking_widget.rs` | Retired; rail now lives in `DungeonHero.leaderboard_rail` |
| `features/spaces/space_common/types/keys.rs` | New query keys for chapters, quest map, global profile |
| `common/types/error.rs` | Register `GamificationError` via `#[from]` + `#[translate(from)]` |

---

## 10. Primitives & Design Tokens Refresh

**Principle:** the new aesthetic (glass surfaces + multi-stop gradients + inset highlights + radial spheres + semi-3D depth) becomes the **default** for the whole app. No gamification-only theme split.

### New/extended design tokens

```css
/* tailwind.css — added alongside existing semantic tokens */

/* Glass surfaces */
--glass-surface-primary:   linear-gradient(135deg, rgba(255,255,255,0.05), rgba(255,255,255,0.01));
--glass-surface-accent:    linear-gradient(135deg, rgba(252,179,0,0.08), rgba(0,0,0,0.3));
--glass-surface-teal:      linear-gradient(135deg, rgba(110,237,216,0.08), rgba(0,0,0,0.3));
--glass-blur:              blur(16px);
--glass-border:            1px solid rgba(255,255,255,0.08);
--glass-border-primary:    1px solid rgba(252,179,0,0.3);
--glass-border-accent:     1px solid rgba(110,237,216,0.35);

/* Depth shadows (stacked) */
--depth-sm:   0 8px 24px -8px rgba(0,0,0,0.5), inset 0 1px 0 rgba(255,255,255,0.05);
--depth-md:   0 16px 40px -12px rgba(0,0,0,0.6), inset 0 1px 0 rgba(255,255,255,0.06);
--depth-lg:   0 24px 60px -16px rgba(0,0,0,0.7), inset 0 1px 0 rgba(255,255,255,0.08);
--depth-hero: 0 30px 80px -20px rgba(0,0,0,0.8),
              0 0 120px -30px rgba(252,179,0,0.25),
              inset 0 1px 0 rgba(255,255,255,0.08);

/* Rim glows */
--rim-glow-primary: 0 0 60px -20px rgba(252,179,0,0.35);
--rim-glow-accent:  0 0 60px -20px rgba(110,237,216,0.35);
--rim-glow-danger:  0 0 60px -20px rgba(219,39,128,0.35);

/* Gradient text fills */
--text-gradient-gold: linear-gradient(180deg, #fff8e7 0%, #f0d28a 60%, #a07a30 100%);
--text-gradient-teal: linear-gradient(180deg, #f0fffa 0%, #9fdcc9 50%, #3a7a6a 100%);

/* Progress bar */
--rail-recessed:     linear-gradient(180deg, rgba(0,0,0,0.6), rgba(0,0,0,0.25));
--rail-fill-primary: linear-gradient(90deg, #a07a30, #fcb300 40%, #ffe082 70%, #9fdcc9 100%);

/* Avatar spheres */
--sphere-gold:   radial-gradient(circle at 30% 30%, #fff8e7, #ffe082, #fcb300, #8a5d00);
--sphere-teal:   radial-gradient(circle at 30% 30%, #c4f5e8, #9fdcc9, #3a7a6a);
--sphere-bronze: radial-gradient(circle at 30% 30%, #f5e8d0, #b08d5c, #5a4220);
```

All tokens have light-theme counterparts under `[data-theme="light"]`.

### Primitives updated

| Component | Change | New variants |
|---|---|---|
| `Card` | Default gets glass-surface + depth-sm + glass-border. Existing variants preserved. | `Glass`, `GlassPrimary`, `GlassAccent`, `Hero` |
| `Button` | `Primary` gets gradient fill + inset highlight + depth-sm. Hover states softened. | `Hero` |
| `Badge` | Gradient fills with inset highlight. Same prop API. | — |
| `Avatar` | Radial sphere gradient with rim highlight. | `Sphere` |
| `Progress` | Recessed inset rail + gradient fill + rim glow. | — |
| `Dialog` / `Popup` | `backdrop-filter: blur(20px)` + glass surface + depth-lg + rim glow. | — |
| `Separator` | Default unchanged. New gradient fade variant for section dividers. | `Gradient` |
| `Switch` | Inset recessed track + glowing thumb. | — |
| `Tooltip` | Glass surface + depth-sm. | — |
| `Input` | Focus ring becomes a soft rim glow instead of hard border. | — |
| `LoadingIndicator` | Glow pulse. | — |
| `Skeleton` | Gradient sweep animation. | — |

**Backward compatibility:** all existing `<Card variant={Normal}>` etc. keep working — default variants get a *visual* refresh but the prop API is unchanged. No rsx changes required in existing pages. `data-testid` selectors are unaffected.

**Worktree isolation:** the refresh lands entirely on the `renewal/new-action-ui` worktree; no feature flag. Visual regression is handled by walking through representative pages before merging to `dev`.

---

## 11. Screen Designs

### 11.1 Dungeon Hero band (persistent)

Lives in `SpaceLayout`. Visible on every space sub-page. Three regions:

1. **Title region** — "Dungeon · Floor N" mini-label, gradient space title, party stats (`⚔ N explorers · 🏆 N chapters · ⏳ Nd Nh left`).
2. **Leaderboard rail** — top-3 sphere avatars + your rank pill, links to the full leaderboard page.
3. **XP HUD strip** — level badge (radial sphere tile) + level progress bar + streak chip + combo chip.
4. **Creator Earnings card** (author-only) — crown icon + total XP/P earned from 10% creator fee + Breakdown button. Sits above the XP HUD when the current user is the space author.

### 11.2 Quest Map (`/spaces/{id}/actions`, participant)

- **Collapsed past chapters strip** at the top (✓ N chapters passed · role unlocked · +XP earned · chevron to expand)
- **Active chapter section** with `▶ ACTIVE` pill + chapter name + serial order + internal progress (`1/3`)
- **DAG canvas** inside the active chapter:
  - Quest tiles rendered as glass cards with type badge, title, XP-earned (if cleared) or XP-at-stake (if active)
  - Active tile is a glass hero card with teal rim glow, gradient XP number, "BEGIN ▶" CTA
  - Locked tiles dashed border, greyed, with explicit unlock condition
  - SVG Bezier curves connecting parents → children (gradient strokes for unlocked paths, dashed grey for locked)
- **Locked chapter cards** below the active chapter, dashed border, "Complete Chapter N to unlock" subtitle

### 11.3 Quest Briefing overlay

Appears when tapping an active quest tile. Blurred quest map backdrop. Glass card with teal rim glow:

- **Header band**: type pill · chapter pill · gradient quest title · description · × close
- **Hero XP number** (centered): huge gradient text, "XP at stake" label above, math breakdown below (`base × participants × combo × streak`)
- **Footnote**: "Snapshot locked at submission · combo lost if you abandon this chapter"
- **Rules grid** (2×2): Time window · Retries · Prerequisites · Unlocks
- **Chapter progress strip**: `Chapter N progress · 1/3`
- **Bottom CTA row**: ghost "Back to Map" + teal "BEGIN ▶" hero button

### 11.4 Action Player (restyled)

Existing question-by-question flow (poll/quiz/discussion/follow) keeps its mechanics. Visual restyling only:

- `FullActionLayover` bottom bar gets the glass treatment + persistent combo/XP chips on the left
- `QuestionViewer` title/description get the gradient text treatment
- Answer buttons use new `Button` primitives with depth + hover states
- Progress indicator switches from "Question 2/5" to a dot-row at the top

### 11.5 Completion / Level-Up Overlay

Orchestrated animation sequence after submit:

| Time | Event |
|---|---|
| **t+0** | Submit button pulses, spark particles burst, screen flash |
| **t+0.4s** | +XP number flies up from center; XP HUD counter ticks |
| **t+0.9s** | Combo chip bumps (×2 → ×3), streak counter +1 with shake |
| **t+1.6s** | Level-up hero frame (only if `new_level > old_level`) — sunburst rays, large level badge, tier unlock banner, +XP card, chip strip, unlock toast |
| **t+3s** | Quest map redraws behind the dismissing overlay, new unlocked node lights up |

**Skippable** after t+0.9s. **No level-up frame** if no level threshold crossed — flow is ~2s total. **Role upgrade** replaces the tier label with a bigger "PARTICIPANT UNLOCKED" banner.

### 11.6 Per-Space Leaderboard (`/spaces/{id}/leaderboard`)

- Dungeon Hero band persists at the top with filter chips (`All chapters · Chapter N · This week`)
- **Podium** — #1 tallest (crowned, golden sphere, golden pedestal), #2 teal, #3 bronze, each with avatar + name + XP
- **Scrollable list** below — rank · avatar · name · XP in space · quests · level
- **Your row highlighted** with a golden gradient row, `▴N` delta indicator if your rank changed

### 11.7 Global Player Profile (`/me/profile`)

- **Hero player card** — rotated level badge tile, avatar, gradient tier text, name, global XP progress bar, streak + combo chips
- **4-stat grid** — Total XP · Ratel Points · Dungeons · Quests cleared (each with a "+ this week" subtitle)
- **Creator Earnings card** — crown icon + "N spaces you authored" + total XP/P + Breakdown button (visible only if the user has authored at least one space)
- **Dungeon standings list** — per-space: name + status + mini progress bar + current rank

### 11.8 Chapter Editor (`/spaces/{id}/actions`, creator)

- Top bar: "Creator · Chapter Editor" label + "Preview as Participant" ghost button + "Save & Publish" hero button
- **Chapter list** — drag-reorderable rows (`⋮⋮` handle + CH N pill + name + actor/benefit summary + quest count + Edit + expand arrow)
- **Expanded chapter** — inline editable fields (name input, actor dropdown, benefit dropdown) + DAG canvas below
- **DAG canvas** — quest tiles with dashed teal borders, drag to reorder, shift-drag to wire edges, dependency chips with × to remove, SVG Bezier edges
- **+ Add chapter** dashed slot at the bottom (inherits previous chapter's actor role by default)

**Interaction model** — see Section 6 of this document (interaction table).

---

## 12. Data Migration Plan

One-shot migration binary under `app/ratel/src/bin/migrate_chapters.rs`. Idempotent — safe to re-run.

**Step 1:** Seed two default chapters on every existing space:

```rust
for space in SpaceModel::scan_all(cli).await? {
    if space_has_chapters(&space.pk).await? { continue; }
    let ch0 = SpaceChapter::new(space.pk.clone(), 0, "Qualify".into(),
        SpaceUserRole::Candidate,
        ChapterBenefit::RoleUpgradeAndXp(SpaceUserRole::Participant));
    let ch1 = SpaceChapter::new(space.pk.clone(), 1, "Participate".into(),
        SpaceUserRole::Participant, ChapterBenefit::XpOnly);
    ch0.create(cli).await?;
    ch1.create(cli).await?;
}
```

Only two defaults (not four) — the Viewer→Candidate chapter is a new creator opt-in, not imposed on existing spaces.

**Step 2:** Backfill `chapter_id` on every existing `SpaceAction`:

```rust
for action in SpaceActionModel::scan_all(cli).await? {
    if action.chapter_id.is_some() { continue; }
    let chapter_id = if action.prerequisite { "qualify" } else { "participate" };
    action.update_chapter_id(cli, chapter_id).await?;
}
```

**Step 3:** Backfill `depends_on: Vec::new()` on every existing `SpaceAction` (empty DAG = all parallel, matches current behavior).

**Step 4:** Backfill `UserGlobalXp` from historical `total_point` totals:

```rust
for user_id in distinct_users_with_completions().await? {
    let total_points = sum_total_points_for_user(user_id).await?;
    // Historical XP = total_point (no multipliers, because old system didn't track them)
    UserGlobalXp::upsert(user_id, total_points, total_points, level_from(total_points)).await?;
}
```

Old quests show `XP == P` (no multipliers). New quests show the full formula. Documented in migration log.

**Step 5:** Seed `UserStreak` with `streak=0, longest=0` for all users — no historical reconstruction.

**Step 6:** **Skip** retroactive `SpaceCreatorEarnings` seeding. Start tracking from the moment the new system ships. Documented.

**Step 7:** `RankingWidget` UI retired (rail moves to `DungeonHero`); `get_ranking_handler` endpoint preserved as the data source.

**Rollback plan:** the migration only adds fields; nothing is deleted. Rolling back = deploying the old code. The new fields become dead data but cause no harm.

---

## 13. Testing Plan

### Server function tests (`app/ratel/src/tests/gamification_tests.rs`, new file)

| Test | Asserts |
|---|---|
| `test_create_chapter` | Creator creates chapter; order autoincrements; list returns it |
| `test_reorder_chapters` | Batch reorder atomic, preserves IDs |
| `test_delete_chapter_with_actions_blocked` | Rejected when actions still assigned |
| `test_delete_chapter_empty_ok` | Empty chapter deletes cleanly |
| `test_update_action_depends_on_cycle_rejected` | A→B→A returns `CycleDetected` |
| `test_update_action_depends_on_cross_chapter_rejected` | Cross-chapter dependency returns `CrossChapterDependency` |
| `test_award_xp_basic` | `base_P × participants = XP`; ledger entry written |
| `test_award_xp_combo_bump` | Sequential clears → combo ×2 at 3, ×3 at 5 |
| `test_award_xp_streak_bonus` | 7-day streak → ×1.15 |
| `test_award_xp_creator_share_user` | 10% additive credit to author's `UserGlobalXp` + `SpaceCreatorEarnings` |
| `test_award_xp_creator_share_team` | 10% credit to `Team` recipient |
| `test_chapter_complete_role_upgrade` | Finishing all actions in chapter with `RoleUpgradeTo(Participant)` flips role |
| `test_chapter_complete_xp_only` | No role change; next chapter unlocks |
| `test_locked_action_rejected` | Action with unmet DAG parents returns `ActionLocked` |
| `test_prior_chapter_incomplete_rejected` | Ch2 action rejected if Ch1 incomplete |
| `test_viewer_cannot_submit_ch1` | Role enforcement on `actor_role` |
| `test_space_finished_read_only` | `Finished` status rejects all submissions |
| `test_get_quest_map_response` | Response shape matches client expectation |
| `test_get_my_global_profile_response` | Aggregates reflect recent writes |
| `test_get_leaderboard_with_chapter_filter` | Chapter filter narrows correctly |

### MCP tool tests (`app/ratel/src/tests/mcp_tests.rs`, extended)

New MCP tools: `create_chapter`, `update_chapter`, `delete_chapter`, `list_chapters`, `get_quest_map`, `get_my_global_profile`, `get_leaderboard`. One test per tool using `setup_mcp_test` / `mcp_tool_call` / `extract_tool_content` helpers.

### Playwright e2e

**`playwright/tests/web/quest-map.spec.js`** (new file, participant scenario, `test.describe.serial`):

1. Log in as participant
2. Navigate to `/spaces/{id}` — assert landing at `/actions`
3. Assert `data-testid="dungeon-hero"` visible with XP HUD
4. Assert collapsed past chapters strip visible
5. Click active quest in Chapter 2
6. Assert Quest Briefing overlay with XP math breakdown visible
7. Click "BEGIN ▶"
8. Fill answers via `fill` / `click` helpers
9. Submit
10. Assert `data-testid="completion-overlay"` appears, wait for animation-complete signal (not `networkidle`)
11. Assert "Continue questing" button visible
12. Navigate to `/spaces/{id}/leaderboard`
13. Assert podium + highlighted "you" row

**`playwright/tests/web/quest-map-creator.spec.js`** (new file, creator scenario):

1. Log in as creator
2. Navigate to `/spaces/{id}/actions`
3. Assert `data-testid="chapter-editor"` visible
4. Click `+ Add chapter`, set name "Bonus round"
5. Set actor to Participant, benefit to XP only
6. Drag an action into the new chapter
7. Click "Save & Publish"
8. Click "Preview as Participant"
9. Assert new chapter appears in the quest map

### `data-testid` additions

`dungeon-hero`, `xp-hud-level`, `xp-hud-streak`, `xp-hud-combo`, `leaderboard-rail`, `quest-node-{id}`, `quest-briefing`, `briefing-begin-btn`, `completion-overlay`, `level-up-scene`, `unlock-reveal`, `chapter-editor`, `chapter-row-{id}`, `chapter-name-input`, `chapter-actor-select`, `chapter-benefit-select`, `dag-canvas`, `creator-earnings-card`, `global-profile-hero`, `dungeon-standings-list`.

### Visual regression (manual)

After the primitives refresh, walk through: home feed · post detail · team page · settings · space dashboard · space overview · space apps · space report · space actions (participant + creator). Screenshot before/after. Fix anything that regressed.

---

## 14. Rollout Sequence

Each PR independently reviewable, compiles, passes tests.

| # | PR | What ships |
|---|---|---|
| 1 | Primitives + tokens refresh | Site-wide visual update. No new features. |
| 2 | Data model + migration script | New entities, new controllers, migration binary, tests. No UI. |
| 3 | `DungeonHero` + redirect flip | Persistent hero band, `/` → `/actions` redirect. Action page still renders old card grid below. |
| 4 | `QuestMap` participant view | Replaces card grid. Reads chapters from backend. |
| 5 | `QuestBriefing` + restyled `ActionPlayer` | Briefing overlay, player body gets new tone. |
| 6 | `CompletionOverlay` + `award_xp` | Server returns `XpGainResponse`; client plays animation. |
| 7 | `ChapterEditor` creator view | Unlocks custom chapter structure. |
| 8 | `SpaceLeaderboardPage` + `GlobalPlayerProfilePage` | Full-page surfaces. |
| 9 | MCP tools + e2e tests + polish pass | Final QA. |

Worktree is ready to merge to `dev` after PR 9.

---

## 15. Open Questions / Risks

| Area | Risk | Mitigation |
|---|---|---|
| **Historical XP looks small** | Old quests show `XP == P` (no multipliers), new quests show the multiplied number. Users might feel their history was devalued. | Documented in migration log. Optional: one-shot "historical XP bonus" of 10× at migration for all users' historical XP to soften the visual gap. Decision deferred to launch. |
| **Large space XP inflation** | A 10,000-person space × 200 P × ×3 combo × ×1.5 streak = 9M XP per action. Levels could explode. | Level curve `sqrt(xp/1000)` already damps this. Level 30 = 841k XP. Level 100 = 9.9M XP. Acceptable for the first year; revisit if needed. |
| **Team creator earnings distribution** | Teams accrue XP + P as an entity but team members don't automatically see it on their personal profile. | Team-internal sharing is out of V1 scope. Team page gets a new card showing team earnings; distribution mechanics deferred. |
| **Chapter editor backend cost** | DAG validation + reorder + cycle checks run on every creator edit. | Validation is O(V+E) in a single chapter; chapters are small (typically <20 actions). Fine. |
| **Role downgrade via chapter benefit mistake** | A creator might accidentally set a chapter benefit that effectively blocks users. | `ChapterBenefit` enum is forward-only at the type level — no `RoleDowngrade` variant exists. Validation in `update_chapter` also rejects benefits that would move a user backwards relative to the `actor_role`. |
| **Animation performance on low-end devices** | Level-up scene has SVG rays + gradient text + multiple shadows. | `prefers-reduced-motion` skips all animation. Mobile default = reduced animation (no rays, simpler +XP popup). |
| **Viewer → Candidate is a new capability** | Existing role transition logic assumes Candidate appears via some other mechanism. | Reusing `SpacePanelParticipant` writes for explicit transitions; verify with `get_user_role` middleware. Tested by `test_chapter_complete_role_upgrade`. |
| **Cross-space creator leveling** | A user who authors 10 active spaces gets 10× creator fees flowing into their personal XP. | Intentional. Authoring active spaces is a legitimate way to level up; encourages quality creation. Capped implicitly by the 10% rate. |
| **"Completion" for discussion + follow is fuzzy** | Polls and quizzes have a clear submit event. Discussions and Follow actions are continuous (comments, follow/unfollow). When does the XP award fire? | **Design choice:** XP awarded once per `(user_id, action_id)` pair — on the user's first comment in a discussion action, or on the first follow of any target in a Follow action. Subsequent comments/follows do not re-award XP. Recorded in `SpaceXpLedgerEntry` as append-only with a unique constraint. This preserves existing "completion" behavior in `action_card` (where `quiz_score` / participation check determines "done"). Verify with product before implementation. |

---

## 16. Glossary

| Term | Meaning |
|---|---|
| **Quest** | A single action (poll / quiz / discussion / follow) in the new UI vocabulary |
| **Chapter** | An ordered grouping of quests with a shared actor role and completion benefit |
| **DAG** | The directed-acyclic dependency graph between quests within a single chapter |
| **Dungeon** | A user-facing nickname for a Space, used in UI copy ("Dungeon Hero", "Enter the Dungeon") |
| **XP** | Derived experience points, `base_P × participants × combo × streak`. Drives level. |
| **P** | Raw "Ratel Points" — the existing `total_point` on each action, unchanged. Economy currency. |
| **Combo** | In-space multiplier for consecutive action clears |
| **Streak** | Global daily participation counter |
| **Creator fee** | The 10% additive share of every user's reward minted to the space author |
| **Actor role** | The `SpaceUserRole` that's allowed to submit actions in a chapter |
| **Completion benefit** | What the user receives upon finishing every action in a chapter (role upgrade, XP, or both) |

---

*End of design. Ready for writing-plans skill.*
