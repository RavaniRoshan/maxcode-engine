<p align="center">
  <img src="https://raw.githubusercontent.com/RavaniRoshan/maxcode/main/assets/max.svg" width="120" height="135" alt="Max, the maxcode mascot">
</p>

<h1 align="center">maxcode-engine</h1>

<p align="center">The execution engine behind <a href="https://github.com/RavaniRoshan/maxcode">maxcode</a>.</p>

---

## What this is

`maxcode-engine` is the process that edits files and runs commands for the
`maxcode` CLI. It is a self-contained Rust workspace. The Python CLI owns the
plan/verify loop; this engine owns the non-interactive `exec` session.

The engine reads its state from `MAXCODE_HOME` (default `~/.maxcode`). Provider
keys are read from the environment variable named in the provider config. Keys
are never written to disk by this project.

## Build

Requires a recent stable Rust toolchain and a C toolchain.

```bash
cd maxcode-rs
cargo build --release -p maxcode-cli --bin maxcode-engine
install -m 755 target/release/maxcode-engine ~/.local/bin/maxcode-engine
maxcode-engine --version
```

### Prebuilt binaries

Tag pushes build and publish two assets on the GitHub Release page:

- `maxcode-engine-linux-x86_64`
- `maxcode-engine-macos-arm64`
- `SHA256SUMS`

Download the binary for your platform and mark it executable. The release
workflow uses cargo's standard `release` profile; the `dev-small` profile below
is for local low-RAM builds only.

### Low-memory build

The workspace is large. On a machine with 8 GB of RAM, avoid release LTO and
parallel codegen. The checked-in `dev-small` profile is the fastest path to a
working binary:

```bash
CARGO_BUILD_JOBS=4 cargo build --profile dev-small -p maxcode-cli --bin maxcode-engine
```

For a smaller, faster release build without LTO:

```bash
CARGO_PROFILE_RELEASE_LTO=off \
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
CARGO_PROFILE_RELEASE_DEBUG=0 \
CARGO_BUILD_JOBS=2 \
cargo build --release -p maxcode-cli --bin maxcode-engine
```

If the linker is the bottleneck, Rust ships an LLD binary. Point the linker at
it without installing anything system-wide:

```bash
RUSTFLAGS="-C linker=$HOME/.local/bin/ld.lld -C linker-flavor=ld" \
cargo build --release -p maxcode-cli --bin maxcode-engine
```

Create that symlink once:

```bash
ln -sf "$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/^host: //p')/bin/rust-lld" \
  "$HOME/.local/bin/ld.lld"
```

Builds are resumable: if a build is interrupted, run the same command again and
Cargo reuses completed crates. Keep the `target/` directory between attempts.

## Use

```bash
maxcode-engine exec --json --skip-git-repo-check \
  -C /path/to/repo -s workspace-write \
  -m "$MODEL" "fix the failing test"
```

Provider configuration is passed with `-c` overrides, for example:

```bash
maxcode-engine exec -m "provider/model" \
  -c 'model_provider=maxcode' \
  -c 'model_providers.maxcode.base_url="https://openrouter.ai/api/v1"' \
  -c 'model_providers.maxcode.env_key="OPENROUTER_API_KEY"' \
  -c 'model_providers.maxcode.wire_api="responses"' \
  "fix the failing test"
```

Run `maxcode-engine exec --help` for the full flag list.

## Workspace

```text
maxcode-rs/
  cli/        maxcode-engine binary
  exec/       non-interactive session
  core/       agent loop and tools
  tui/        interactive terminal UI
  protocol/   event and app-server types
  config/     layered configuration
```

## Development

```bash
cargo check -p maxcode-cli
cargo test -p maxcode-exec
cargo fmt --check
```

The workspace pins dependencies in `Cargo.lock`. Do not delete it when
rebuilding: the `rama` alpha crates must move together.

## Legal

`maxcode-engine` is an independent fork of the Codex CLI (Apache-2.0) and is not
affiliated with OpenAI. The upstream license and notice are preserved in
[`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).
