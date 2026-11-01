# Branding audit

Date: 2026-11-01
Scope: user-visible strings in `maxcode-rs/**`, plus the root `README.md` and
`NOTICE`.
Evidence: [`docs/branding-check.log`](branding-check.log) holds the binary output
and `git grep` results quoted below.

## Result

- `Codex` in `maxcode-rs/`: **0 occurrences**.
- `OpenAI` in `maxcode-rs/`: **450 occurrences across 201 files**. They are
  provider names, auth and host names, wire headers, Windows package identities,
  schema field names, test fixtures, or third-party documentation links. The fork
  does not present itself as OpenAI.
- `maxcode-engine --version` prints `maxcode-engine 0.1.0`.
- `maxcode-engine --help` prints `Maxcode CLI` and `Run Maxcode non-interactively`,
  with no `Codex` or `OpenAI`.
- `exec --help` contains one `OpenAI` reference, `(OpenAI provider only)`, which
  names the supported third-party provider. `login`, `update`, and `doctor --help`
  contain none.

## Changed user-visible strings

| File:line | Now reads |
|---|---|
| `maxcode-rs/login/src/device_code_auth.rs:152` | `maxcode-engine's command-line coding agent` |
| `maxcode-rs/prompts/templates/realtime/backend_prompt.md:3` | `You are maxcode-engine, an open-source general-purpose agentic assistant` |
| `maxcode-rs/exec/src/event_processor_with_human_output.rs:221` | `maxcode-engine v{VERSION}` |
| `maxcode-rs/ext/git-attribution/src/world_state.rs:12,20,22` | `Co-authored-by: Maxcode <noreply@maxcode-engine.invalid>` and `https://github.com/RavaniRoshan/maxcode-engine` |
| `maxcode-rs/tui/src/history_cell/notices.rs:42,59,76` | `https://github.com/RavaniRoshan/maxcode-engine` links |
| `maxcode-rs/tui/src/update_prompt.rs:42` | Fork release-notes URL |
| `maxcode-rs/tui/src/bottom_pane/feedback_view.rs:18` | Fork issue URL |
| `maxcode-rs/tui/src/model_migration.rs:470,493,522` | `https://github.com/RavaniRoshan/maxcode-engine` |
| `maxcode-rs/tui/src/updates.rs:62` | `https://api.github.com/repos/RavaniRoshan/maxcode-engine/releases/latest` |
| `maxcode-rs/cli/src/doctor/updates.rs:36` | Same fork release URL |
| `maxcode-rs/cli/src/doctor/updates.rs:399` | Fork releases URL replaces npm install instructions |
| `maxcode-rs/cli/src/main.rs:879,899,2884` | Fork releases URL replaces `developers.openai.com` links |
| Test snapshots | Source-aligned `maxcode`, `maxcode-engine`, and `Use your own API key` |

The mechanical passes replaced `github.com/openai/codex`,
`https://openai.com/codex/`, and `developers.openai.com/codex` across source,
docs, and snapshots. They did **not** touch `github.com/openai/codex` in the root
attribution.

## Retained on purpose (third-party references)

These are not the fork's self-identification. Replacing them would either break a
provider integration or misstate a third-party service.

| Category | Examples | Why retained |
|---|---|---|
| Provider name and endpoint | `OPENAI_PROVIDER_NAME = "OpenAI"` at `model-provider-info/src/lib.rs:79`; `api.openai.com/v1` defaults | The OpenAI provider is a supported third-party endpoint; users select it by name |
| Auth/login | `auth.openai.com`, `chatgpt.com/backend-api`, device-code and OAuth labels | Required to sign in to an OpenAI/ChatGPT account; opt-in |
| Wire protocol | `OpenAI-Beta`, `x-openai-*`, `X-OpenAI-Fedramp`, `openai/elicitation.*` | Upstream-compatible protocol identifiers; changing them breaks interop |
| Windows identity | `OpenAI.Maxcode_2p2nqsd0c76g0`, `%ProgramData%\OpenAI\Maxcode` | Installed-app identity and enforced-config path; changing them breaks uninstall/update and reads the wrong policy location |
| Third-party docs/help | `help.openai.com`, `developers.openai.com`, `platform.openai.com`, `chatgpt.com` help links | Documentation and login links that explain the third-party service |
| Attribution | Root `README.md` “fork of the Codex CLI … not affiliated with OpenAI”; `NOTICE` upstream block | Required attribution/non-affiliation statement |

## Finding: leftover upstream product text in TUI tooltips

Two user-visible tooltip strings still name upstream product surfaces:

- `maxcode-rs/tui/src/tooltips.rs:25` — `Install it from https://chatgpt.com/maxcode?app-landing-page=true and run \`maxcode app\``
- `maxcode-rs/tui/src/tooltips.rs:28` — `Install it from https://learn.chatgpt.com/docs/linux/linux-app and run \`chatgpt\``

`chatgpt` is not a command this binary provides, and `chatgpt.com/maxcode` is a
rebranded path. Three snapshot tests pin the current strings
(`maxcode_tui__tooltips__tests__desktop_app_tip_{windows,linux,macos}.snap`), so
changing them means updating those snapshots. That is a follow-up, not part of
this pass: the task is branding cleanup, and the tooltip text is functional advice
tied to the desktop-app onboarding flow.

## Remaining product decisions

1. `tui/src/history_cell/notices.rs:152` links to an OpenAI help article for the
   safety-access policy. There is no fork equivalent. Consider removing the link
   or replacing it with a maxcode-engine policy page.
2. `tui/src/history_cell/notices.rs:179` links to OpenAI's enterprise
   trusted-access form. The fork cannot grant that access; consider removing it.
3. `tui/src/chatwidget/plugin_catalog.rs:67` links to an OpenAI help article
   about ChatGPT apps.
4. `tui/src/npm_registry.rs:5,79` and `cli/src/doctor/updates.rs` still know the
   `@openai/maxcode` npm package and a `maxcode.json` Homebrew cask. Neither
   exists; see the rebranded-path finding in [`AUDIT.md`](../AUDIT.md).
5. `maxcode-rs/docs/bazel.md` contains OpenAI-employee-specific build
   instructions. It is not part of the runtime, but it is user-visible
   documentation.
6. `ANNOUNCEMENT_TIP_URL` remains in source even though the fetch is disabled.
   A follow-up should remove the URL and the unused fetch path.

## Verification

```bash
./maxcode-rs/target/dev-small/maxcode-engine --help | grep -E "Codex|OpenAI"
./maxcode-rs/target/dev-small/maxcode-engine --version
git grep -n "Codex" -- maxcode-rs
git grep -c "OpenAI" -- maxcode-rs | awk -F: '{s+=$2} END {print s}'
```

The observed output is in [`docs/branding-check.log`](branding-check.log).
