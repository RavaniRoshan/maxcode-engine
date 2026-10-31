# Open pull request review

Date: 2026-10-31
Repository: `RavaniRoshan/maxcode-engine`

Ten PRs were open. All ten are Dependabot PRs. All ten target paths that the
fork removed: `codex-rs/` (renamed to `maxcode-rs/`) or `.github/workflows/`
(removed with the upstream CI tree). They cannot apply to the current tree.

| PR | Title | Author | Files | Classification | Action |
|---:|---|---|---|---|---|
| #1 | Bump codespell-project/codespell-problem-matcher from 1.1.0 to 1.2.0 | dependabot | `.github/workflows/codespell.yml` | bot, obsolete path | closed |
| #2 | Bump pnpm/action-setup … | dependabot | `.github/workflows/*.yml` | bot, obsolete path | closed |
| #3 | Bump actions/upload-artifact from 7.0.0 to 7.0.2 | dependabot | `.github/workflows/*.yml` | bot, obsolete path | closed |
| #4 | Bump actions/cache/save from 5.0.4 to 6.1.0 | dependabot | `.github/workflows/*.yml` | bot, obsolete path | closed |
| #5 | Bump taiki-e/install-action from 2.62.49 to 2.87.31 | dependabot | `.github/workflows/*.yml` | bot, obsolete path | closed |
| #6 | Bump protoc-bin-vendored from 3.2.0 to 3.3.0 in `/codex-rs` | dependabot | `codex-rs/Cargo.lock` | bot, obsolete path | closed |
| #7 | Bump dirs from 6.0.0 to 7.0.0 in `/codex-rs` | dependabot | `codex-rs/Cargo.{lock,toml}` | bot, obsolete path | closed |
| #8 | Bump opentelemetry-appender-tracing from 0.31.1 to 0.33.1 in `/codex-rs` | dependabot | `codex-rs/Cargo.{lock,toml}` | bot, obsolete path | closed |
| #9 | Bump webbrowser from 1.2.4 to 1.2.5 in `/codex-rs` | dependabot | `codex-rs/Cargo.lock` | bot, obsolete path | closed |
| #10 | Bump quick-xml from 0.41.0 to 0.42.0 in `/codex-rs` | dependabot | `codex-rs/Cargo.{lock,toml}` | bot, obsolete path | closed |

No non-bot or code-changing PR was closed. If Dependabot is re-enabled in the
fork, configure it to target `maxcode-rs/` and the new workflow tree, otherwise
it will recreate these obsolete PRs.

Closing is reversible from the GitHub UI. The branches were not deleted.
