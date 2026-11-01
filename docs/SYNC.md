# Upstream sync guide

Pinned upstream: OpenAI Codex `rust-v0.165.0`, commit
`360319d8dee62d87e2d59fc0c21805d79519cfed`. See [`UPSTREAM.md`](../UPSTREAM.md).

## Hard rule

**After 2026-10-31, the fork's functional delta must stay under 500 changed
lines outside pure renames and the mechanical rebrand.**

The initial rebrand is the baseline, not a precedent. It changed 9,692 paths:
8,759 renames, 546 true removals, 186 low-similarity moves, 13 true additions,
and 2 modifications. A future sync must not grow that delta by more than 500
functional lines. The counts are derived in [`CHANGES.md`](../CHANGES.md) and
[`docs/upstream-verify.log`](upstream-verify.log).

“Pure renames and the mechanical rebrand” means:

- `codex-rs/` → `maxcode-rs/`
- crate prefix `codex-` → `maxcode-`
- binary `codex` → `maxcode-engine`
- environment variable `CODEX_HOME` → `MAXCODE_HOME`
- case variants of the product name in strings, snapshots, and docs

Anything else — a new feature, a behavioral fix, a config key, a dependency
change with code impact — counts toward the 500-line budget. A sync that needs
more than 500 functional lines needs an explicit design review and an update to
this rule, not a silent merge.

## 1. Diff a new upstream tag

Do this in a temporary clone, never in the fork's working tree:

```bash
OLD=360319d8dee62d87e2d59fc0c21805d79519cfed
NEW_TAG=rust-v0.166.0   # example

git clone --depth 1 --branch "$NEW_TAG" https://github.com/openai/codex.git /tmp/codex-new
NEW=$(git -C /tmp/codex-new rev-parse HEAD)

# Full upstream change set between the pinned base and the new tag:
git -C /tmp/codex-new fetch --depth 1 origin "$OLD"
git -C /tmp/codex-new diff --name-status -M -l0 "$OLD" "$NEW" \
  > /tmp/upstream-$NEW_TAG.tsv
```

Read the diff before applying anything. Separate:

- changes under `codex-rs/` (candidate engine patches),
- changes under `sdk/`, `.github/`, `third_party/`, `patches/`, `scripts/`,
  `tools/`, `bazel/`, `docs/`, `codex-cli/` (fork removed these; usually drop),
- root metadata and lockfile changes (decide case by case).

## 2. Port fixes into the fork

Work on a branch:

```bash
git switch -c sync/$NEW_TAG
```

For each upstream commit or hunk you want:

1. Read the upstream commit message and tests.
2. Map the path: `codex-rs/` → `maxcode-rs/`, and apply the crate renames
   (`codex-api` → `maxcode-api`, `codex-home` → `maxcode-home`, and so on).
3. Apply the minimal patch. Do not copy upstream branding, telemetry defaults,
   or deleted trees back in.
4. Add or update the corresponding fork test. If the upstream change has no
   test, state why in the commit message.
5. Run the affected crate tests, then:
   ```bash
   cd maxcode-rs
   cargo check -p maxcode-cli
   cargo test -p maxcode-config --lib
   cargo test -p maxcode-analytics --lib
   ```
   The checked-in rustfmt configuration uses unstable options, so run
   `cargo fmt --all -- --check` only with a nightly toolchain that supports
   them.
6. Update `UPSTREAM.md`, `CHANGES.md`, and regenerate
   `docs/upstream-diff.tsv` against the new tag.

Do not merge the upstream branch wholesale. A merge would reintroduce:

- the `codex-rs/` tree and `codex` binary,
- upstream CI, SDK, Bazel, and third-party voice/V8 trees,
- upstream telemetry, analytics, and update defaults,
- upstream branding strings and OpenAI-only docs.

## 3. Measure the delta

The rebrand makes a raw `git diff --stat` misleading. Measure the functional
patch, not the rename noise:

```bash
# Paths changed since the last sync base, excluding pure path moves.
git diff --numstat -M -l0 <last-sync-base> HEAD -- 'maxcode-rs/**' > /tmp/delta.tsv

# Review only non-rename changes; add/subtract lines per file.
git diff --shortstat -M -l0 <last-sync-base> HEAD -- 'maxcode-rs/**'
```

The reviewer must subtract the mechanical rebrand. A practical rule: if a
changed file's diff is only `codex`→`maxcode`, path, or snapshot-name churn, it
does not count. Every other line counts toward the 500-line budget.

Record the result in the sync PR description:

```text
Functional delta: N changed lines outside pure renames (budget: 500)
Upstream tag: rust-vX.Y.Z (commit ...)
Tests: cargo check -p maxcode-cli; targeted crate tests; ...
```

## 4. Path and name mapping

| Upstream | Fork |
|---|---|
| `codex-rs/` | `maxcode-rs/` |
| `codex-rs/codex-api/` | `maxcode-rs/maxcode-api/` |
| `codex-rs/codex-home/` | `maxcode-rs/maxcode-home/` |
| crate prefix `codex-` | `maxcode-` |
| binary `codex` | `maxcode-engine` |
| `CODEX_HOME` | `MAXCODE_HOME` |
| `codex-cli` package/bin | `maxcode-cli` package, `maxcode-engine` bin |

Some upstream paths changed name as well as prefix. Check
`docs/upstream-diff.tsv` for the exact mapping before applying a patch.

## 5. Cadence and ownership

- Sync at least monthly, or when a security fix lands upstream.
- Pin the exact tag and commit in `UPSTREAM.md`; do not track a moving branch.
- Keep `LICENSE` and `NOTICE` intact. Re-read
  [`docs/LICENSE-REVIEW.md`](LICENSE-REVIEW.md) before changing either.
- Re-run the network audit after every sync. New telemetry or update paths can
  appear in upstream code even when the diff looks small.
- If a sync would exceed the 500-line budget, stop and open a design issue. Do
  not let scope grow silently.
