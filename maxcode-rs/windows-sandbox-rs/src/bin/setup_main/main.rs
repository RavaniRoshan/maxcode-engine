#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    maxcode_windows_sandbox::setup_helper_main()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("maxcode-windows-sandbox-setup is Windows-only");
}
