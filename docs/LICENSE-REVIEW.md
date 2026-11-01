# License review notes

Date: 2026-10-31; checked against the tree 2026-11-01
Status: **flags for lawyer review, not legal conclusions**

Provenance evidence for every count below: [`docs/upstream-verify.log`](upstream-verify.log)
(upstream clone, tag resolution, tree hash, license hashes, diff derivation).

## What changed

- `LICENSE`: unchanged. It remains the upstream Apache-2.0 text.
- `NOTICE`: modified. A fork copyright, an Apache-2.0 section 4(b) modification
  statement, and a pointer to `CHANGES.md` were added above the upstream notice.
  The entire upstream `NOTICE` block is preserved byte-for-byte, including its
  non-breaking spaces.
- `README.md`: rewritten for maxcode-engine. It is not a license header.
- Rust source license headers: **none changed**. Upstream `codex-rs` has 0
  `Copyright ... OpenAI` headers and 14 `SPDX-License-Identifier` headers; fork
  `maxcode-rs` has the same 0 and 14.
- 546 upstream files were removed. `LICENSE` and `NOTICE` were not among them.

Verification (all four commands and their output are in
[`docs/upstream-verify.log`](upstream-verify.log)):

```bash
UP=$(mktemp -d)/codex   # shallow clone of rust-v0.165.0
git -C "$UP" grep -l "SPDX-License-Identifier" -- codex-rs | wc -l   # 14
git grep -l "SPDX-License-Identifier" -- maxcode-rs | wc -l          # 14
git -C "$UP" grep -l "Copyright.*OpenAI" -- codex-rs | wc -l         # 0
git grep -l "Copyright.*OpenAI" -- maxcode-rs | wc -l                # 0
sha256sum "$UP/LICENSE" LICENSE                                     # identical
```

## Files whose license-bearing text changed

| File | Change | Review |
|---|---|---|
| `NOTICE` | Fork statement added; upstream notice preserved verbatim | Flag 1 |
| `README.md` | Rewritten; upstream Apache-2.0 text removed from the README (the root `LICENSE` remains) | Flag 6 |

No source file header was edited, so the list of files with changed license
headers is empty.

## Flags for lawyer review

1. **Added copyright and modification wording.** `NOTICE` adds
   `Copyright 2026 maxcode contributors` and a section 4(b) modification
   statement. Confirm the wording, whether a legal entity should replace
   “maxcode contributors”, and whether the statement is sufficiently prominent.
2. **Removed third-party trees.** `third_party/voice/` (46 files),
   `third_party/v8/` (22 files), and `patches/` (36 files) were removed. Confirm
   that no retained code or binary requires those notices, and that removing the
   files does not remove a notice for code still distributed.
3. **Embedded third-party text.** `maxcode-rs/worktree/tests/worktree.rs`
   contains inherited third-party license text (V8, PCRE2, and others) inside a
   test fixture. The file is otherwise unchanged except for the mechanical
   rebrand. Confirm whether that fixture must retain or reproduce the notices.
4. **Trademark and naming.** The name `maxcode-engine` has not been searched for
   trademark conflicts. Residual `OpenAI`/`Codex` references remain in
   `README.md` and `NOTICE` as attribution, and in factual provider/auth paths.
   Confirm nominative use and that no endorsement is implied.
5. **Preserved upstream NOTICE.** The upstream block is byte-identical, including
   non-breaking spaces. Confirm that this satisfies the NOTICE-preservation
   requirement and that the added fork block does not obscure it.
6. **README rewrite.** The upstream README contained Apache-2.0 text and links.
   The root `LICENSE` is unchanged, but confirm that removing the README copy is
   acceptable and that the new README still points to `LICENSE` and `NOTICE`.
7. **Removed security/community files.** Root `SECURITY.md` was removed along
   with `.github/`. Confirm that the fork has an adequate security contact and
   contribution policy elsewhere, or add one.
8. **Retained OpenAI service paths.** `maxcode-rs/login`, `chatgpt`,
   `agent-identity`, `backend-client`, and the default model provider still use
   `chatgpt.com`, `auth.openai.com`, `api.openai.com`, and related hosts. Confirm
   that the fork's use of those services and names complies with the applicable
   terms.
9. **Sentry DSN in feedback.** `maxcode-rs/feedback/src/lib.rs:63` contains a
   hard-coded Sentry DSN (`o33249.ingest.us.sentry.io`). Feedback is
   user-initiated, but confirm data ownership, consent, and whether the DSN must
   be removed from a fork. See `AUDIT.md`.
10. **Statsig credentials.** `maxcode-rs/otel/src/config.rs:9-11` contains a
    hard-coded Statsig endpoint and client key. The default is being disabled in
    this audit, but confirm whether the credential and endpoint should remain in
    the source at all.
11. **Help and documentation links.** `maxcode-rs/tui/src/history_cell/notices.rs`
    links to `help.openai.com`, `openai.com`, and ChatGPT documentation. Confirm
    that retaining these third-party links is acceptable.
12. **Project URLs.** `maxcode-rs/tui/src/model_migration.rs:470,493,522` used the
    renamed placeholder `https://www.maxcode.com/models/...`; it now points at
    `https://github.com/RavaniRoshan/maxcode-engine`. Confirm that this is the
    intended public project URL.
13. **Git attribution.** The fork's attribution trailer/URL was changed from
    OpenAI-owned addresses to a project address. Confirm the new wording and
    that the `.invalid` email cannot be mistaken for a working contact.

These flags are questions for counsel. This file does not approve or reject any
use.
