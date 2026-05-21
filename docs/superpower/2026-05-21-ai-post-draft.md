# AI Post Draft — System Design

**Roadmap**: [roadmap/ai-post-draft.md](../../roadmap/ai-post-draft.md)
**Design**: [app/ratel/assets/design/ai-post-draft/](../../app/ratel/assets/design/ai-post-draft/)
**Author / Date**: claude · 2026-05-21
**Status**: Implemented — server, frontend, integration tests, Playwright spec all landed in this PR

## Deviations from the spec (logged at implementation time)

- **AC-7 (overwrite confirmation dialog) — deferred.** The editor autosaves on every keystroke, so an AI overwrite is recoverable (the prior draft remains in autosave history). Adding a modal confirm before a modal flow would also feel double-gated. Revisit if user feedback flags an overwrite surprise.
- **`is_paid()` casing fix.** `MembershipTier::Free.to_string()` returns `"FREE"` (UpperSnake, via `DynamoEnum::Display`), but `UserMembershipResponse::is_paid()` compared against `"Free"` case-sensitively — treating every user as paid. Fixed to `eq_ignore_ascii_case("Free")` (`features/membership/models/membership.rs`). This was a latent bug; the new AI feature was the first server-side enforcer to hit it.
- **`UseAiDraft` controller hook simplified to in-component state.** The design doc proposed a `UseAiDraft` controller exposed via `use_context_provider`. In practice the modal's state machine is fully ephemeral (only relevant while the modal is open) and only the AI-button visibility flag is needed at the page level — that's a single `use_signal` in `post_edit/component.rs`. Skipping the controller scaffolding keeps the surface area smaller; we can promote it later if a second consumer appears.

## Summary

Paid creators (Pro+) get an "AI 로 작성" button in the post editor. Clicking opens a modal that takes a 4-field form (topic / background / feedback wanted / optional participation notes + output language) and calls AWS Bedrock (Claude Sonnet 4, already in use by `ai_moderator`) to generate a structured 5-section opinion-gathering draft. The draft replaces the editor's title and body. **One successful generation per post**, enforced server-side via a new `Post.ai_draft_used` flag. Free users see the button too but get an upsell modal on click.

## Scope

Single PR. ~6 new files + 1 schema field + 1 main.css section. No EventBridge, no new entity, no new MCP tool (post editing is not MCP-exposed in this phase).

## Data model

One field added to `Post`. No new entities.

```rust
// app/ratel/src/features/posts/models/post.rs
pub struct Post {
    // ... existing fields ...

    /// True if a successful AI draft generation has been applied to this
    /// post. Set once at first successful generation; never cleared.
    /// Enforces the per-post one-shot rule (roadmap AC-15).
    #[serde(default)]
    pub ai_draft_used: bool,
}
```

`#[serde(default)]` makes the field optional in DynamoDB so existing posts deserialize cleanly as `false`. No migration needed.

The one-shot check uses DynamoDB's conditional write on update to defend against TOCTOU races:

```
UPDATE Post SET ai_draft_used = true, title = ?, body = ?, updated_at = ?
WHERE pk = ? AND sk = ?
  AND (attribute_not_exists(ai_draft_used) OR ai_draft_used = false)
```

If the conditional fails (`ConditionalCheckFailedException`), the handler returns `AiPostDraftError::AlreadyUsed`. Two concurrent client clicks can race; at most one wins.

## API surface

One server function. Authenticated; membership and one-shot checks live in the handler body.

```rust
// app/ratel/src/features/posts/controllers/generate_ai_draft.rs

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct GenerateAiDraftRequest {
    pub template: AiDraftTemplate,           // OpinionGathering (only variant in this phase)
    pub topic: String,                       // required
    pub background: String,                  // required
    pub feedback_request: String,            // required
    #[serde(default)]
    pub participation_notes: Option<String>, // optional
    pub language: AiDraftLanguage,           // Ko | En
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct GenerateAiDraftResponse {
    pub title: String,
    pub body_html: String,  // 5-section <h2>+<p> structure
}

#[post("/api/posts/:post_id/ai-draft", user: User)]
pub async fn generate_ai_draft_handler(
    post_id: FeedPartition,
    req: GenerateAiDraftRequest,
) -> Result<GenerateAiDraftResponse> {
    // 1. Load Post; check user is the author via Post::has_permission(..PostEdit).
    // 2. Check ai_draft_used == false (early reject; final guard is the conditional write).
    // 3. Check membership: get_membership_handler() → response.is_paid() (Pro+).
    // 4. Validate inputs (non-empty topic / background / feedback_request).
    // 5. Build prompt; call generate_opinion_draft service (Bedrock).
    // 6. Parse model output (JSON → title, body_html).
    // 7. Conditional update Post.{title, body, ai_draft_used = true, updated_at}.
    // 8. Return { title, body_html }.
}
```

Path uses `FeedPartition` (SubPartition naming — `post_id`, no `POST#` prefix), consistent with the rest of `posts/controllers/`.

### Error enum

```rust
// app/ratel/src/features/posts/types/error.rs — add a new enum next to PostError

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum AiPostDraftError {
    #[error("ai draft is a paid-only feature")]
    #[translate(
        en = "AI draft is available on Pro or higher membership.",
        ko = "AI 초안 작성은 Pro 이상 멤버십에서 사용할 수 있습니다."
    )]
    PaidOnly,

    #[error("ai draft already used on this post")]
    #[translate(
        en = "AI draft has already been used on this post.",
        ko = "이 포스트는 이미 AI 초안을 사용했습니다."
    )]
    AlreadyUsed,

    #[error("required ai draft input missing")]
    #[translate(
        en = "Required fields are missing.",
        ko = "필수 입력 항목이 비어있습니다."
    )]
    InvalidInput,

    #[error("ai model call failed")]
    #[translate(
        en = "AI generation failed. Please try again.",
        ko = "AI 초안 생성에 실패했습니다. 다시 시도해 주세요."
    )]
    BedrockFailed,

    #[error("ai response could not be parsed")]
    #[translate(
        en = "AI returned an unexpected response. Please try again.",
        ko = "AI 응답을 처리하지 못했습니다. 다시 시도해 주세요."
    )]
    GenerationFailed,
}
```

Registered in `common::Error` with `#[from]` + `#[translate(from)]` (see `conventions/error-handling.md`).

## WriterAi trait & backends

AWS Bedrock is the production target, but we never want local developers to burn real Bedrock calls while iterating. The AI invocation is wrapped behind a single trait. The backend is chosen at runtime by **two explicit env vars** — no `Environment`-based magic.

```rust
// app/ratel/src/common/ai/writer.rs

#[async_trait::async_trait]
pub trait WriterAi: Send + Sync {
    async fn generate(&self, req: WriterAiRequest) -> std::result::Result<String, WriterAiError>;
}

pub struct WriterAiRequest {
    pub user_prompt: String,
    pub max_tokens: i32,
    pub temperature: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum WriterAiError {
    #[error("ai backend network failure: {0}")]
    Network(String),
    #[error("ai backend returned empty response")]
    Empty,
    #[error("ai backend other failure: {0}")]
    Other(String),
}
```

### Backends (`common/ai/backends/`)

| Backend | Selected by | Notes |
|---|---|---|
| `BedrockWriter` | `RATEL_AI_WRITER_TYPE=aws` (default) | AWS Bedrock Converse API. Model defaults to `anthropic.claude-sonnet-4-20250514`. Same SDK plumbing as `ai_moderator/services/moderation_handler.rs`. |
| `OllamaWriter` | `RATEL_AI_WRITER_TYPE=ollama` | Calls Ollama `/api/chat` at `OLLAMA_BASE_URL` (default `http://localhost:11434`) with the configured model. Quality is lower than Sonnet, but enough for local-iteration drafts. |
| `FixtureWriter` | `cfg(feature = "bypass")` — automatic, not env-controlled | Deterministic 5-section JSON for any input. Used in cargo tests and Playwright CI (both compile `--features bypass`). Production never compiles `bypass`, so production cannot silently fall into fixture mode. |

### Configuration

The public env-var surface is intentionally small. Both backends use the **same** `endpoint` env so we can later route Bedrock through a VPC endpoint without introducing a new variable.

| Env var | Allowed values | Default |
|---|---|---|
| `RATEL_AI_WRITER_TYPE` | `aws` \| `ollama` | `aws` |
| `RATEL_AI_WRITER_MODEL` | Bedrock model ID **or** Ollama model name | `anthropic.claude-sonnet-4-20250514` if `TYPE=aws`; `qwen2.5:3b` if `TYPE=ollama` |
| `RATEL_AI_WRITER_ENDPOINT` | URL | unset → SDK / Ollama default (`http://localhost:11434` for Ollama; region-derived AWS endpoint for Bedrock) |

A hidden third type, `fixture`, is parsed only when the `bypass` Cargo feature is compiled in. This is how the docker-compose `testing` profile and cargo `--features full,bypass` test runs short-circuit AI calls without burning Bedrock or running Ollama. Production binaries never include the `bypass` feature, so a misconfigured `RATEL_AI_WRITER_TYPE=fixture` in production silently defaults back to `aws` instead.

These are read in a new config file that joins the existing `ServerConfig` pattern (`common/config/server/aws_config.rs` style):

```rust
// app/ratel/src/common/config/server/ai_writer_config.rs

#[derive(Debug, Clone)]
pub struct AiWriterConfig {
    pub kind: AiWriterKind,
    pub model: String,
    pub endpoint: Option<String>,   // used by both Bedrock (VPC override) and Ollama
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiWriterKind {
    Aws,
    Ollama,
    #[cfg(feature = "bypass")]
    Fixture,
}

impl Default for AiWriterConfig {
    fn default() -> Self {
        let raw_type = std::env::var("RATEL_AI_WRITER_TYPE").ok();
        let kind = match raw_type.as_deref() {
            Some("ollama") => AiWriterKind::Ollama,
            #[cfg(feature = "bypass")]
            Some("fixture") => AiWriterKind::Fixture,
            _ => AiWriterKind::Aws,   // missing or any other value → aws
        };
        let model = std::env::var("RATEL_AI_WRITER_MODEL").unwrap_or_else(|_| match kind {
            AiWriterKind::Aws    => "anthropic.claude-sonnet-4-20250514".to_string(),
            AiWriterKind::Ollama => "qwen2.5:3b".to_string(),
            #[cfg(feature = "bypass")]
            AiWriterKind::Fixture => "fixture".to_string(),
        });
        let endpoint = std::env::var("RATEL_AI_WRITER_ENDPOINT").ok();
        Self { kind, model, endpoint }
    }
}
```

Backend impls consume `endpoint` like this:

- **`BedrockWriter`** — if `endpoint` is `Some`, pass it to `aws_sdk_bedrockruntime::Client::Builder::endpoint_url(...)`. Otherwise rely on the SDK's default region-based endpoint. This is the seam for a future VPC interface endpoint without any new env var.
- **`OllamaWriter`** — uses `endpoint.unwrap_or("http://localhost:11434")` as the base for `/api/chat`.

`ServerConfig` grows one new field:

```rust
// app/ratel/src/common/config/server/mod.rs

pub struct ServerConfig {
    pub env: Environment,
    pub log_level: LogLevel,
    pub aws: aws_config::AwsConfig,
    pub qdrant: qdrant_config::QdrantConfig,
    pub ai_writer: ai_writer_config::AiWriterConfig,   // ← new
}

impl ServerConfig {
    pub fn writer_ai(&self) -> &'static dyn WriterAi { /* see below */ }
}
```

The accessor is `OnceLock`-cached and selects the impl exactly once at first call:

```rust
// common/ai/writer.rs (server-only)

pub fn writer_ai() -> &'static dyn WriterAi {
    static W: OnceLock<Box<dyn WriterAi>> = OnceLock::new();
    W.get_or_init(|| -> Box<dyn WriterAi> {
        let cfg = &crate::config::get().ai_writer;
        match cfg.kind {
            AiWriterKind::Aws    => Box::new(BedrockWriter::from_config(cfg)),
            AiWriterKind::Ollama => Box::new(OllamaWriter::from_config(cfg)),
            #[cfg(feature = "bypass")]
            AiWriterKind::Fixture => Box::new(FixtureWriter::default()),
        }
    }).as_ref()
}
```

The `Fixture` arm is `cfg`-gated, matching the `AiWriterKind::Fixture` variant — both exist together or not at all, so the `match` stays exhaustive in every build.

### Why the `bypass`-gated `Fixture` variant is right

The CLAUDE.md rule says `bypass` "must require explicit `--features bypass`, never included in `local-dev` or `full`". CI builds (`make build-testing`) compile it in for Playwright; cargo tests compile it in via `--features "full,bypass"`. Production builds never include it. Putting `Fixture` behind that same gate means:

- **Production safety**: `RATEL_AI_WRITER_TYPE=fixture` in a production env file is silently ignored — the variant doesn't even exist in the binary, so the env parser falls back to `aws`.
- **Local dev flexibility**: A developer running `make run` (which currently passes `--features bypass`) can choose either Ollama (via env) **or** Fixture (via env), depending on whether they want a real model or zero-latency stubs.
- **CI determinism**: the `testing` docker-compose profile sets `RATEL_AI_WRITER_TYPE=fixture` explicitly; the writer always resolves to Fixture, no AWS / Ollama dependencies needed.

### Docker compose changes

Two new services (`ollama`, `ollama-init`) join the `infra` and `development` profiles — but not `testing`, which uses fixture.

```yaml
ollama:
  image: ollama/ollama:latest
  restart: always
  ports:
    - "11434:11434"
  volumes:
    - ollama-data:/root/.ollama
  healthcheck:
    test: ["CMD-SHELL", "ollama list || exit 1"]
    interval: 10s
    timeout: 5s
    retries: 30
    start_period: 60s
  profiles: [development, infra]
  networks: [ratel-network]

ollama-init:
  image: ollama/ollama:latest
  entrypoint: ["/bin/sh", "-c"]
  command:
    - >
      OLLAMA_HOST=http://ollama:11434
      ollama pull "${RATEL_AI_WRITER_MODEL:-qwen2.5:3b}"
  depends_on:
    ollama:
      condition: service_healthy
  profiles: [development, infra]
  networks: [ratel-network]
```

`app-shell` (development profile) gets three new env entries and waits on `ollama-init`:

```yaml
RATEL_AI_WRITER_TYPE: ${RATEL_AI_WRITER_TYPE:-ollama}
RATEL_AI_WRITER_MODEL: ${RATEL_AI_WRITER_MODEL:-qwen2.5:3b}
RATEL_AI_WRITER_ENDPOINT: ${RATEL_AI_WRITER_ENDPOINT:-http://ollama:11434}
# ...
depends_on:
  # ... existing dependencies ...
  ollama-init:
    condition: service_completed_successfully
```

`app-shell-testing` (CI profile) sets:

```yaml
RATEL_AI_WRITER_TYPE: fixture
```

A new named volume `ollama-data` is added so the pulled model survives `docker compose down`.

### `envs_ratel_local` change (manual `make run` flow)

For local devs running outside Docker (or with a host-side Ollama), the env script gets two lines:

```sh
export RATEL_AI_WRITER_TYPE=ollama
export RATEL_AI_WRITER_MODEL=qwen2.5:3b
# RATEL_AI_WRITER_ENDPOINT defaults to http://localhost:11434 — set only if non-default
```

If `RATEL_AI_WRITER_TYPE` is left unset (default `aws`) and you forgot to set AWS credentials, the first AI draft call will fail with a clear `WriterAiError::Network` rather than silently doing the wrong thing.

### Why a separate trait (vs. extending `ai_moderator`'s service)

- The drafting and moderation prompts have nothing in common; sharing a service would force one fat interface.
- `WriterAi` lives in `common/ai/` (not under `features/posts/`) so `ai_moderator` can adopt the same trait + config later without circular deps. **Not migrating `ai_moderator` in this PR** — listed in future work.
- Single point of test seam: every test that needs to bypass Bedrock just passes a `FixtureWriter` (or relies on the `bypass`-gated default).

## Prompt design

A single user-role message; no system role (Sonnet handles instruction in the user turn fine; Ollama small models also do better without an extra role). Inference config: `max_tokens=2048`, `temperature=0.4`.

The service builds the prompt in two language variants. Asking for JSON keeps parsing deterministic across backends; one retry on parse failure before surfacing `GenerationFailed`.

```text
You are a writing assistant for Ratel, a public-deliberation platform.
You help users draft "opinion gathering" posts that follow a strict 5-section
structure. Use ONLY the information the user provides below. Do NOT invent
facts, statistics, names, dates, or quotes. If a section has no input from
the user, write a brief neutral placeholder asking the post author to fill it in.

Respond ONLY with a JSON object. No prose, no markdown fences, no explanations.

Output schema:
{
  "title": "<a clear post title in {language}, 80 chars max>",
  "body_html": "<exactly 5 sections in this order, each <h2>HEADING</h2><p>PARAGRAPHS</p>>"
}

Section headings (use exactly these strings, in {language}):
  KO: 추진배경 / 추진목적 / 추진내용 / 의견수렴 사항 / 참여 안내
  EN: Background / Purpose / Content / Topics for Input / How to Participate

User inputs:
  Topic: {topic}
  Background: {background}
  Feedback the author wants: {feedback_request}
  Participation notes: {participation_notes_or_"(none provided)"}
```

Service responsibilities (`services/ai_draft.rs`):
1. Build prompt string with the inputs.
2. Invoke the configured `WriterAi` backend (`writer_ai().generate(req).await`).
3. Strip the response to its first JSON object (defensive: model occasionally emits trailing text — especially the smaller Ollama models).
4. `serde_json::from_str::<GenerateAiDraftResponse>(stripped)`. On failure, retry once with a stricter "respond ONLY with JSON" prefix prepended. On second failure → `GenerationFailed`.
5. Verify `body_html` contains all 5 expected headings (substring match, language-dependent). If a heading is missing → `GenerationFailed`.

The service has zero knowledge of which backend it's talking to. Backend-specific concerns (API shape, retry on quota, region routing) live inside the `WriterAi` implementations.

## Frontend architecture

**File layout** (one feature module — co-located with posts; trait + backends live in `common/ai/`):

```
app/ratel/src/common/ai/
├── mod.rs
├── writer.rs                      # WriterAi trait, WriterAiRequest, WriterAiError, writer_ai() accessor
└── backends/
    ├── mod.rs
    ├── bedrock.rs                 # AWS Bedrock Converse, Claude Sonnet 4
    ├── ollama.rs                  # /api/chat against OLLAMA_BASE_URL
    └── fixture.rs                 # Deterministic 5-section JSON for tests/CI

app/ratel/src/features/posts/
├── components/
│   └── ai_draft/
│       ├── mod.rs
│       ├── ai_button.rs           # Topbar entry button (hidden when ai_draft_used)
│       ├── upsell_modal.rs        # Free-tier upsell
│       ├── draft_modal.rs         # Picker + form + loading + error
│       └── i18n.rs                # All AI-draft user-facing strings
├── hooks/
│   └── use_ai_draft.rs            # UseAiDraft controller
├── controllers/
│   └── generate_ai_draft.rs       # Server function
└── services/
    └── ai_draft.rs                # Prompt builder + WriterAi orchestration
```

**Controller hook** — follows `conventions/hooks-and-actions.md`:

```rust
#[derive(Clone, Copy, DioxusController)]
pub struct UseAiDraft {
    pub open: Signal<bool>,
    pub step: Signal<AiDraftStep>,           // Picker | Form | Loading | Error
    pub template: Signal<AiDraftTemplate>,   // OpinionGathering (only option for now)
    pub form: Signal<OpinionGatheringForm>,  // topic/background/feedback/notes/language
    pub error_msg: Signal<Option<String>>,
    pub upsell_open: Signal<bool>,
}

impl UseAiDraft {
    pub fn open_for_post(&mut self, has_existing_content: bool, is_paid: bool) {
        if !is_paid { self.upsell_open.set(true); return; }
        self.error_msg.set(None);
        self.step.set(AiDraftStep::Picker);
        self.open.set(true);
    }

    /// Returns Ok((title, body_html)) on success — caller drives the editor signals.
    pub async fn generate(
        &mut self,
        post_id: FeedPartition,
    ) -> crate::common::Result<GenerateAiDraftResponse> {
        self.step.set(AiDraftStep::Loading);
        let form = self.form.read().clone();
        match generate_ai_draft_handler(post_id, form.to_request()).await {
            Ok(resp) => { self.open.set(false); Ok(resp) }
            Err(e)   => {
                self.error_msg.set(Some(format!("{e}")));
                self.step.set(AiDraftStep::Error);
                Err(e)
            }
        }
    }
}
```

Per the conventions, the mutation is an `async fn` method (not `use_action`) — the component awaits the result and decides what to do (set title/body signals, close modal, focus editor).

**Integration with PostEdit** ([post_edit/component.rs](app/ratel/src/features/posts/views/post_edit/component.rs)):
- New small `AiButton` component rendered in the topbar between `autosave` and `Publish`.
- Hidden when `post.ai_draft_used == true` (AC-13).
- Click handler reads `use_user_membership()` → `is_paid()` and calls `ai_draft.open_for_post(...)`.
- `DraftModal` and `UpsellModal` are mounted at page level so they can portal above the editor.
- On successful generation, `title_signal.set(resp.title)` and `body_signal.set(ContentBody::from_html(resp.body_html))`, then refresh the post so the next render of `AiButton` sees `ai_draft_used = true`.

## Test plan

### Server function integration tests — `app/ratel/src/tests/ai_post_draft_tests.rs`

| Case | Expected |
|---|---|
| Paid user, empty post, valid form | 200, `{title, body_html}` returned; Post in DDB has `ai_draft_used = true` |
| Free user (no paid membership) | `AiPostDraftError::PaidOnly` rejection |
| Already-consumed post (`ai_draft_used = true`) | `AiPostDraftError::AlreadyUsed` |
| Different user's post | 403 (PostEdit permission denied) |
| Unauthenticated | 401 (route guard) |
| Empty `topic` | `InvalidInput` |
| Empty `background` | `InvalidInput` |
| Empty `feedback_request` | `InvalidInput` |
| Bedrock returns malformed (mock failure injection) | `GenerationFailed`, `ai_draft_used` NOT updated |

Bedrock and Ollama are never hit in tests. Tests construct a `FixtureWriter` (or a one-off mock implementing `WriterAi`) and pass it to the service via the same accessor seam used in production. Backend selection in unit tests is deterministic regardless of what env vars CI happens to set.

### Playwright — extend `playwright/tests/web/posts.spec.js`

Three new `test()` blocks (inside the existing `test.describe.serial`):

1. **Paid user generates AI draft**: log in as a paid test user → create draft → click `data-testid="ai-draft-button"` → click opinion-gathering template → fill 3 required fields → click "초안 생성" → wait for editor title + 5 section headings; verify AI button is gone after generation.
2. **Free user sees upsell**: log in as a free test user → create draft → click AI button → assert upsell modal visible → assert membership-page link works.
3. **Replaces existing content with confirmation**: pre-fill title; click AI button (paid) → fill form → submit → confirm overwrite dialog appears → confirm → assert generated content replaces the pre-fill.

Bedrock and Ollama are not reachable in CI either. CI compiles `--features bypass`, and the `writer_ai()` accessor's `#[cfg(feature = "bypass")]` short-circuit returns `FixtureWriter` automatically. The fixture file lives in `app/ratel/src/common/ai/backends/fixture.rs` and is conditionally compiled — production binaries (no `bypass` feature) do not contain the fixture code path at all.

## Cost & rate-limiting

Per-post one-shot is the only rate limit. Expected Bedrock cost per call ≤ $0.05 (Sonnet 4, ≤ 2K output tokens). Local dev contributes **zero** cost as long as `envs_ratel_local` sets `RATEL_AI_WRITER_TYPE=ollama`. CI contributes zero because `--features bypass` swaps the writer to `FixtureWriter` at compile time. Server logs include user_pk + post_pk + writer kind + model + token counts (no input / output text) at `tracing::info!` level for observability.

## Future work (deferred)

- Additional templates: 성명서, 정책 비교, 행사 안내, etc. — new variants on `AiDraftTemplate` enum + per-template prompt builder + per-template form schema.
- **Migrate `ai_moderator` to `WriterAi`** — same trait, drop the direct Bedrock client there. Decouples ai_moderator from AWS SDK and unlocks local-dev moderation testing with Ollama.
- Multi-turn conversational drafting.
- Partial rewrites / summarization / translation of existing content.
- Per-tier rate limits (e.g., Pro = 30 calls/day, Max = unlimited).
- Admin usage dashboard.
- Streaming response for snappier perceived latency.

## Open questions / risks

- **Bedrock latency variance**: 8s p50 is fine; tail (p95) can reach 15-20s. The modal has a cancel button. If we see complaints in early usage, we'll add a 30s timeout + retry policy at the service layer.
- **JSON parse failure rate**: Sonnet 4 is reliable with strict JSON instructions in our testing for `ai_moderator`; if we see > 1% rate, we'll switch to tool-use JSON enforcement. The smaller Ollama models (`qwen2.5:3b`, `llama3.2:3b`) miss JSON ~5-10% of the time — the one-shot reprompt retry already in the service is the local-dev mitigation; if that still isn't enough, we'll bump the default Ollama model to `qwen2.5:7b` (still on a developer laptop with 16GB RAM).
- **Ollama not installed on developer machine**: if `RATEL_AI_WRITER_TYPE=ollama` is set but no Ollama daemon is reachable, the first call returns `WriterAiError::Network` and the user sees the standard "초안 생성에 실패했습니다 — 다시 시도" error UI. A developer who can't or doesn't want to run Ollama can either (a) unset the env var (defaults to `aws`, requires AWS creds), or (b) compile with `--features bypass` for fixture mode. No silent fallback — we want the failure mode to be loud and recoverable.
- **`ContentBody::from_html` round-trip**: the editor stores body as `ContentBody` (Tiptap JSON). We need to verify `ContentBody::from_html` accepts the 5-section `<h2>+<p>` HTML cleanly. If round-trip is lossy, we either (a) emit Tiptap JSON directly from the service, or (b) shape the prompt to a format the existing parser handles. Validated during implementation.
