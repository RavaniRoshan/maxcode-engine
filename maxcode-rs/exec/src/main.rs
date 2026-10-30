// Release builds exceed rustc's default query depth when laying out the async
// MessageProcessor::handle_initialized_client_request future.
#![recursion_limit = "256"]

//! Entry-point for the `maxcode-exec` binary.
//!
//! When this CLI is invoked normally, it parses the standard `maxcode-exec` CLI
//! options and launches the non-interactive Maxcode agent. However, if it is
//! invoked with arg0 as `maxcode-linux-sandbox`, we instead treat the invocation
//! as a request to run the logic for the standalone `maxcode-linux-sandbox`
//! executable (i.e., parse any -s args and then run a *sandboxed* command under
//! Landlock + seccomp.
//!
//! This allows us to ship a completely separate set of functionality as part
//! of the `maxcode-exec` binary.
use clap::Parser;
use maxcode_arg0::Arg0DispatchPaths;
use maxcode_arg0::arg0_dispatch_or_else;
use maxcode_exec::Cli;
use maxcode_exec::run_main;
use maxcode_utils_cli::CliConfigOverrides;

#[derive(Parser, Debug)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    inner: Cli,
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|arg0_paths: Arg0DispatchPaths| async move {
        let top_cli = TopCli::parse();
        // Merge root-level overrides into inner CLI struct so downstream logic remains unchanged.
        let mut inner = top_cli.inner;
        inner
            .config_overrides
            .prepend_root_overrides(top_cli.config_overrides);

        run_main(inner, arg0_paths).await?;
        Ok(())
    })
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
