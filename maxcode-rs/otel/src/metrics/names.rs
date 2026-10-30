use std::sync::LazyLock;

pub const TOOL_CALL_COUNT_METRIC: &str = "maxcode.tool.call";
pub const TOOL_CALL_DURATION_METRIC: &str = "maxcode.tool.call.duration_ms";
pub const TOOL_CALL_UNIFIED_EXEC_METRIC: &str = "maxcode.tool.unified_exec";
pub const MULTI_AGENT_SPAWN_FAILURE_METRIC: &str = "maxcode.multi_agent.spawn.failure";
pub const MULTI_AGENT_SPAWN_PHASE_DURATION_METRIC: &str =
    "maxcode.multi_agent.spawn.phase.duration_ms";
pub const ARTIFACT_OPERATION_STARTED_METRIC: &str = "maxcode.artifact.operation.started";
pub const ARTIFACT_OPERATION_EXPECTED_OUTPUT_COUNT_METRIC: &str =
    "maxcode.artifact.operation.expected_output_count";
pub const PROCESS_START_METRIC: &str = "maxcode.process.start";
/// Caller-side exec-server RPC attempts, including local admission and transport failures.
pub const EXEC_SERVER_CLIENT_REQUEST_COUNT_METRIC: &str = "exec_server_client_requests_total";
pub const API_CALL_COUNT_METRIC: &str = "maxcode.api_request";
pub const API_CALL_DURATION_METRIC: &str = "maxcode.api_request.duration_ms";
pub const SSE_EVENT_COUNT_METRIC: &str = "maxcode.sse_event";
pub const SSE_EVENT_DURATION_METRIC: &str = "maxcode.sse_event.duration_ms";
pub const WEBSOCKET_REQUEST_COUNT_METRIC: &str = "maxcode.websocket.request";
pub const WEBSOCKET_CONTINUATION_COUNT_METRIC: &str = "maxcode.websocket.continuation";
pub const WEBSOCKET_REQUEST_DURATION_METRIC: &str = "maxcode.websocket.request.duration_ms";
pub const WEBSOCKET_EVENT_COUNT_METRIC: &str = "maxcode.websocket.event";
pub const WEBSOCKET_EVENT_DURATION_METRIC: &str = "maxcode.websocket.event.duration_ms";
pub const RESPONSES_API_OVERHEAD_DURATION_METRIC: &str = "maxcode.responses_api_overhead.duration_ms";
pub const RESPONSES_API_INFERENCE_TIME_DURATION_METRIC: &str =
    "maxcode.responses_api_inference_time.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TTFT_DURATION_METRIC: &str =
    "maxcode.responses_api_engine_iapi_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TTFT_DURATION_METRIC: &str =
    "maxcode.responses_api_engine_service_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TBT_DURATION_METRIC: &str =
    "maxcode.responses_api_engine_iapi_tbt.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TBT_DURATION_METRIC: &str =
    "maxcode.responses_api_engine_service_tbt.duration_ms";
pub const TURN_E2E_DURATION_METRIC: &str = "maxcode.turn.e2e_duration_ms";
pub const TURN_TTFT_DURATION_METRIC: &str = "maxcode.turn.ttft.duration_ms";
pub const TURN_TTFM_DURATION_METRIC: &str = "maxcode.turn.ttfm.duration_ms";
pub const TURN_NETWORK_PROXY_METRIC: &str = "maxcode.turn.network_proxy";
pub const TURN_MEMORY_METRIC: &str = "maxcode.turn.memory";
pub const TURN_TOOL_CALL_METRIC: &str = "maxcode.turn.tool.call";
pub const TURN_TOKEN_USAGE_METRIC: &str = "maxcode.turn.token_usage";
pub const TURN_COST_MICROUSD_METRIC: &str = "maxcode.turn.cost_microusd";
pub const TURN_UNIFIED_EXEC_RUNNING_PROCESSES_METRIC: &str =
    "maxcode.turn.unified_exec.running_processes";
pub const GUARDIAN_REVIEW_COUNT_METRIC: &str = "maxcode.guardian.review";
pub const GUARDIAN_REVIEW_DURATION_METRIC: &str = "maxcode.guardian.review.duration_ms";
pub const GUARDIAN_REVIEW_TTFT_DURATION_METRIC: &str = "maxcode.guardian.review.ttft.duration_ms";
pub const GUARDIAN_REVIEW_TOKEN_USAGE_METRIC: &str = "maxcode.guardian.review.token_usage";
pub const GUARDIAN_REVIEW_TRUNCATION_METRIC: &str = "maxcode.guardian.review.truncation";
pub const GUARDIAN_REVIEW_TRUNCATION_BYTES_METRIC: &str = "maxcode.guardian.review.truncation.bytes";
pub const GOAL_CREATED_METRIC: &str = "maxcode.goal.created";
pub const GOAL_RESUMED_METRIC: &str = "maxcode.goal.resumed";
pub const GOAL_COMPLETED_METRIC: &str = "maxcode.goal.completed";
pub const GOAL_BUDGET_LIMITED_METRIC: &str = "maxcode.goal.budget_limited";
pub const GOAL_USAGE_LIMITED_METRIC: &str = "maxcode.goal.usage_limited";
pub const GOAL_BLOCKED_METRIC: &str = "maxcode.goal.blocked";
pub const GOAL_TOKEN_COUNT_METRIC: &str = "maxcode.goal.token_count";
pub const GOAL_DURATION_SECONDS_METRIC: &str = "maxcode.goal.duration_s";
pub const PLUGIN_INSTALL_ELICITATION_SENT_METRIC: &str = "maxcode.plugins.install_elicitation.sent";
pub const PLUGIN_INSTALL_SUGGESTION_METRIC: &str = "maxcode.plugins.install_suggestion";
pub const CURATED_PLUGINS_STARTUP_SYNC_METRIC: &str = "maxcode.plugins.startup_sync";
pub const CURATED_PLUGINS_STARTUP_SYNC_FINAL_METRIC: &str = "maxcode.plugins.startup_sync.final";
pub const HOOK_RUN_METRIC: &str = "maxcode.hooks.run";
pub const HOOK_RUN_DURATION_METRIC: &str = "maxcode.hooks.run.duration_ms";
/// Duration for coarse startup phases, tagged by low-cardinality phase and status.
pub const STARTUP_PHASE_DURATION_METRIC: &str = "maxcode.startup.phase.duration_ms";
/// Total runtime of a startup prewarm attempt until it completes, tagged by final status.
pub const STARTUP_PREWARM_DURATION_METRIC: &str = "maxcode.startup_prewarm.duration_ms";
/// Age of the startup prewarm attempt when the first real turn resolves it, tagged by outcome.
pub const STARTUP_PREWARM_AGE_AT_FIRST_TURN_METRIC: &str =
    "maxcode.startup_prewarm.age_at_first_turn_ms";
pub const THREAD_STARTED_METRIC: &str = "maxcode.thread.started";
pub const THREAD_SKILLS_ENABLED_TOTAL_METRIC: &str = "maxcode.thread.skills.enabled_total";
pub const THREAD_SKILLS_KEPT_TOTAL_METRIC: &str = "maxcode.thread.skills.kept_total";
pub const THREAD_SKILLS_DESCRIPTION_TRUNCATED_CHARS_METRIC: &str =
    "maxcode.thread.skills.description_truncated_chars";
pub const THREAD_SKILLS_TRUNCATED_METRIC: &str = "maxcode.thread.skills.truncated";
// Tools measure the rendered block; kind=snapshot|delta distinguishes full catalogs from updates.
pub const THREAD_TOOLS_NAMESPACES_TOTAL_METRIC: &str = "maxcode.thread.tools.namespaces_total";
pub const THREAD_TOOLS_FRAGMENT_BYTES_METRIC: &str = "maxcode.thread.tools.fragment_bytes";

/// Finalized tool registrations per sampling request, tagged by exposure, tool_mode,
/// and effective incremental_tools mode.
pub const TOOL_REGISTRATIONS_METRIC: &str = "maxcode.tools.registered";
pub const TOOL_REGISTRATIONS_METRIC_BUCKETS: &[f64] = &[
    0.0, 1.0, 2.0, 5.0, 10.0, 20.0, 30.0, 50.0, 75.0, 100.0, 150.0, 200.0, 300.0, 500.0, 750.0,
    1_000.0, 2_000.0, 5_000.0, 10_000.0, 20_000.0,
];

/// Logarithmic boundaries up to 32,768 for tools bytes and namespace counts.
pub static THREAD_TOOLS_METRIC_BUCKETS: LazyLock<[f64; 511]> =
    LazyLock::new(|| context_log_buckets(/*max_exponent*/ 15.0));

/// Rendered incremental tool deltas by action and qualified tool; excludes initial catalogs.
/// schema_changed includes any definition change, such as description edits.
pub const TOOL_INCREMENTAL_UPDATES_METRIC: &str = "maxcode.tools.incremental_updates";

/// UTF-8 description bytes on replacement by qualified tool; skips missing descriptions.
pub const TOOL_SPEC_DESCRIPTION_BYTES_METRIC: &str = "maxcode.tools.spec_description_bytes";
/// Logarithmic boundaries up to 1 MiB.
pub static TOOL_SPEC_DESCRIPTION_BYTES_METRIC_BUCKETS: LazyLock<[f64; 511]> =
    LazyLock::new(|| context_log_buckets(/*max_exponent*/ 20.0));

pub static THREAD_SKILLS_COUNT_METRIC_BUCKETS: LazyLock<[f64; 513]> =
    LazyLock::new(|| std::array::from_fn(|index| index as f64));

/// Logarithmic boundaries up to 131,072 removed description characters.
pub static THREAD_SKILLS_DESCRIPTION_TRUNCATED_CHARS_BUCKETS: LazyLock<[f64; 511]> =
    LazyLock::new(|| context_log_buckets(/*max_exponent*/ 17.0));

pub const THREAD_SKILLS_TRUNCATED_BUCKETS: &[f64] = &[0.0, 1.0];

fn context_log_buckets(max_exponent: f64) -> [f64; 511] {
    std::array::from_fn(|index| {
        if index == 0 {
            0.0
        } else {
            2.0_f64.powf(max_exponent * (index - 1) as f64 / 509.0)
        }
    })
}
