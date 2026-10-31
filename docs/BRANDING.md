# Branding audit

Date: 2026-10-31
Scope: user-visible strings in `maxcode-rs/**`, plus the root `README.md` and
`NOTICE`.

## Search result

- `Codex` in `maxcode-rs/`: **0 occurrences**.
- `OpenAI` in `maxcode-rs/`: many occurrences remain, but they are
  overwhelmingly provider names, auth/host names, wire headers, bundle IDs,
  schema field names, test fixtures, or third-party documentation links. The
  fork does not present itself as OpenAI.
- `maxcode-engine --help` prints `Maxcode CLI`, `Run Maxcode non-interactively`,
  and no `Codex` or `OpenAI`.
- `maxcode-engine --version` prints `maxcode-engine 0.1.0`.

## Changed user-visible strings

| File:line | Before | After |
|---|---|---|
| `maxcode-rs/login/src/device_code_auth.rs:152` | `OpenAI's command-line coding agent` | `maxcode-engine's command-line coding agent` |
| `maxcode-rs/prompts/templates/realtime/backend_prompt.md:3` | `You are Maxcode, an OpenAI general-purpose agentic assistant` | `You are maxcode-engine, an open-source general-purpose agentic assistant` |
| `maxcode-rs/exec/src/event_processor_with_human_output.rs:221` | `OpenAI Maxcode v{...}` | `maxcode-engine v{...}` |
| `maxcode-rs/ext/git-attribution/src/world_state.rs` | `noreply@openai.com`; `https://openai.com/maxcode/` | `noreply@maxcode-engine.invalid`; `https://github.com/RavaniRoshan/maxcode-engine` |
| `maxcode-rs/tui/src/history_cell/notices.rs:42,59,76,84` | `github.com/openai/maxcode` links | `github.com/RavaniRoshan/maxcode-engine` links |
| `maxcode-rs/tui/src/update_prompt.rs:42` | OpenAI repository release notes URL | Fork release notes URL |
| `maxcode-rs/tui/src/bottom_pane/feedback_view.rs:18` | OpenAI repository issue URL | Fork issue URL |
| `maxcode-rs/tui/src/model_migration.rs:470,493,522` | `https://www.maxcode.com/models/...` | `https://github.com/RavaniRoshan/maxcode-engine` |
| `maxcode-rs/tui/src/updates.rs:62` | `api.github.com/repos/openai/maxcode` | `api.github.com/repos/RavaniRoshan/maxcode-engine` |
| `maxcode-rs/cli/src/doctor/updates.rs:36` | `api.github.com/repos/openai/maxcode` | `api.github.com/repos/RavaniRoshan/maxcode-engine` |
| `maxcode-rs/cli/src/doctor/updates.rs:392-399` | `npm install -g @openai/maxcode` etc. | Fork releases URL |
| `maxcode-rs/cli/src/main.rs:879,899,2884` | `developers.openai.com/maxcode/cli/` | Fork releases URL |
| `maxcode-rs/tui/src/tooltips.rs` | Announcement feed fetched from `raw.githubusercontent.com/openai/maxcode` | Fetch disabled (`ANNOUNCEMENT_TIPS_ENABLED = false`) |
| Test snapshots | `OpenAI Maxcode`, `OpenAI's command-line...`, `Use your own OpenAI API key` | Source-aligned `maxcode`, `maxcode-engine`, `Use your own API key` |

The mechanical pass also replaced `github.com/openai/maxcode` and
`https://openai.com/maxcode/` in 47 source/doc/test files. It did **not** touch
`github.com/openai/codex` in the root attribution.

## Retained on purpose (third-party references)

These are not the fork's self-identification. Replacing them would either break
a provider integration or misstate a third-party service.

| Category | Examples | Why retained |
|---|---|---|
| Provider name and endpoint | `OPENAI_PROVIDER_NAME = "OpenAI"` at `model-provider-info/src/lib.rs:79`; `api.openai.com/v1` defaults | The OpenAI provider is a supported third-party endpoint; users select it by name |
| Auth/login | `auth.openai.com`, `chatgpt.com/backend-api`, device-code and OAuth labels | Required to sign in to an OpenAI/ChatGPT account; opt-in |
| Wire protocol | `OpenAI-Beta`, `x-openai-*`, `openai/elicitation.*`, `openai/userVerification` | Upstream-compatible protocol identifiers; changing them breaks interop |
| Bundle/package IDs | `com.openai.maxcode`, `OpenAI.Maxcode`, `@openai/maxcode` | OS bundle identity and npm package identifiers; changing them without a new registration breaks installs/updates |
| Third-party docs/help | `help.openai.com`, `developers.openai.com`, `platform.openai.com`, `chatgpt.com` help links | Documentation and login links that explain the third-party service |
| Attribution | Root `README.md` “fork of the Codex CLI … not affiliated with OpenAI”; `NOTICE` upstream block | Required attribution/non-affiliation statement |

## Remaining product decisions (not changed)

1. `tui/src/history_cell/notices.rs:152` links to an OpenAI help article for
   safety-access policy. There is no fork equivalent. Consider removing the
   link or replacing it with a maxcode-engine policy page.
2. `tui/src/history_cell/notices.rs:179` links to OpenAI's enterprise
   trusted-access form. The fork cannot grant that access; consider removing it.
3. `tui/src/chatwidget/plugin_catalog.rs:67` links to an OpenAI help article
   about ChatGPT apps.
4. `tui/src/tooltips.rs:22,28` still advertise the ChatGPT desktop app and
   `learn.chatgpt.com`. These are third-party product links, not the fork.
5. `tui/src/npm_registry.rs:5` and `cli/src/doctor/updates.rs` still know the
   `@openai/maxcode` npm package; no fork package is published. The startup
   update check is off by default, but `maxcode update`/`doctor` remain explicit.
6. `maxcode-rs/docs/bazel.md` contains OpenAI-employee-specific build
   instructions. It is not part of the runtime, but it is user-visible
   documentation.
7. `ANNOUNCEMENT_TIP_URL` remains in source even though the fetch is disabled.
   A follow-up should remove the URL and the unused fetch path.

## Verification

```bash
./maxcode-rs/target/dev-small/maxcode-engine --help | grep -E "Codex|OpenAI"
./maxcode-rs/target/dev-small/maxcode-engine --version
git grep -n "Codex" -- maxcode-rs
git grep -n "OpenAI's command-line\|OpenAI Maxcode\|OpenAI general-purpose" -- maxcode-rs
```
