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

## Verification log

Re-verified on 2026-11-01 by cloning the tag into a temporary directory and
running the commands below. The temporary clone is deleted after capture; it is
not part of this repository. Raw output, including the annotated tag object, the
tree hash, file counts, license hashes, and the derived diff counts, is committed
at [`docs/upstream-verify.log`](docs/upstream-verify.log):

```bash
TMP=$(mktemp -d)
git clone --depth 1 --branch rust-v0.165.0 https://github.com/openai/codex.git "$TMP/codex"
git -C "$TMP/codex" rev-parse HEAD
# 360319d8dee62d87e2d59fc0c21805d79519cfed
git -C "$TMP/codex" rev-parse HEAD^{tree}
# 307e4fa207fe563a11f3f7af09cee6fccd2d8376
git ls-remote https://github.com/openai/codex.git 'refs/tags/rust-v0.165.0*'
# e374836b2728d9b7aa981c2902649e640a2edaa8  refs/tags/rust-v0.165.0        (annotated tag)
# 360319d8dee62d87e2d59fc0c21805d79519cfed  refs/tags/rust-v0.165.0^{}     (commit)
```

`LICENSE` is byte-identical to upstream
(`sha256 d17f227e4df5da1600391338865ce0f3055211760a36688f816941d58232d8dc`).
`NOTICE` differs by design: the fork prepends its copyright and the Apache-2.0
section 4(b) modification statement, and keeps the upstream block.

The fork base commit `c9ca102` is not tree-identical to the upstream tag.
The audit therefore diffs the fork HEAD directly against the upstream commit,
not against the fork's first commit. The exact delta is in
[`docs/upstream-diff.tsv`](docs/upstream-diff.tsv).
