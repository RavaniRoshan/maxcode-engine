//! Resolves an executor sandbox context to a concrete local sandbox implementation.

use maxcode_file_system::FileSystemSandboxContext;
use maxcode_file_system::WindowsSandboxSelection;
use maxcode_protocol::config_types::WindowsSandboxLevel;
use maxcode_protocol::models::PermissionProfile;
use maxcode_sandboxing::SandboxManager;
use maxcode_sandboxing::SandboxType;
use maxcode_sandboxing::SandboxablePreference;

pub(crate) fn select_sandbox(
    manager: &SandboxManager,
    permission_profile: &PermissionProfile,
    sandbox_context: &FileSystemSandboxContext,
    has_managed_network_requirements: bool,
) -> (SandboxType, Option<WindowsSandboxLevel>) {
    let windows_sandbox_level = match sandbox_context.windows_sandbox_selection {
        WindowsSandboxSelection::Disabled => WindowsSandboxLevel::Disabled,
        WindowsSandboxSelection::RestrictedToken => WindowsSandboxLevel::RestrictedToken,
        WindowsSandboxSelection::Elevated => WindowsSandboxLevel::Elevated,
        WindowsSandboxSelection::Mxc => return (SandboxType::WindowsMxc, None),
    };
    let windows_sandbox_type = match windows_sandbox_level {
        WindowsSandboxLevel::Disabled => SandboxType::None,
        WindowsSandboxLevel::RestrictedToken | WindowsSandboxLevel::Elevated => {
            SandboxType::WindowsRestrictedToken
        }
    };
    let sandbox_type = manager.select_initial(
        permission_profile,
        SandboxablePreference::Require,
        windows_sandbox_type,
        has_managed_network_requirements,
    );
    (sandbox_type, Some(windows_sandbox_level))
}
