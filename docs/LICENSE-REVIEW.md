# License review notes

Date: 2026-10-31
Status: **flags for lawyer review, not legal conclusions**

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
- 553 upstream files were removed. `LICENSE` and `NOTICE` were not among them.

Verification:

```bash
git -C /tmp/maxcode-engine-upstream grep -l "SPDX-License-Identifier" \
  360319d8dee62d87e2d59fc0c21805d79519cfed -- codex-rs | wc -l   # 14
git grep -l "SPDX-License-Identifier" -- maxcode-rs | wc -l       # 14
git -C /tmp/maxcode-engine-upstream grep -l "Copyright.*OpenAI" \
  360319d8dee62d87e2d59fc0c21805d79519cfed -- codex-rs | wc -l   # 0
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
2. **Removed third-party trees.** `third_party/voice/` (39 files),
   `third_party/v8/` (19 files), and `patches/` (36 files) were removed. Confirm
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
12. **Placeholder project URLs.** `maxcode-rs/tui/src/model_migration.rs` uses
    `https://www.maxcode.com/...`, which appears to be a renamed placeholder.
    Confirm the correct project URL or remove it. (Branding step replaces it.)
13. **Git attribution.** The fork's attribution trailer/URL was changed from
    OpenAI-owned addresses to a project address. Confirm the new wording and
    that the `.invalid` email cannot be mistaken for a working contact.

These flags are questions for counsel. This file does not approve or reject any
use.
