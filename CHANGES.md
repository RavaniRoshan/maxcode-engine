# Changes from upstream

Fork: `maxcode-engine`
Upstream: OpenAI Codex `rust-v0.165.0`, commit
`360319d8dee62d87e2d59fc0c21805d79519cfed`
Fork HEAD at audit: `0a109582cc02eeba10134e005b0cfc1d6ffae7f9`
Date: 2026-10-31

This report covers the fork tree at the audit commit. Files added by the audit
itself (`docs/`, `.github/workflows/release.yml`, `AUDIT.md`, `UPSTREAM.md`,
`CHANGES.md`) are not upstream-derived and are not part of the counts below.

## Method

The audit cloned the upstream tag into a temporary directory, fetched the fork,
and ran:

```bash
git diff --name-status -M -l0 360319d8dee62d87e2d59fc0c21805d79519cfed <fork-head>
```

`-M` enables rename detection, and `-l0` removes rename-detection limits.
The raw output is committed at
[`docs/upstream-diff.tsv`](docs/upstream-diff.tsv): every rename, addition,
removal, and modification is one tab-separated line. This file summarizes the
counts and the non-rename changes.

## Counts

| Category | Entries |
|---|---:|
| Renames detected by Git | 8,759 |
| — of which exact renames (`R100`) | 3,840 |
| — of which renames with content edits (`R050`–`R098`) | 4,919 |
| — of which changed the path below `codex-rs/` | 1,645 |
| Additions reported by Git | 199 |
| — low-similarity moves paired by path (`maxcode-rs/X` ← `codex-rs/X`) | 179 |
| — genuinely new files | 20 |
| Removals reported by Git | 732 |
| — low-similarity move sources paired by path | 179 |
| — true removals | 553 |
| Modifications | 2 |
| **Total name-status entries** | **9,692** |

File counts: upstream 9,507 tracked files; fork 8,974; net **-533**.

Line delta from `git diff --shortstat -M`: **92,828 insertions, 206,210
deletions**. This includes the mechanical rebrand inside moved files. It is not
the size of a functional patch.

## Renames

All 8,759 detected renames move `codex-rs/**` to `maxcode-rs/**`. The rename
preserves the crate layout for 7,114 files; 1,645 files also changed their path
below the root because crate names changed (`codex-api` → `maxcode-api`,
`codex-home` → `maxcode-home`, and similar).

The exhaustive rename list is `docs/upstream-diff.tsv`, lines whose first field
starts with `R`. Example:

```text
R100	codex-rs/core/src/client.rs	maxcode-rs/core/src/client.rs
R095	codex-rs/cli/src/main.rs	maxcode-rs/cli/src/main.rs
```

## Genuinely new files (20)

These have no path-paired deletion; most are renamed snapshots or new crate
manifests whose names changed more than the rename threshold allows.

```text
maxcode-rs/app-server-protocol/schema/typescript/v2/MaxcodeErrorInfo.ts
maxcode-rs/cli/e2e_benches/maxcode_help.rs
maxcode-rs/cli/src/doctor/snapshots/maxcode__doctor__background__tests__configured_updater.snap
maxcode-rs/cli/src/snapshots/maxcode__tests__unsupported_worktree_commands.snap
maxcode-rs/cli/src/snapshots/maxcode_cli__login__storage_tests__login_default_filter_records_sanitized_storage_fallback.snap
maxcode-rs/core/src/context/world_state/snapshots/maxcode_core__context__world_state__tools__tests__snapshots_namespace_omission.snap
maxcode-rs/maxcode-api/BUILD.bazel
maxcode-rs/maxcode-backend-openapi-models/BUILD.bazel
maxcode-rs/maxcode-client/BUILD.bazel
maxcode-rs/maxcode-experimental-api-macros/BUILD.bazel
maxcode-rs/maxcode-home/BUILD.bazel
maxcode-rs/maxcode-home/src/lib.rs
maxcode-rs/maxcode-mcp/BUILD.bazel
maxcode-rs/tui/src/app/tests/snapshots/maxcode_tui__app__tests__background_exit_tests__remote_disconnect_exit.snap
maxcode-rs/tui/src/bottom_pane/snapshots/maxcode_tui__bottom_pane__feedback_view__tests__feedback_success_employee.snap
maxcode-rs/tui/src/chatwidget/realtime/snapshots/maxcode_tui__chatwidget__realtime__tests__recording_controls_tests__voice_meters_do_not_sample_again_on_early_redraws.snap
maxcode-rs/tui/src/snapshots/maxcode_tui__app__agents_overview__tests__agents_daemon_started.snap
maxcode-rs/tui/src/snapshots/maxcode_tui__keymap_setup__tests__keymap_debug_view_initial.snap
maxcode-rs/tui/src/snapshots/maxcode_tui__markdown_render__markdown_render_tests__file_link_keeps_descriptive_label_and_target.snap
maxcode-rs/tui/src/snapshots/maxcode_tui__tooltips__tests__desktop_app_tip_windows.snap
```

The remaining 179 additions are low-similarity moves. They are paired by path
in `docs/upstream-diff.tsv` and counted separately above.

## True removals (553)

Every removed path is in `docs/upstream-diff.tsv` (lines starting with `D` minus
the 179 path-paired moves). The removals by top-level directory are:

| Directory | Files removed |
|---|---:|
| `sdk/` | 125 |
| `.github/` | 124 |
| `third_party/` | 71 |
| `scripts/` | 58 |
| `codex-rs/` (files with no `maxcode-rs` counterpart) | 38 |
| `patches/` | 36 |
| `tools/` | 31 |
| `bazel/` | 28 |
| `docs/` | 15 |
| `codex-cli/` | 7 |
| `.vscode/` | 3 |
| root files | 17 |

The root removals are `BUILD.bazel`, `CHANGELOG.md`, `MODULE.bazel`,
`MODULE.bazel.lock`, `SECURITY.md`, `announcement_tip.toml`, `defs.bzl`,
`flake.lock`, `flake.nix`, `justfile`, `package.json`, `pnpm-lock.yaml`,
`pnpm-workspace.yaml`, `rbe.bzl`, `ruff.toml`,
`workspace_root_test_launcher.bat.tpl`, and
`workspace_root_test_launcher.sh.tpl`.

The removals are mostly upstream-only release, SDK, CI, Bazel, third-party
voice/V8, and packaging machinery. `LICENSE` and `NOTICE` were not removed.

## Modifications (2)

| File | Change |
|---|---|
| `README.md` | Rewritten for maxcode-engine; keeps the upstream attribution. |
| `NOTICE` | Fork copyright and modification statement added; upstream notices retained. |

No Rust source file's license header was changed. Upstream `codex-rs` had
zero `Copyright ... OpenAI` headers and 14 `SPDX-License-Identifier` headers;
the fork's `maxcode-rs` has the same 14. See
[`docs/LICENSE-REVIEW.md`](docs/LICENSE-REVIEW.md).

## Caveat

Git rename detection is similarity-based. A file whose rename also changes most
of its content can be reported as `A` + `D` rather than `R`. This report
separates those 179 path-paired moves so the true addition/removal counts are
not inflated. The raw `docs/upstream-diff.tsv` preserves Git's mechanical
output; the summary applies the path-pair correction.
