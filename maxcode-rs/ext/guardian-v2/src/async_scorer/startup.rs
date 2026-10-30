//! Resolves the classifier's own model budget and transport configuration.
//! Parent-model context overrides do not change the classifier allowance.

use std::sync::Arc;

use maxcode_core::ThreadManager;
use maxcode_core::config::Config;
use maxcode_core::cyber_access_program::ApiKeyCyberAccessPrograms;
use maxcode_extension_api::ThreadOriginator;
use maxcode_extension_api::ThreadStartInput;
use maxcode_features::Feature;
use maxcode_login::AgentIdentityAuthPolicy;
use maxcode_login::AuthManager;
use maxcode_login::MaxcodeAuth;
use maxcode_model_provider::create_model_provider;

use super::decisions::DecisionsError;
use super::decisions::DecisionsSampler;
use super::decisions::URL;
use super::sampler::LunaSamplerConfig;
use super::sampler::MODEL;

// Decisions setup failures are telemetry only; they never affect the baseline sampler.
pub(super) fn decisions_sampler(
    input: &ThreadStartInput<'_, Config>,
    auth_manager: &AuthManager,
) -> Option<DecisionsSampler> {
    use maxcode_http_client::ClientRouteClass;
    let result = decisions_api_key(
        &input.config.model_provider_id,
        input.config.model_provider.base_url.as_deref(),
        auth_manager,
    )
    .and_then(|key| {
        let client = input
            .config
            .http_client_factory()
            .build_client_without_request_logging(URL, ClientRouteClass::Api)
            .map_err(|_| DecisionsError::ClientSetup)?;
        DecisionsSampler::new(client, key, URL.to_owned())
    });
    match result {
        Ok(sampler) => Some(sampler),
        Err(error) => {
            if let Some(metrics) = input.extension_metrics.as_deref() {
                metrics.counter(
                    "maxcode.guardian_v2.decisions_comparison.setup_failure",
                    /*inc*/ 1,
                    &[("reason", super::metrics::decisions_failure_reason(error))],
                );
            }
            None
        }
    }
}

pub(super) fn decisions_api_key(
    provider_id: &str,
    base_url: Option<&str>,
    auth_manager: &AuthManager,
) -> Result<String, DecisionsError> {
    match std::env::var("MAXCODE_GUARDIAN_DECISIONS_API_KEY") {
        Ok(key) => Ok(key),
        Err(std::env::VarError::NotPresent) if provider_id == "openai" && base_url.is_none() => {
            auth_manager
                .auth_cached()
                .as_ref()
                .and_then(MaxcodeAuth::api_key)
                .map(str::to_owned)
                .or_else(maxcode_login::read_openai_api_key_from_env)
                .ok_or(DecisionsError::Credentials)
        }
        Err(std::env::VarError::NotPresent | std::env::VarError::NotUnicode(_)) => {
            Err(DecisionsError::Credentials)
        }
    }
}

pub(super) async fn sampler_config(
    input: &ThreadStartInput<'_, Config>,
    auth_manager: Arc<AuthManager>,
    manager: Option<Arc<ThreadManager>>,
) -> LunaSamplerConfig {
    let luna_model = if let Some(thread_manager) = manager {
        let mut model_config = input.config.to_models_manager_config();
        model_config.model_context_window = None;
        Some(
            thread_manager
                .get_models_manager()
                .get_model_info(MODEL, &model_config)
                .await,
        )
    } else {
        None
    };
    let max_input_tokens =
        luna_model
            .as_ref()
            .map_or(maxcode_guardian_context::DEFAULT_MAX_INPUT_TOKENS, |model| {
                maxcode_guardian_context::effective_input_token_limit(
                    model, /*configured_window*/ None,
                )
            });
    let luna_compaction_hash = luna_model.and_then(|model| model.comp_hash);
    LunaSamplerConfig {
        api_key_cyber_access_programs: ApiKeyCyberAccessPrograms::from_config(input.config),
        workspace_routing: input
            .thread_store
            .get_or_init(|| input.config.workspace_routing_context())
            .as_ref()
            .clone(),
        provider: create_model_provider(input.config.model_provider.clone(), Some(auth_manager)),
        http_client_factory: input.config.http_client_factory(),
        agent_identity_policy: if input.config.features.enabled(Feature::UseAgentIdentity) {
            AgentIdentityAuthPolicy::ChatGptAuth
        } else {
            AgentIdentityAuthPolicy::JwtOnly
        },
        session_source: input.session_source.clone(),
        session_id: input.session_store.level_id().to_string(),
        thread_id: input.thread_store.level_id().to_string(),
        originator: input
            .thread_store
            .get::<ThreadOriginator>()
            .map(|originator| originator.0.clone()),
        service_tier: input.config.service_tier.clone(),
        luna_compaction_hash,
        max_input_tokens,
        metrics: input.extension_metrics.clone(),
    }
}
