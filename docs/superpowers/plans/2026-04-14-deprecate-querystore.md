# Deprecate QueryStore Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Eliminate `common/query/mod.rs` (QueryStore, use_query, use_query_store) and replace all usages with `use_loader` + Loader-in-Context pattern via `use_context_provider`.

**Architecture:** QueryStore provided global key-based cache invalidation. The replacement is: (1) shared loaders live in context providers (SpaceContextProvider for cross-cutting data; page-level for scoped data), consumers call `loader.restart()` for invalidation; (2) local-only loaders use `use_loader` directly with no context.

**Tech Stack:** Dioxus 0.7 fullstack, Rust, `use_loader`, `use_context_provider`, `Loader<T>::restart()`

---

## File Structure

### Modified files

| File | Change |
|------|--------|
| `app/ratel/src/common/mod.rs` | Remove `pub mod query` and `pub use query::*` |
| `app/ratel/src/app.rs` | Remove `use_context_provider(QueryStore::new)` |
| `app/ratel/src/features/spaces/space_common/providers/space_context_provider.rs` | Add `actions`, `ranking`, `my_score` loaders |
| `app/ratel/src/features/spaces/space_common/hooks/use_space.rs` | Remove `use_space_query`, keep `use_space` |
| `app/ratel/src/features/spaces/space_common/hooks/mod.rs` | Add `use_actions`, `use_ranking`, `use_my_score` hooks |
| `app/ratel/src/features/spaces/hooks/use_user.rs` | Replace `use_query` with `use_loader` |
| `app/ratel/src/features/spaces/layout.rs` | Remove unused `use_space_query` import |
| `app/ratel/src/features/activity/components/ranking_widget.rs` | Use `use_ranking()`/`use_my_score()` from context |
| `app/ratel/src/features/spaces/pages/index/leaderboard_panel/component.rs` | Use `use_ranking()`/`use_my_score()` from context |
| `app/ratel/src/features/spaces/pages/index/action_dashboard/component.rs` | Use `use_actions()` from context |
| `app/ratel/src/features/spaces/pages/index/component.rs` | Use `use_actions()` from context in CandidateView |
| `app/ratel/src/features/spaces/pages/index/action_pages/poll/component.rs` | Replace `use_query`→`use_loader`, `query.invalidate`→`loader.restart()`+context |
| `app/ratel/src/features/spaces/pages/index/action_pages/quiz/component.rs` | Replace `use_query`→`use_loader`, `query.invalidate`→`loader.restart()`+context |
| `app/ratel/src/features/spaces/pages/index/action_pages/discussion/component.rs` | Replace `use_query`→`use_loader`, `query.invalidate`→`loader.restart()`+context |
| `app/ratel/src/features/spaces/pages/actions/actions/poll/views/main/mod.rs` | Replace `use_query`→`use_loader` |
| `app/ratel/src/features/spaces/pages/actions/actions/poll/views/main/participant/content.rs` | Replace `use_query`→`use_loader`, `query.invalidate`→`loader.restart()`+context |
| `app/ratel/src/features/spaces/pages/actions/actions/quiz/context.rs` | Replace `use_query`→`use_loader` |
| `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/mod.rs` | Replace `use_query`→`use_loader` |
| `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/quiz_read_page.rs` | Replace `use_query_store`→context for ranking/score |
| `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/creator/overview_tab.rs` | Replace `use_query_store`→`ctx.quiz.restart()` |
| `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/creator/upload_tab.rs` | Replace `use_query_store`→`ctx.quiz.restart()` |
| `app/ratel/src/features/spaces/pages/actions/actions/discussion/views/main/creator/upload_tab.rs` | Replace `use_query_store`→pass loader prop |
| `app/ratel/src/features/spaces/pages/actions/actions/discussion/components/discussion_comments.rs` | Replace `use_query_store`→context for ranking/score |
| `app/ratel/src/features/spaces/pages/actions/actions/follow/views/main/viewer/mod.rs` | Replace `use_query_store`→context for ranking/score |
| `app/ratel/src/features/spaces/pages/apps/apps/panels/views/main/creator/mod.rs` | Replace `use_query`→`use_loader`, pass loader to children |
| `app/ratel/src/features/spaces/pages/apps/apps/panels/components/attribute_groups.rs` | Replace `use_query_store`→receive loader prop |
| `app/ratel/src/features/spaces/pages/apps/apps/panels/components/panels_table.rs` | Replace `use_query_store`→receive loader prop |
| `app/ratel/src/features/spaces/pages/apps/apps/panels/components/collective_panel.rs` | Replace `use_query_store`→receive loader prop |
| `app/ratel/src/features/spaces/pages/apps/apps/analyzes/views/analyze/poll/page.rs` | Replace `use_query`→`use_loader` |
| `app/ratel/src/features/spaces/space_common/components/space_nav/participation_verification_section.rs` | Replace `use_query_store`→`ctx.panel_requirements.restart()` |

### Deleted files

| File | Reason |
|------|--------|
| `app/ratel/src/common/query/mod.rs` | QueryStore, use_query, use_query_store no longer needed |
| `app/ratel/src/features/spaces/space_common/types/keys.rs` | Query key constants/functions no longer needed |

---

## Task 1: Extend SpaceContextProvider with shared loaders

Add `actions`, `ranking`, and `my_score` loaders to `SpaceContextProvider`. These are the cross-cutting loaders invalidated from many components.

**Files:**
- Modify: `app/ratel/src/features/spaces/space_common/providers/space_context_provider.rs`

- [ ] **Step 1: Add imports and new Loader fields**

In `space_context_provider.rs`, add the three new loaders:

```rust
use dioxus::fullstack::{Loader, Loading};

use crate::{
    features::spaces::space_common::{
        controllers::{get_user_role, SpaceResponse},
        hooks::*,
        *,
    },
    spaces::controllers::panel_requirements::PanelRequirementStatus,
};
// New imports:
use crate::features::activity::controllers::{
    get_my_score_handler, get_ranking_handler, MyScoreResponse, RankingEntryResponse,
};
use crate::features::spaces::pages::actions::{
    controllers::list_actions, types::SpaceActionSummary,
};

#[derive(Clone, Copy, DioxusController)]
pub struct SpaceContextProvider {
    pub role: Loader<SpaceUserRole>,
    pub space: Loader<SpaceResponse>,
    pub current_role: Memo<SpaceUserRole>,
    pub panel_requirements: Loader<Vec<PanelRequirementStatus>>,
    // New shared loaders:
    pub actions: Loader<Vec<SpaceActionSummary>>,
    pub ranking: Loader<ListResponse<RankingEntryResponse>>,
    pub my_score: Loader<MyScoreResponse>,
}
```

- [ ] **Step 2: Initialize new loaders in `init()`**

Add the three `use_loader` calls inside `SpaceContextProvider::init()`:

```rust
impl SpaceContextProvider {
    pub fn init(space_id: ReadSignal<SpacePartition>) -> crate::common::Result<Self, Loading> {
        let role = use_loader(move || async move { get_user_role(space_id()).await })?;
        let space = use_loader(move || async move { get_space(space_id()).await })?;
        let mut panel_requirements = use_loader(move || async move {
            crate::features::spaces::controllers::panel_requirements::get_panel_requirements(
                space_id(),
            )
            .await
        })?;
        // New loaders:
        let actions = use_loader(move || async move { list_actions(space_id()).await })?;
        let ranking = use_loader(move || async move {
            get_ranking_handler(space_id(), None).await
        })?;
        let my_score =
            use_loader(move || async move { get_my_score_handler(space_id()).await })?;

        // ... rest of init unchanged, add new fields to Self { ... }
        let srv = Self {
            role,
            space,
            current_role,
            panel_requirements,
            actions,
            ranking,
            my_score,
        };
        // ...
    }

    pub fn restart(&mut self) {
        self.role.restart();
        self.space.restart();
        self.panel_requirements.restart();
        self.actions.restart();
        self.ranking.restart();
        self.my_score.restart();
    }
    // ... toggle_role unchanged
}
```

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

Expected: May have warnings about unused fields (since consumers haven't switched yet), but should compile. If `RUSTFLAGS='-D warnings'` fails on unused fields, temporarily allow with `#[allow(dead_code)]` on the new fields — remove in Task 12.

---

## Task 2: Add hook functions for new shared loaders

Create convenience hooks that match the `use_space()` pattern.

**Files:**
- Modify: `app/ratel/src/features/spaces/space_common/hooks/mod.rs`
- Modify: `app/ratel/src/features/spaces/space_common/hooks/use_space.rs`

- [ ] **Step 1: Remove `use_space_query` from `use_space.rs`**

```rust
// BEFORE (use_space.rs):
use dioxus::fullstack::{Loader, Loading};
use crate::features::spaces::space_common::{
    controllers::{SpaceResponse, get_space},
    providers::use_space_context,
    types::space_key,
    *,
};

pub fn use_space_query(
    space_id: &SpacePartition,
) -> dioxus::prelude::Result<Loader<SpaceResponse>, Loading> {
    let key = space_key(space_id);
    use_query(&key, {
        let space_id = space_id.clone();
        move || get_space(space_id.clone())
    })
}

pub fn use_space() -> Loader<SpaceResponse> {
    let ctx = use_space_context();
    ctx.space
}

// AFTER (use_space.rs):
use dioxus::fullstack::Loader;
use crate::features::spaces::space_common::{
    controllers::SpaceResponse,
    providers::use_space_context,
};

pub fn use_space() -> Loader<SpaceResponse> {
    let ctx = use_space_context();
    ctx.space
}
```

- [ ] **Step 2: Add new hook functions to `hooks/mod.rs`**

Add hook functions for actions, ranking, my_score. Check current content of `hooks/mod.rs` and add the new hooks alongside existing ones.

```rust
// Add these functions (in a new file or inline in mod.rs):

pub fn use_actions() -> Loader<Vec<crate::features::spaces::pages::actions::types::SpaceActionSummary>> {
    let ctx = crate::features::spaces::space_common::providers::use_space_context();
    ctx.actions
}

pub fn use_ranking() -> Loader<ListResponse<crate::features::activity::controllers::RankingEntryResponse>> {
    let ctx = crate::features::spaces::space_common::providers::use_space_context();
    ctx.ranking
}

pub fn use_my_score() -> Loader<crate::features::activity::controllers::MyScoreResponse> {
    let ctx = crate::features::spaces::space_common::providers::use_space_context();
    ctx.my_score
}
```

- [ ] **Step 3: Remove unused `use_space_query` import from layout.rs**

In `app/ratel/src/features/spaces/layout.rs`, remove the line:

```rust
// REMOVE this line:
use crate::features::spaces::space_common::hooks::use_space_query;
```

- [ ] **Step 4: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

---

## Task 3: Update ranking widget and leaderboard panel

Replace `use_query` with context hooks for ranking and my_score.

**Files:**
- Modify: `app/ratel/src/features/activity/components/ranking_widget.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/leaderboard_panel/component.rs`

- [ ] **Step 1: Update `ranking_widget.rs`**

```rust
// BEFORE:
use crate::features::activity::controllers::{get_my_score_handler, get_ranking_handler};
use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;
use crate::features::spaces::space_common::types::{space_my_score_key, space_ranking_key};

#[component]
pub fn RankingWidget(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: ActivityTranslate = use_translate();
    let ranking_key = space_ranking_key(&space_id());
    let ranking_loader = use_query(&ranking_key, move || async move {
        get_ranking_handler(space_id(), None).await
    })?;
    let my_score_key = space_my_score_key(&space_id());
    let my_score_loader = use_query(&my_score_key, move || async move {
        get_my_score_handler(space_id()).await
    })?;
    let ranking = ranking_loader();
    let my_score = my_score_loader();
    // ...

// AFTER:
use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;
use crate::features::spaces::space_common::hooks::{use_my_score, use_ranking};

#[component]
pub fn RankingWidget(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: ActivityTranslate = use_translate();
    let ranking_loader = use_ranking();
    let my_score_loader = use_my_score();
    let ranking = ranking_loader();
    let my_score = my_score_loader();
    // ... rest unchanged
```

- [ ] **Step 2: Update `leaderboard_panel/component.rs`**

Same pattern — replace `use_query` calls with context hooks:

```rust
// BEFORE (in LeaderboardContent):
let ranking_key = space_ranking_key(&space_id());
let ranking_loader = use_query(&ranking_key, move || async move {
    get_ranking_handler(space_id(), None).await
})?;
let my_score_key = space_my_score_key(&space_id());
let my_score_loader = use_query(&my_score_key, move || async move {
    get_my_score_handler(space_id()).await
})?;

// AFTER:
let ranking_loader = use_ranking();
let my_score_loader = use_my_score();
```

Remove the unused imports: `get_ranking_handler`, `get_my_score_handler`, `space_ranking_key`, `space_my_score_key`.

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 4: Update action dashboard and candidate view

Replace `use_query` for actions with context hooks.

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_dashboard/component.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/component.rs`

- [ ] **Step 1: Update `action_dashboard/component.rs`**

```rust
// BEFORE:
let actions_key = space_page_actions_key(&space_id());
let actions_loader = use_query(&actions_key, move || list_actions(space_id()))?;
let actions = actions_loader();
// ...
let mut query = use_query_store();
// ...
// In use_effect:
query.invalidate(&actions_key);

// AFTER:
use crate::features::spaces::space_common::hooks::use_actions;
use crate::features::spaces::space_common::providers::use_space_context;

let mut actions_loader = use_actions();
let actions = actions_loader();
// ...
let mut space_ctx = use_space_context();
// ...
// In use_effect (replace query.invalidate(&actions_key)):
space_ctx.actions.restart();
```

Remove: `use_query_store`, `space_page_actions_key` import, `list_actions` import (if only used for `use_query`).

- [ ] **Step 2: Update `CandidateView` in `index/component.rs`**

```rust
// BEFORE:
fn CandidateView(space_id: ReadSignal<SpacePartition>) -> Element {
    use crate::features::spaces::space_common::types::space_page_actions_key;
    let key = space_page_actions_key(&space_id());
    let actions = use_query(&key, move || async move {
        crate::features::spaces::pages::actions::controllers::list_actions(space_id()).await
    })?;
    let actions = actions();
    // ...

// AFTER:
fn CandidateView(space_id: ReadSignal<SpacePartition>) -> Element {
    let actions_loader = crate::features::spaces::space_common::hooks::use_actions();
    let actions = actions_loader();
    // ...
```

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 5: Update arena overlay components (poll, quiz, discussion)

These live in `pages/index/action_pages/`. Each uses `use_query` for its own data AND `query.invalidate` for ranking/score/actions.

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/poll/component.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/quiz/component.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/discussion/component.rs`

- [ ] **Step 1: Update `poll/component.rs` (ActionPollViewer)**

```rust
// BEFORE:
let mut query = use_query_store();
let key = space_page_actions_poll_key(&space_id(), &poll_id());
let poll_loader = use_query(&key, move || get_poll(space_id(), poll_id()))?;
// ...
// In do_submit callback:
query.invalidate(&space_page_actions_poll_key(&space_id(), &poll_id()));
query.invalidate(&space_page_actions_key(&space_id()));
query.invalidate(&space_ranking_key(&space_id()));
query.invalidate(&space_my_score_key(&space_id()));

// AFTER:
use crate::features::spaces::space_common::providers::use_space_context;

let mut poll_loader = use_loader(move || get_poll(space_id(), poll_id()))?;
let mut space_ctx = use_space_context();
// ...
// In do_submit callback:
poll_loader.restart();
space_ctx.actions.restart();
space_ctx.ranking.restart();
space_ctx.my_score.restart();
```

Remove: `use_query_store`, `use_query`, `space_page_actions_poll_key`, `space_page_actions_key`, `space_ranking_key`, `space_my_score_key` imports.

- [ ] **Step 2: Update `quiz/component.rs` (QuizArenaPage)**

Same pattern:

```rust
// BEFORE:
let mut query = use_query_store();
let key = space_page_actions_quiz_key(&space_id(), &quiz_id());
let quiz_loader = use_query(&key, move || get_quiz(space_id(), quiz_id()))?;
// In on_submit:
query.invalidate(&keys);
query.invalidate(&space_ranking_key(&space_id()));
query.invalidate(&space_my_score_key(&space_id()));
query.invalidate(&space_page_actions_key(&space_id()));

// AFTER:
let mut quiz_loader = use_loader(move || get_quiz(space_id(), quiz_id()))?;
let mut space_ctx = use_space_context();
// In on_submit:
quiz_loader.restart();
space_ctx.ranking.restart();
space_ctx.my_score.restart();
space_ctx.actions.restart();
```

- [ ] **Step 3: Update `discussion/component.rs` (DiscussionArenaPage + CommentItem)**

`DiscussionArenaPage` loads disc, comments, members. `CommentItem` (child component in same file) also uses `use_query_store`.

```rust
// BEFORE (DiscussionArenaPage):
let mut query = use_query_store();
let disc_loader = use_query(&disc_key, move || { ... })?;
let comments_loader = use_query(&comments_key, move || { ... })?;
let members_loader = use_query(&members_key, move || { ... })?;
// In on_submit_comment:
query.invalidate(&keys);
query.invalidate(&space_page_actions_key(&space_id()));

// AFTER (DiscussionArenaPage):
let mut disc_loader = use_loader(move || get_discussion_detail(space_id(), discussion_id()))?;
let mut comments_loader = use_loader(move || list_comments(space_id(), discussion_id(), None))?;
let members_loader = use_loader(move || list_space_members(space_id(), None))?;
let mut space_ctx = use_space_context();
// In on_submit_comment:
comments_loader.restart();
space_ctx.actions.restart();
```

For `CommentItem` child: remove `use_query_store()`, pass `comments_loader: Loader<...>` as a prop.

```rust
// CommentItem gets comments_loader as prop:
#[component]
fn CommentItem(
    comment: DiscussionCommentResponse,
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    members: ReadSignal<Vec<MentionCandidate>>,
    comments_loader: Loader<ListResponse<DiscussionCommentResponse>>,
) -> Element {
    // ...
    // In on_like handler, replace:
    //   query.invalidate(&keys);
    // with:
    let mut comments_loader = comments_loader;
    comments_loader.restart();
    // In on_submit_reply:
    comments_loader.restart();
```

Update the CommentItem callsite to pass the new prop:

```rust
CommentItem {
    key: "{comment.sk}",
    comment: comment.clone(),
    space_id,
    discussion_id,
    can_comment,
    members,
    comments_loader,
}
```

- [ ] **Step 4: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 6: Update quiz Context and full quiz views

The quiz has an existing `Context` struct using `use_query`. Replace with `use_loader`. Child components use `use_query_store` to invalidate quiz data — replace with `ctx.quiz.restart()`.

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/quiz/context.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/mod.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/quiz_read_page.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/creator/overview_tab.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/quiz/views/main/creator/upload_tab.rs`

- [ ] **Step 1: Update `quiz/context.rs`**

```rust
// BEFORE:
let quiz_key = space_page_actions_quiz_key(&space_id(), &quiz_id());
let quiz = use_query(&quiz_key, { move || get_quiz(space_id(), quiz_id()) })?;
let answer_key = { ... };
let answer = use_query(&answer_key, { move || async move { ... } })?;

// AFTER:
let quiz = use_loader(move || get_quiz(space_id(), quiz_id()))?;
let answer = use_loader({
    move || async move {
        if role == SpaceUserRole::Creator {
            get_quiz_answer(space_id(), quiz_id()).await
        } else {
            Ok(QuizAnswerResponse::default())
        }
    }
})?;
```

Remove: `use crate::features::spaces::space_common::types::space_page_actions_quiz_key;`

- [ ] **Step 2: Update `quiz/views/main/mod.rs`**

```rust
// BEFORE:
let key = crate::features::spaces::space_common::types::space_page_actions_quiz_key(...);
let quiz_loader = use_query(&key, move || get_quiz(space_id(), poll_id()))?;

// AFTER:
let quiz_loader = use_loader(move || get_quiz(space_id(), quiz_id()))?;
```

- [ ] **Step 3: Update `quiz_read_page.rs`**

```rust
// BEFORE:
let mut query = use_query_store();
// In on_submit:
query.invalidate(&keys);
query.invalidate(&space_ranking_key(&space_id()));
query.invalidate(&space_my_score_key(&space_id()));

// AFTER:
let mut space_ctx = use_space_context();
// In on_submit:
ctx.quiz.restart();
ctx.answer.restart();
space_ctx.ranking.restart();
space_ctx.my_score.restart();
```

Remove: `use_query_store`, `space_ranking_key`, `space_my_score_key`, `space_page_actions_quiz_key` imports.

- [ ] **Step 4: Update `creator/overview_tab.rs`**

```rust
// BEFORE:
let mut query = use_query_store();
// In auto-save and on_save:
let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
query.invalidate(&keys);

// AFTER (ctx is available from use_space_quiz_context()):
// In auto-save and on_save:
let mut quiz_ctx = use_space_quiz_context();
quiz_ctx.quiz.restart();
```

Remove: `use_query_store`, `space_page_actions_quiz_key` imports.

- [ ] **Step 5: Update `creator/upload_tab.rs`**

Same pattern as overview_tab:

```rust
// BEFORE:
let mut query = use_query_store();
// query.invalidate(&keys);

// AFTER:
let mut quiz_ctx = use_space_quiz_context();
// quiz_ctx.quiz.restart();
```

- [ ] **Step 6: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 7: Update full poll views

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/poll/views/main/mod.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/poll/views/main/participant/content.rs`

- [ ] **Step 1: Update `poll/views/main/mod.rs`**

```rust
// BEFORE:
let key = crate::features::spaces::space_common::types::space_page_actions_poll_key(...);
let poll_loader = use_query(&key, move || get_poll(space_id(), poll_id()))?;

// AFTER:
let poll_loader = use_loader(move || get_poll(space_id(), poll_id()))?;
```

- [ ] **Step 2: Update `poll/views/main/participant/content.rs`**

```rust
// BEFORE:
let mut query = use_query_store();
let key = space_page_actions_poll_key(&space_id(), &poll_id());
let poll_loader = use_query(&key, { move || get_poll(space_id(), poll_id()) })?;
// In build_submit_response:
query.invalidate(&keys);
query.invalidate(&space_ranking_key(&space_id()));
query.invalidate(&space_my_score_key(&space_id()));

// AFTER:
let mut poll_loader = use_loader(move || get_poll(space_id(), poll_id()))?;
let mut space_ctx = use_space_context();
// In build_submit_response:
poll_loader.restart();
space_ctx.ranking.restart();
space_ctx.my_score.restart();
```

Remove: `use_query_store`, `space_page_actions_poll_key`, `space_ranking_key`, `space_my_score_key` imports.

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 8: Update discussion views and discussion_comments

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/views/main/creator/upload_tab.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/components/discussion_comments.rs`

- [ ] **Step 1: Update `discussion/views/main/creator/upload_tab.rs`**

This component invalidates the discussion key after file upload. It needs access to the discussion loader. Since this is a child of the discussion page, it can receive the loader as a prop or get it from a context.

Check how the parent renders this component and determine how to pass the discussion loader. If a context exists (`use_discussion_comment_context`), extend it. Otherwise, add a prop.

```rust
// BEFORE:
let mut query = use_query_store();
// query.invalidate(&keys);

// AFTER — simplest: accept a callback prop for refresh:
// Or use use_loader in the parent and pass down.
// Check the parent component to determine the approach.
```

The simplest approach: since the upload_tab modifies the discussion (updates files), call a server function and then use a `Callback<()>` prop (`on_refresh`) that the parent sets to `disc_loader.restart()`.

- [ ] **Step 2: Update `discussion_comments.rs`**

Replace `use_query_store` with SpaceContextProvider access for ranking/score:

```rust
// BEFORE:
use crate::common::query::use_query_store;
let mut query = use_query_store();
// ...
query.invalidate(&space_ranking_key(&space_id()));
query.invalidate(&space_my_score_key(&space_id()));

// AFTER:
use crate::features::spaces::space_common::providers::use_space_context;
let mut space_ctx = use_space_context();
// ...
space_ctx.ranking.restart();
space_ctx.my_score.restart();
```

This applies to ALL `query.invalidate` calls in this file (lines ~151, 154, 195, 196, 812, 813, 840, 841). Each `query.invalidate(&space_ranking_key(...))` becomes `space_ctx.ranking.restart()`. Each `query.invalidate(&space_my_score_key(...))` becomes `space_ctx.my_score.restart()`.

Note: the `ReplyInputBox` sub-component (around line 787) also has its own `let mut query = use_query_store();`. Replace that with `let mut space_ctx = use_space_context();` as well.

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 9: Update follow viewer and participation verification

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/follow/views/main/viewer/mod.rs`
- Modify: `app/ratel/src/features/spaces/space_common/components/space_nav/participation_verification_section.rs`

- [ ] **Step 1: Update `follow/views/main/viewer/mod.rs`**

```rust
// BEFORE:
use crate::common::query::use_query_store;
let mut query = use_query_store();
// In on_follow:
query.invalidate(&space_ranking_key(&space_id()));
query.invalidate(&space_my_score_key(&space_id()));
// In on_unfollow:
query.invalidate(&space_ranking_key(&space_id()));
query.invalidate(&space_my_score_key(&space_id()));

// AFTER:
use crate::features::spaces::space_common::providers::use_space_context;
let mut space_ctx = use_space_context();
// In on_follow:
space_ctx.ranking.restart();
space_ctx.my_score.restart();
// In on_unfollow:
space_ctx.ranking.restart();
space_ctx.my_score.restart();
```

Remove: `use crate::common::query::use_query_store;`, `space_ranking_key`, `space_my_score_key` imports.

- [ ] **Step 2: Update `participation_verification_section.rs`**

```rust
// BEFORE:
let mut query = use_query_store();
// ...
query.invalidate(&panel_requirements_key);

// AFTER:
let mut space_ctx = use_space_context();
// ...
space_ctx.panel_requirements.restart();
```

Remove: `use_query_store` usage. Remove `panel_requirements_key` variable (it was only used for invalidation).

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 10: Update panels page and children

The panels page loads panels with `use_query` and passes `panels_query_key` to children for invalidation. Replace: parent uses `use_loader`, children receive the `Loader` as a prop instead of the key.

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/apps/apps/panels/views/main/creator/mod.rs`
- Modify: `app/ratel/src/features/spaces/pages/apps/apps/panels/components/attribute_groups.rs`
- Modify: `app/ratel/src/features/spaces/pages/apps/apps/panels/components/panels_table.rs`
- Modify: `app/ratel/src/features/spaces/pages/apps/apps/panels/components/collective_panel.rs`

- [ ] **Step 1: Update `panels/views/main/creator/mod.rs` (PanelPage)**

```rust
// BEFORE:
let panels_query_key = panels_key(&space_id());
let panels_loader = use_query(&panels_query_key, { move || list_panels(space_id()) })?;

// AFTER:
let panels_loader = use_loader(move || list_panels(space_id()))?;
```

Then wherever `panels_query_key` is passed to children as a prop, replace with `panels_loader`:

```rust
// BEFORE:
AttributeGroupsSection { panels_query_key: panels_query_key.clone(), ... }
CollectivePanel { panels_query_key: panels_query_key.clone(), ... }

// AFTER:
AttributeGroupsSection { panels_loader, ... }
CollectivePanel { panels_loader, ... }
```

- [ ] **Step 2: Update child components' prop signatures**

For `attribute_groups.rs`, `panels_table.rs`, `collective_panel.rs`:

```rust
// BEFORE:
fn AttributeGroupsSection(
    // ...
    panels_query_key: Vec<String>,
) -> Element {
    let mut query = use_query_store();
    // ...
    query.invalidate(&panels_query_key)

// AFTER:
fn AttributeGroupsSection(
    // ...
    panels_loader: Loader<Vec<SpacePanelQuotaResponse>>,
) -> Element {
    // ...
    let mut panels_loader = panels_loader;
    panels_loader.restart();
```

Apply the same change to all three child component files. Remove `use_query_store` import from each.

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 11: Update standalone loaders (use_user, analyzes page)

**Files:**
- Modify: `app/ratel/src/features/spaces/hooks/use_user.rs`
- Modify: `app/ratel/src/features/spaces/pages/apps/apps/analyzes/views/analyze/poll/page.rs`

- [ ] **Step 1: Update `use_user.rs`**

```rust
// BEFORE:
use crate::features::spaces::controllers::user::get_user;
use crate::common::use_query;
use dioxus::prelude::*;
use crate::features::auth::models::user::User;
use std::collections::HashMap;

pub const USER_QUERY_KEY: &[&str] = &["User"];

#[track_caller]
pub fn use_user(
) -> dioxus::prelude::Result<dioxus_fullstack::Loader<Option<User>>, dioxus_fullstack::Loading> {
    use_query(USER_QUERY_KEY, get_user)
}

// AFTER:
use crate::features::spaces::controllers::user::get_user;
use dioxus::prelude::*;
use dioxus::fullstack::{Loader, Loading};
use crate::features::auth::models::user::User;

#[track_caller]
pub fn use_user() -> dioxus::prelude::Result<Loader<Option<User>>, Loading> {
    use_loader(get_user)
}
```

Remove: `use_query` import, `USER_QUERY_KEY` constant, `HashMap` import.

- [ ] **Step 2: Update `analyzes/views/analyze/poll/page.rs`**

```rust
// BEFORE:
use crate::common::use_query;
let panels_query = use_query(&panels_query_key, move || list_panels(space_id()))?;
let poll_query = use_query(&poll_key, move || get_poll(space_id(), poll_id()))?;
let result_query = use_query(&result_key, move || get_poll_result(space_id(), poll_id()))?;

// AFTER:
let panels_query = use_loader(move || list_panels(space_id()))?;
let poll_query = use_loader(move || get_poll(space_id(), poll_id()))?;
let result_query = use_loader(move || get_poll_result(space_id(), poll_id()))?;
```

Remove: `use crate::common::use_query;`, all key variable definitions (`panels_query_key`, `poll_key`, `result_key`), key function imports.

- [ ] **Step 3: Verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

---

## Task 12: Remove QueryStore and clean up

Delete the query module, remove all remaining references, and delete the query keys file.

**Files:**
- Delete: `app/ratel/src/common/query/mod.rs`
- Delete: `app/ratel/src/features/spaces/space_common/types/keys.rs`
- Modify: `app/ratel/src/common/mod.rs`
- Modify: `app/ratel/src/app.rs`
- Modify: `app/ratel/src/features/spaces/space_common/types/mod.rs`

- [ ] **Step 1: Remove `QueryStore` from `app.rs`**

```rust
// REMOVE this line from App component:
use_context_provider(QueryStore::new);
```

- [ ] **Step 2: Remove query module from `common/mod.rs`**

```rust
// REMOVE these two lines:
pub mod query;
pub use query::*;
```

- [ ] **Step 3: Delete `common/query/mod.rs`**

```bash
rm app/ratel/src/common/query/mod.rs
rmdir app/ratel/src/common/query/
```

- [ ] **Step 4: Remove keys.rs from `space_common/types/`**

Read `app/ratel/src/features/spaces/space_common/types/mod.rs` and remove the `keys` module declaration and re-export. Then delete the file:

```bash
rm app/ratel/src/features/spaces/space_common/types/keys.rs
```

- [ ] **Step 5: Remove any `#[allow(dead_code)]` added in Task 1**

If temporary `#[allow(dead_code)]` was added to SpaceContextProvider fields, remove it now.

- [ ] **Step 6: Grep for remaining references**

```bash
cd app/ratel && grep -r "use_query\b\|use_query_store\|QueryStore\|QueryKey\|space_key\|space_ranking_key\|space_my_score_key\|space_page_actions" src/ --include="*.rs" -l
```

Fix any remaining references found. Common leftover patterns:
- Unused `use` imports
- References to `QueryKey` type alias
- References to key functions from `keys.rs`

- [ ] **Step 7: Final full build verification**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

All three must pass clean (zero warnings).

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "refactor: remove QueryStore, replace use_query with use_loader + context pattern

Replace the global QueryStore pub/sub cache invalidation system with
Loader-in-Context pattern:
- Shared loaders (actions, ranking, my_score) added to SpaceContextProvider
- Page-specific loaders use use_loader directly
- Invalidation via loader.restart() instead of key-based query.invalidate()
- Delete common/query/mod.rs and space_common/types/keys.rs

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```
