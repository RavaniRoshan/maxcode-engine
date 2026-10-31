# Outbound network audit

Date: 2026-10-31
Scope: `maxcode-rs/**` Rust source, excluding tests and fixtures unless noted.
Method: source search for `http://`, `https://`, `wss://`, `reqwest`, telemetry,
analytics, update, login, and auth code paths, followed by reading the
production call sites. This is an audit of distinct destination families, not a
count of every HTTP call site; provider endpoints are user-configured and are
called once per model request.

## Defaults changed in this audit

Three automatic, non-essential behaviours are now off by default. Core `exec`
does not depend on any of them.

| Behaviour | Before | After | Where |
|---|---|---|---|
| Startup update check | on (`unwrap_or(true)`) | off (`unwrap_or(false)`) | `maxcode-rs/core/src/config/mod.rs:4271` |
| TUI analytics consent | on (`true`) | off (`false`) | `maxcode-rs/tui/src/startup_orchestration.rs:673` |
| Analytics event delivery | enabled when config was absent (`!= Some(false)`) | enabled only when explicitly `Some(true)` | `maxcode-rs/analytics/src/client.rs:314` |
| Default metrics exporter | Statsig | `none` | `maxcode-rs/config/src/types.rs:682` |
| Announcement-tip feed | fetched on TUI prewarm | disabled (`ANNOUNCEMENT_TIPS_ENABLED = false`) | `maxcode-rs/tui/src/tooltips.rs:16-20,181` |

Users can opt back in with `check_for_update_on_startup = true`,
`[analytics] enabled = true`, and `[otel] metrics_exporter = "statsig"` or a
custom OTLP endpoint. These are configuration choices, not hidden defaults.

## Automatic or default-on destinations

| Destination | File:line | Purpose | Default | Action |
|---|---|---|---|---|
| `https://ab.chatgpt.com/otlp/v1/metrics` | `maxcode-rs/otel/src/config.rs:9` | Statsig OTLP metrics export; hard-coded client key at line 11 | **Now off** (`metrics_exporter` defaults to `none`); explicit `statsig` opt-in | Disabled by default; key/endpoint flagged in `docs/LICENSE-REVIEW.md` |
| `https://chatgpt.com/backend-api/maxcode/analytics-events/events` | `maxcode-rs/analytics/src/client.rs:154` | Analytics event POST, base URL from `chatgpt_base_url` | **Now off**; enabled only with `[analytics] enabled = true` | Disabled by default |
| GitHub `openai/maxcode` latest release | `maxcode-rs/cli/src/doctor/updates.rs:36`, `maxcode-rs/tui/src/updates.rs:62` | Version freshness check | Startup check now off; `maxcode doctor`/`maxcode update` remain explicit | Disabled startup check; explicit commands left untouched |
| `https://formulae.brew.sh/api/cask/maxcode.json` | `maxcode-rs/cli/src/doctor/updates.rs:37`, `maxcode-rs/tui/src/updates.rs:61` | Homebrew update check | Same as above | Disabled startup check; explicit commands left untouched |
| `https://persistent.oaistatic.com/...` | `maxcode-rs/cli/src/doctor/updates.rs:39-46` | Desktop app update appcast | Only `maxcode doctor` on macOS/Windows | Left untouched; explicit command |
| `https://chatgpt.com/backend-api/wham/app/appcast` | `maxcode-rs/cli/src/doctor/updates.rs:43` | Desktop backend appcast | Only `maxcode doctor` on macOS | Left untouched; explicit command |
| `https://registry.npmjs.org/@openai%2fmaxcode` | `maxcode-rs/tui/src/npm_registry.rs:5` | npm package metadata for update checks | Debug builds exclude it; release update path | Left untouched; tied to explicit/startup update check |
| `https://raw.githubusercontent.com/openai/maxcode/main/announcement_tip.toml` | `maxcode-rs/tui/src/tooltips.rs:16` | Announcement tooltip feed | **Now off**; prewarm returns without fetching | Disabled; URL remains for audit until removed |
| `https://o33249.ingest.us.sentry.io/4510195390611458` | `maxcode-rs/feedback/src/lib.rs:63` | Feedback envelope upload (Sentry DSN) | Only after explicit feedback submission; `feedback_enabled` defaults true, but no automatic upload | Left untouched; flagged for review |
| `https://chatgpt.com/backend-api` | `maxcode-rs/model-provider-info/src/lib.rs:82`; `backend-client/src/client.rs:231` | ChatGPT backend for login-backed sessions, usage, profiles, remote control | Only for ChatGPT-auth sessions; not API-key/custom-provider exec | Left untouched; provider functionality |
| `https://auth.openai.com/api/accounts` | `maxcode-rs/agent-identity/src/lib.rs:41-65` | Agent identity / login token endpoints | Only login/agent-identity flows | Left untouched; opt-in login |
| `https://chatgpt.com/.../agent-identity` | `maxcode-rs/agent-identity/src/lib.rs:41-65` | Agent identity registration/JWKS | Only login/agent-identity flows | Left untouched; opt-in login |
| `https://api.openai.com/v1` | `maxcode-rs/model-provider-info/src/lib.rs:445`; `maxcode-api/src/provider.rs:55` | Default OpenAI provider base URL | Only when the OpenAI provider is selected | Left untouched; provider functionality |
| `https://api.openai.com/v1` (realtime) | `maxcode-rs/maxcode-api/src/endpoint/realtime_websocket/methods.rs:63` | Realtime WebSocket base | Only realtime sessions | Left untouched; explicit feature |
| `wss://api.openai.com/v1/realtime` | same file, lines 2340/2455 | Realtime WebSocket | Only realtime sessions | Left untouched; explicit feature |
| `wss://chatgpt.com/backend-api/wham/remote/control/server` | `maxcode-rs/app-server-transport/src/transport/remote_control/protocol.rs:303` | Remote-control pairing/refresh | Only `maxcode remote-control` | Left untouched; explicit command |
| `https://api.githubcopilot.com` | `maxcode-rs/maxcode-mcp/src/connection_manager/startup.rs:104` | GitHub Copilot MCP integration | Only configured MCP servers | Left untouched; user-configured |
| `https://gov.chatgpt.com` | `maxcode-rs/maxcode-mcp/src/mcp/mod.rs:383` | Government-cloud MCP path | Only government-cloud configuration | Left untouched; user-configured |
| `https://cdn.jsdelivr.net`, `cdnjs.cloudflare.com`, `esm.sh`, `fonts.bunny.net`, `fonts.googleapis.com`, `fonts.gstatic.com`, `unpkg.com` | `maxcode-rs/tui/src/inline_visualization/viewer.rs:16` | External assets for inline HTML visualization | Only when inline visualization renders remote content | Left untouched; document for privacy review |
| `https://mcp.mercadopago.com`, `api.robinhood.com`, etc. | `maxcode-rs/rmcp-client/src/oauth/issuer_binding.rs` | OAuth issuer bindings for MCP servers | Only configured MCP servers | Left untouched; user-configured |
| `https://learn.microsoft.com`, `developers.openai.com`, `platform.openai.com`, `help.openai.com`, `openai.com` | TUI help/notice links; e.g. `tui/src/history_cell/notices.rs` | Documentation links shown to the user | No request unless the user follows a link | Left untouched; third-party docs |
| `https://www.maxcode.com/models/...` | `maxcode-rs/tui/src/model_migration.rs:470` | Placeholder model migration links | Displayed only in migration messages | Replaced in the branding step with the project URL |
| `https://openai.com/maxcode/`, `noreply@openai.com` | `maxcode-rs/ext/git-attribution/src/world_state.rs` | Git commit/PR attribution | Only when git attribution is enabled | Replaced in the branding step |

## User-initiated or opt-in destinations

These are not automatic and were left untouched:

- `maxcode login` / `logout`: `login/` and `chatgpt/` OAuth/device-code flows,
  including `auth.openai.com`, `chatgpt.com`, and the localhost callback.
- `maxcode update`: `cli/` update command; it is explicit.
- `maxcode doctor`: version and desktop-app probes; it is explicit.
- `maxcode cloud ...`: cloud task API; explicit command.
- `maxcode plugin` / MCP / connectors: user-configured remote servers.
- `maxcode mcp` OAuth: user-configured MCP issuer endpoints.
- Realtime/voice/WebRTC: explicit feature use.
- Inline visualization: only when rendering content that references remote
  assets.

## Recommendations, not applied

- Remove the hard-coded Statsig client key and OpenAI-owned Sentry DSN from a
  fork, even though the defaults are now off. Legal review is requested in
  `docs/LICENSE-REVIEW.md`.
- Consider replacing `www.maxcode.com` placeholders with the project's real URL.
- Decide whether the TUI help links should point at upstream OpenAI docs or at
  maxcode-engine documentation.
- Re-audit after every upstream merge; new telemetry or update paths can appear.

## Verification

```bash
# Automatic defaults
grep -n "unwrap_or(false)" maxcode-rs/core/src/config/mod.rs | grep check_for_update
grep -n "default_analytics_enabled" maxcode-rs/tui/src/startup_orchestration.rs
grep -n "metrics_exporter: OtelExporterKind::None" maxcode-rs/config/src/types.rs
grep -n "analytics_enabled == Some(true)" maxcode-rs/analytics/src/client.rs
```
