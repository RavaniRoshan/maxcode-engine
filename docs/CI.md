# CI inventory

Date: 2026-11-01
Evidence: [`docs/ci-runs.log`](ci-runs.log) (raw `gh` output captured the same day).
Every duration and count below is copied from that log.

## Current workflows

`.github/workflows` contains exactly one file.

| File | Trigger | Jobs and timeout | Runs in this repo? |
|---|---|---|---|
| `release.yml` | `push` of a tag matching `v*` | `build` matrix (`linux-x86_64`, `macos-arm64`), `timeout-minutes: 120`; then `publish-release` on `ubuntu-latest` | Yes, on a tag push. No repository secrets: it uses the built-in `GITHUB_TOKEN` with `permissions: contents: write`. No CLA check. Both runners are GitHub-hosted (`ubuntu-latest`, `macos-14`). |

Measured history: `git tag | wc -l` is `0` and no run named `Release` appears in
`gh run list --limit 100`, so this workflow has never executed. Its build time is
therefore **not measured**; 120 minutes is the job timeout, not an observed
duration.

## Dependabot

- `.github/dependabot.yml` and `.github/dependabot.yaml` do not exist.
- No open pull requests.
- Ten closed pull requests (#1–#10), every one authored by `app/dependabot`, all
  targeting paths the fork removed (`codex-rs/`, `.github/workflows/`). See
  [`PR-REVIEW.md`](PR-REVIEW.md) for the per-PR classification.

Recommendation: do not re-add a Dependabot config in this fork. It would recreate
the same obsolete PRs against `codex-rs/` unless it is first rewritten for
`maxcode-rs/` and the one remaining workflow. That is a separate change, not part
of this task.

## Upstream workflows, removed in the rebrand

The upstream tag ships 28 workflow files in `.github/workflows`, plus
`Dockerfile.bazel`, `README.md`, and a `zstd/` directory (31 entries total; listed
in [`docs/upstream-verify.log`](upstream-verify.log)). The fork removed all of
it during the initial rebrand. The YAML is not in this repository, but GitHub
still lists the last runs, which gives real durations for what used to happen on
every Dependabot PR:

| Workflow name (history only) | Last seen trigger | Runs in log | Result | Duration range |
|---|---|---|---:|---|
| `blocking-ci` | `pull_request` | 10 | 10 failed | 1h32m21s – 4h5m48s |
| `v8-canary` | `pull_request` | 10 | 5 failed | 16s – 6h19m18s |
| `CLA Assistant` | `pull_request_target` | 10 | 10 skipped | 1s – 6s |

The `v8-canary` 16s and 20s runs are GitHub-Actions-only Dependabot updates that
touch no Rust code; the Rust-touching runs took 2h38m41s to 6h19m18s.

Why none of them can work here:

- Their YAML is gone; there is nothing to run.
- `blocking-ci` and `v8-canary` reference the upstream `codex-rs/` layout, the
  removed `third_party/v8` and voice trees, and upstream-only CI tooling.
- `CLA Assistant` is a `pull_request_target` workflow that needs OpenAI's CLA
  configuration and org-level app; every historical run was skipped, including on
  bot PRs.

## Decision for this step

Disable nothing further: there is nothing left to disable. The only remaining
workflow is the fork's own release workflow; it is assessed in the next section.

## Release workflow

`.github/workflows/release.yml` is the fork's own workflow, not upstream's. It is
kept.

| Property | Value |
|---|---|
| Trigger | `push` of a tag matching `v*` |
| Jobs | `build` matrix (`linux-x86_64` on `ubuntu-latest`, `macos-arm64` on `macos-14`), then `publish-release` on `ubuntu-latest` |
| Job timeout | 120 minutes (`timeout-minutes: 120`) |
| Permissions | `contents: write`; uses the built-in `GITHUB_TOKEN`, no repository secrets |
| Assets | `maxcode-engine-linux-x86_64`, `maxcode-engine-macos-arm64`, and a combined `SHA256SUMS` |

Build command:

```bash
cargo build --release --locked -p maxcode-cli --bin maxcode-engine --target <target>
```

Prerequisites checked on 2026-11-01, all recorded in
[`docs/ci-runs.log`](ci-runs.log):

- `maxcode-cli 0.165.0` exposes a `maxcode-engine` binary target.
- `Cargo.lock` is tracked (`sha256 37f945cb…`) and `cargo metadata --locked`
  exits `0`, so `--locked` will resolve against the committed lock.
- On `x86_64-unknown-linux-gnu`, `libsqlite3-sys` is reached through
  `sqlx-sqlite` with the `sqlite-bundled` feature
  (`maxcode-rs/Cargo.toml:501`), and `protoc-bin-vendored` is reached through
  `maxcode-code-mode-protocol`. Neither system SQLite nor a system `protoc` is
  needed.
- `openssl` / `openssl-sys` are reached through `native-tls` on the same target.
  The GNU build therefore expects OpenSSL development headers on the runner.
  GitHub's `ubuntu-latest` image normally provides them, but this repository has
  no run proving it. This is the most likely way the first tagged build fails.
- The macOS job uses `macos-14`, which is arm64, matching `aarch64-apple-darwin`.

Not verified: the workflow has never run (`git tag` count is `0`, no `Release`
run in history), so neither success nor wall-clock time is measured. The 120
minute figure is the configured timeout, not an observation. If the first tag
exceeds it, the fix is to raise or split the timeout then, with the run as
evidence.

