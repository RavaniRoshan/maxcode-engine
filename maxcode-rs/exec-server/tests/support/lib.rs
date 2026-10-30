use maxcode_exec_server::EnvironmentManager;
use maxcode_http_client::HttpClientFactory;
use maxcode_http_client::OutboundProxyPolicy;

pub mod relay;

/// Builds a manager without environments using the legacy outbound HTTP policy.
pub fn environment_manager_without_environments() -> EnvironmentManager {
    EnvironmentManager::without_environments(HttpClientFactory::new(
        OutboundProxyPolicy::ReqwestDefault,
    ))
}
