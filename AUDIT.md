# Outbound network audit

Date: 2026-11-01
Scope: `maxcode-rs/**` Rust source, excluding tests and fixtures unless noted.
Evidence: [`docs/network-audit.log`](docs/network-audit.log) contains every
`file:line` listed below, with the two embedded credentials redacted. Numbers and
lines in this file are copied from that log.

This is an audit of distinct destination families and the switch that enables
them, not a count of every HTTP call site. Provider endpoints are user-configured
and are called once per model request.

## Defaults in this fork

These five switches are off by default in the committed tree. Core `exec` with a
user-configured provider does not depend on any of them.

| Behaviour | Value in the tree | File:line |
|---|---|---|
| Startup update check | `unwrap_or(false)` | `maxcode-rs/core/src/config/mod.rs:4270` |
| TUI analytics consent | `/*default_analytics_enabled*/ false` | `maxcode-rs/tui/src/startup_orchestration.rs:673` |
| Analytics event delivery | queue built only when `analytics_enabled == Some(true)` | `maxcode-rs/analytics/src/client.rs:321` |
| OTel exporters | `exporter`, `trace_exporter`, `metrics_exporter` all `OtelExporterKind::None` | `maxcode-rs/config/src/types.rs:680-682` |
| Announcement-tip feed | `ANNOUNCEMENT_TIPS_ENABLED: bool = false`, prewarm returns before fetching | `maxcode-rs/tui/src/tooltips.rs:20,185` |

Users can opt back in with `check_for_update_on_startup = true`,
`[analytics] enabled = true`, `[otel] metrics_exporter = "statsig"` (or a custom
OTLP endpoint), and by removing the announcement-feed guard.

## Automatic or default-on destinations

| Destination | File:line | Purpose | Default |
|---|---|---|---|
| `https://ab.chatgpt.com/otlp/v1/metrics` with a hard-coded `statsig-api-key` header | `maxcode-rs/otel/src/config.rs:9,11,25,28` | Statsig OTLP metrics export | Off: `metrics_exporter` defaults to `none`; only an explicit `statsig` choice builds this exporter |
| `{chatgpt_base_url}/maxcode/analytics-events/events` | `maxcode-rs/analytics/src/client.rs:154` | Analytics event POST | Off: queue exists only when `[analytics] enabled = true` (line 321); debug builds capture to a file instead (line 149) |
| `https://api.github.com/repos/RavaniRoshan/maxcode-engine/releases/latest` | `maxcode-rs/cli/src/doctor/updates.rs:36`, `maxcode-rs/tui/src/updates.rs:62` | Version freshness check | Startup check off (see above); reached only by explicit `maxcode update` / `maxcode doctor` |
| `https://formulae.brew.sh/api/cask/maxcode.json` | `maxcode-rs/cli/src/doctor/updates.rs:37`, `maxcode-rs/tui/src/updates.rs:61` | Homebrew update check | Explicit command only; the cask name is the rebranded, non-existent one (see below) |
| `https://persistent.oaistatic.com/maxcode-app-prod/...` and `https://chatgpt.com/backend-api/wham/app/appcast` | `maxcode-rs/cli/src/doctor/updates.rs:39,41,43,46` | Desktop-app update appcasts | Explicit `maxcode doctor` on macOS/Windows only; rebranded path (see below) |
| `https://registry.npmjs.org/@openai%2fmaxcode` | `maxcode-rs/tui/src/npm_registry.rs:5,79` | npm package metadata for update checks | Only on the update path; rebranded package name (see below) |
| `https://raw.githubusercontent.com/openai/maxcode/main/announcement_tip.toml` | `maxcode-rs/tui/src/tooltips.rs:17` | Announcement tooltip feed | Off: guarded by `ANNOUNCEMENT_TIPS_ENABLED` (line 20, checked at line 185) |
| Sentry envelope endpoint (`o33249.ingest.us.sentry.io`) with an embedded DSN credential | `maxcode-rs/feedback/src/lib.rs:62`, `maxcode-rs/feedback/src/report_upload.rs:57` | Feedback envelope upload | Only after an explicit feedback submission; there is no automatic upload |
| `https://chatgpt.com/backend-api/maxcode` | `maxcode-rs/model-provider-info/src/lib.rs:82` | ChatGPT-auth backend for sessions, usage, profiles | The default provider is `openai` (`maxcode-rs/core/src/config/mod.rs:4007`); with ChatGPT auth this base URL is used, otherwise `https://api.openai.com/v1` (`model-provider-info/src/lib.rs:443-445`) |
| `https://chatgpt.com/backend-api` normalization | `maxcode-rs/backend-client/src/client.rs:231` | Add `/backend-api` to ChatGPT host bases | Only for ChatGPT-host base URLs |
| `https://auth.openai.com/api/accounts`, `https://chatgpt.com/maxcode-backend/agent-identity` | `maxcode-rs/agent-identity/src/lib.rs:41-44` | Agent identity / login token exchange | Only login-bound flows |
| `https://api.openai.com/v1` and `wss://api.openai.com/v1/realtime` | `maxcode-rs/maxcode-api/src/endpoint/realtime_websocket/methods.rs:63` | Realtime WebSocket base | Only realtime sessions |
| `wss://chatgpt.com/backend-api/wham/remote/control/server` | `maxcode-rs/app-server-transport/src/transport/remote_control/protocol.rs:303` | Remote-control pairing | Only `maxcode remote-control` |
| `https://api.githubcopilot.com/mcp/` | `maxcode-rs/maxcode-mcp/src/connection_manager/startup.rs:104` | GitHub Copilot MCP endpoint | Only a configured MCP server |
| `https://gov.chatgpt.com` | `maxcode-rs/maxcode-mcp/src/mcp/mod.rs:383` | Government-cloud MCP path | Only government-cloud configuration |
| `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `esm.sh`, `fonts.bunny.net`, `fonts.googleapis.com`, `fonts.gstatic.com`, `unpkg.com` | `maxcode-rs/tui/src/inline_visualization/viewer.rs:16` | CSP allow-list for assets referenced by inline HTML visualization | Only when rendering content that references them |
| OAuth issuer bindings for configured MCP servers (e.g. `mcp.mercadopago.com`, `api.robinhood.com`) | `maxcode-rs/rmcp-client/src/oauth/issuer_binding.rs:53` | MCP OAuth issuer matching | Only configured MCP servers |

## Finding: the rebrand rewrote upstream service paths

The mechanical `codex` → `maxcode` replacement also rewrote strings that named
real OpenAI services and real package artifacts. Those paths are not valid for
the upstream service:

| Rebranded value | File:line | Effect |
|---|---|---|
| `https://chatgpt.com/backend-api/maxcode` | `model-provider-info/src/lib.rs:82` | ChatGPT-auth sessions target a path that does not exist upstream |
| `{base_url}/maxcode/analytics-events/events` | `analytics/src/client.rs:154` | Analytics POST would 404; off by default |
| `https://formulae.brew.sh/api/cask/maxcode.json` | `cli/src/doctor/updates.rs:37`, `tui/src/updates.rs:61` | No such Homebrew cask |
| `https://registry.npmjs.org/@openai%2fmaxcode` | `tui/src/npm_registry.rs:5,79` | No such npm package |
| `https://persistent.oaistatic.com/maxcode-app-prod/...` | `cli/src/doctor/updates.rs:39,41,46` | No such appcast path |
| `https://raw.githubusercontent.com/openai/maxcode/...` | `tui/src/tooltips.rs:17` | No such repository; fetch already disabled |
| `https://api.github.com/repos/RavaniRoshan/maxcode-engine/releases/latest` | `cli/src/doctor/updates.rs:36`, `tui/src/updates.rs:62` | Valid: repointed at the fork |

Consequence: the explicit `update` and `doctor` commands will fail with 404/404-like
responses rather than returning upstream data. Nothing here silently succeeds or
sends data somewhere unexpected; the destinations are dead names, not
attacker-controlled. Repointing or deleting these probes needs a code change and
its own test run, so it is left as a follow-up.

## Build-time network

`Cargo.lock` pins 11 `git+` sources. Three come from `[patch.crates-io]` at
`maxcode-rs/Cargo.toml:663-668` and are OpenAI-owned forks:
`openai-oss-forks/crossterm`, `openai-oss-forks/tokio-tungstenite`, and
`openai-oss-forks/tungstenite-rs`. These are fetched by Cargo at build time, not
at runtime, and are pinned by revision. The release workflow depends on GitHub
being reachable for them.

## Recommendations, not applied

1. Delete the hard-coded Statsig endpoint and client key from `otel/src/config.rs`.
   The exporter is off by default, but the credential has no reason to ship in a
   fork. This changes the `statsig` exporter arm and needs its crate tests.
2. Delete the embedded Sentry DSN from `feedback/src/lib.rs`, or make feedback
   require a user-supplied endpoint. Same reasoning.
3. Repoint or delete the rebranded probes listed above (`maxcode.json` cask,
   `@openai/maxcode` npm, `maxcode-app-prod` appcast, `openai/maxcode`
   announcement URL).
4. Re-audit after every upstream merge; new telemetry or update paths can appear
   in upstream code even when the diff looks small.

No crate was removed and no runtime default was changed by this audit pass.

## Verification

```bash
grep -n "check_for_update_on_startup.unwrap_or" maxcode-rs/core/src/config/mod.rs
grep -n "default_analytics_enabled" maxcode-rs/tui/src/startup_orchestration.rs
grep -n "metrics_exporter: OtelExporterKind::None" maxcode-rs/config/src/types.rs
grep -n "analytics_enabled == Some(true)" maxcode-rs/analytics/src/client.rs
grep -n "ANNOUNCEMENT_TIPS_ENABLED" maxcode-rs/tui/src/tooltips.rs
```

Expected output is in [`docs/network-audit.log`](docs/network-audit.log).
