# Upstream pin

This fork is based on:

- Repository: <https://github.com/openai/codex>
- Tag: `rust-v0.165.0`
- Commit: `360319d8dee62d87e2d59fc0c21805d79519cfed`
- Short commit: `360319d`
- Upstream tree: `307e4fa207fe563a11f3f7af09cee6fccd2d8376`

The fork records the same base commit in its first commit message
(`Fork Codex v0.165.0 (commit 360319d)`). The fork HEAD used for the
2026-10-31 audit is `0a109582cc02eeba10134e005b0cfc1d6ffae7f9`.

Verify the pin:

```bash
git clone --depth 1 --branch rust-v0.165.0 https://github.com/openai/codex.git /tmp/codex-upstream
git -C /tmp/codex-upstream rev-parse HEAD
# 360319d8dee62d87e2d59fc0c21805d79519cfed
git -C /tmp/codex-upstream rev-parse HEAD^{tree}
# 307e4fa207fe563a11f3f7af09cee6fccd2d8376
```

The fork base commit `c9ca102` is not tree-identical to the upstream tag.
The audit therefore diffs the fork HEAD directly against the upstream commit,
not against the fork's first commit. The exact delta is in
[`docs/upstream-diff.tsv`](docs/upstream-diff.tsv).
