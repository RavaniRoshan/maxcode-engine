use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use maxcode_analytics::AnalyticsEventsClient;
use maxcode_analytics::AnalyticsJsonRpcError;
use maxcode_analytics::InputError;
use maxcode_analytics::TurnSteerRequestError;
use maxcode_app_server_protocol::Account;
use maxcode_app_server_protocol::AccountLoginCompletedNotification;
use maxcode_app_server_protocol::AccountTokenUsageDailyBucket;
use maxcode_app_server_protocol::AccountTokenUsageSummary;
use maxcode_app_server_protocol::AccountUpdatedNotification;
use maxcode_app_server_protocol::AddCreditsNudgeCreditType;
use maxcode_app_server_protocol::AddCreditsNudgeEmailStatus;
use maxcode_app_server_protocol::AdditionalContextEntry;
use maxcode_app_server_protocol::AdditionalContextKind;
use maxcode_app_server_protocol::AppListUpdatedNotification;
use maxcode_app_server_protocol::AppSummary;
use maxcode_app_server_protocol::AppTemplateSummary;
use maxcode_app_server_protocol::AppTemplateUnavailableReason;
use maxcode_app_server_protocol::AppsInstalledParams;
use maxcode_app_server_protocol::AppsInstalledResponse;
use maxcode_app_server_protocol::AppsListParams;
use maxcode_app_server_protocol::AppsListResponse;
use maxcode_app_server_protocol::AppsReadParams;
use maxcode_app_server_protocol::AppsReadResponse;
use maxcode_app_server_protocol::AskForApproval;
use maxcode_app_server_protocol::AuthMode;
use maxcode_app_server_protocol::CancelLoginAccountParams;
use maxcode_app_server_protocol::CancelLoginAccountResponse;
use maxcode_app_server_protocol::CancelLoginAccountStatus;
use maxcode_app_server_protocol::ClientInfo;
use maxcode_app_server_protocol::ClientRequest;
use maxcode_app_server_protocol::ClientResponsePayload;
use maxcode_app_server_protocol::MaxcodeErrorInfo;
use maxcode_app_server_protocol::CollaborationModeListParams;
use maxcode_app_server_protocol::CollaborationModeListResponse;
use maxcode_app_server_protocol::CommandExecParams;
use maxcode_app_server_protocol::CommandExecResizeParams;
use maxcode_app_server_protocol::CommandExecTerminateParams;
use maxcode_app_server_protocol::CommandExecWriteParams;
use maxcode_app_server_protocol::ConfigWarningNotification;
use maxcode_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use maxcode_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use maxcode_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use maxcode_app_server_protocol::ConversationGitInfo;
use maxcode_app_server_protocol::ConversationSummary;
use maxcode_app_server_protocol::DeprecationNoticeNotification;
use maxcode_app_server_protocol::DynamicToolFunctionSpec;
use maxcode_app_server_protocol::DynamicToolNamespaceTool;
use maxcode_app_server_protocol::DynamicToolSpec;
use maxcode_app_server_protocol::EnvironmentAddParams;
use maxcode_app_server_protocol::EnvironmentAddResponse;
use maxcode_app_server_protocol::EnvironmentInfoParams;
use maxcode_app_server_protocol::EnvironmentInfoResponse;
use maxcode_app_server_protocol::EnvironmentShellInfo;
use maxcode_app_server_protocol::EnvironmentStatusKind;
use maxcode_app_server_protocol::EnvironmentStatusParams;
use maxcode_app_server_protocol::EnvironmentStatusResponse;
use maxcode_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use maxcode_app_server_protocol::ExperimentalFeatureListParams;
use maxcode_app_server_protocol::ExperimentalFeatureListResponse;
use maxcode_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use maxcode_app_server_protocol::FeedbackUploadParams;
use maxcode_app_server_protocol::FeedbackUploadResponse;
use maxcode_app_server_protocol::GetAccountParams;
use maxcode_app_server_protocol::GetAccountRateLimitsResponse;
use maxcode_app_server_protocol::GetAccountResponse;
use maxcode_app_server_protocol::GetAccountTokenUsageParams;
use maxcode_app_server_protocol::GetAccountTokenUsageResponse;
use maxcode_app_server_protocol::GetAuthStatusParams;
use maxcode_app_server_protocol::GetAuthStatusResponse;
use maxcode_app_server_protocol::GetConversationSummaryParams;
use maxcode_app_server_protocol::GetConversationSummaryResponse;
use maxcode_app_server_protocol::GetWorkspaceMessagesResponse;
use maxcode_app_server_protocol::GitDiffToRemoteParams;
use maxcode_app_server_protocol::GitDiffToRemoteResponse;
use maxcode_app_server_protocol::GitInfo as ApiGitInfo;
use maxcode_app_server_protocol::HookHandlerMetadata;
use maxcode_app_server_protocol::HookMetadata;
use maxcode_app_server_protocol::HooksListParams;
use maxcode_app_server_protocol::HooksListResponse;
use maxcode_app_server_protocol::InitializeParams;
use maxcode_app_server_protocol::InitializeResponse;
use maxcode_app_server_protocol::InstalledApp;
use maxcode_app_server_protocol::JSONRPCErrorError;
use maxcode_app_server_protocol::ListMcpServerStatusParams;
use maxcode_app_server_protocol::ListMcpServerStatusResponse;
use maxcode_app_server_protocol::LoginAccountParams;
use maxcode_app_server_protocol::LoginAccountResponse;
use maxcode_app_server_protocol::LoginApiKeyParams;
use maxcode_app_server_protocol::LoginAppBrand;
use maxcode_app_server_protocol::LogoutAccountResponse;
use maxcode_app_server_protocol::MarketplaceAddParams;
use maxcode_app_server_protocol::MarketplaceAddResponse;
use maxcode_app_server_protocol::MarketplaceInterface;
use maxcode_app_server_protocol::MarketplaceRemoveParams;
use maxcode_app_server_protocol::MarketplaceRemoveResponse;
use maxcode_app_server_protocol::MarketplaceUpgradeErrorInfo;
use maxcode_app_server_protocol::MarketplaceUpgradeParams;
use maxcode_app_server_protocol::MarketplaceUpgradeResponse;
use maxcode_app_server_protocol::McpResourceReadParams;
use maxcode_app_server_protocol::McpResourceReadResponse;
use maxcode_app_server_protocol::McpServerOauthClientRegistration;
use maxcode_app_server_protocol::McpServerOauthLoginCompletedNotification;
use maxcode_app_server_protocol::McpServerOauthLoginParams;
use maxcode_app_server_protocol::McpServerOauthLoginResponse;
use maxcode_app_server_protocol::McpServerRefreshResponse;
use maxcode_app_server_protocol::McpServerStatus;
use maxcode_app_server_protocol::McpServerStatusDetail;
use maxcode_app_server_protocol::McpServerToolCallParams;
use maxcode_app_server_protocol::McpServerToolCallResponse;
use maxcode_app_server_protocol::MemoryResetResponse;
use maxcode_app_server_protocol::MockExperimentalMethodParams;
use maxcode_app_server_protocol::MockExperimentalMethodResponse;
use maxcode_app_server_protocol::ModelListParams;
use maxcode_app_server_protocol::ModelListResponse;
use maxcode_app_server_protocol::PermissionProfileListParams;
use maxcode_app_server_protocol::PermissionProfileListResponse;
use maxcode_app_server_protocol::PermissionProfileSummary;
use maxcode_app_server_protocol::PluginDetail;
use maxcode_app_server_protocol::PluginInstallParams;
use maxcode_app_server_protocol::PluginInstallResponse;
use maxcode_app_server_protocol::PluginInstalledParams;
use maxcode_app_server_protocol::PluginInstalledResponse;
use maxcode_app_server_protocol::PluginInterface;
use maxcode_app_server_protocol::PluginListMarketplaceKind;
use maxcode_app_server_protocol::PluginListParams;
use maxcode_app_server_protocol::PluginListResponse;
use maxcode_app_server_protocol::PluginMarketplaceEntry;
use maxcode_app_server_protocol::PluginReadParams;
use maxcode_app_server_protocol::PluginReadResponse;
use maxcode_app_server_protocol::PluginShareCheckoutParams;
use maxcode_app_server_protocol::PluginShareCheckoutResponse;
use maxcode_app_server_protocol::PluginShareContext;
use maxcode_app_server_protocol::PluginShareDeleteParams;
use maxcode_app_server_protocol::PluginShareDeleteResponse;
use maxcode_app_server_protocol::PluginShareDiscoverability;
use maxcode_app_server_protocol::PluginShareListItem;
use maxcode_app_server_protocol::PluginShareListParams;
use maxcode_app_server_protocol::PluginShareListResponse;
use maxcode_app_server_protocol::PluginSharePrincipal;
use maxcode_app_server_protocol::PluginSharePrincipalType;
use maxcode_app_server_protocol::PluginShareSaveParams;
use maxcode_app_server_protocol::PluginShareSaveResponse;
use maxcode_app_server_protocol::PluginShareTarget;
use maxcode_app_server_protocol::PluginShareUpdateDiscoverability;
use maxcode_app_server_protocol::PluginShareUpdateTargetsParams;
use maxcode_app_server_protocol::PluginShareUpdateTargetsResponse;
use maxcode_app_server_protocol::PluginSkillReadParams;
use maxcode_app_server_protocol::PluginSkillReadResponse;
use maxcode_app_server_protocol::PluginSource;
use maxcode_app_server_protocol::PluginSummary;
use maxcode_app_server_protocol::PluginUninstallParams;
use maxcode_app_server_protocol::PluginUninstallResponse;
use maxcode_app_server_protocol::RateLimitResetCredit;
use maxcode_app_server_protocol::RateLimitResetCreditStatus;
use maxcode_app_server_protocol::RateLimitResetCreditsSummary;
use maxcode_app_server_protocol::RateLimitResetType;
use maxcode_app_server_protocol::RequestId;
use maxcode_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use maxcode_app_server_protocol::ReviewStartParams;
use maxcode_app_server_protocol::ReviewStartResponse;
use maxcode_app_server_protocol::ReviewTarget as ApiReviewTarget;
use maxcode_app_server_protocol::SandboxMode;
use maxcode_app_server_protocol::SendAddCreditsNudgeEmailParams;
use maxcode_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use maxcode_app_server_protocol::ServerNotification;
use maxcode_app_server_protocol::ServerRequestResolvedNotification;
use maxcode_app_server_protocol::SkillSummary;
use maxcode_app_server_protocol::SkillsConfigWriteParams;
use maxcode_app_server_protocol::SkillsConfigWriteResponse;
use maxcode_app_server_protocol::SkillsExtraRootsSetParams;
use maxcode_app_server_protocol::SkillsExtraRootsSetResponse;
use maxcode_app_server_protocol::SkillsListParams;
use maxcode_app_server_protocol::SkillsListResponse;
use maxcode_app_server_protocol::SortDirection;
use maxcode_app_server_protocol::Thread;
use maxcode_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use maxcode_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use maxcode_app_server_protocol::ThreadArchiveParams;
use maxcode_app_server_protocol::ThreadArchiveResponse;
use maxcode_app_server_protocol::ThreadArchivedNotification;
use maxcode_app_server_protocol::ThreadBackgroundTerminal;
use maxcode_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use maxcode_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use maxcode_app_server_protocol::ThreadBackgroundTerminalsListParams;
use maxcode_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use maxcode_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use maxcode_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use maxcode_app_server_protocol::ThreadClosedNotification;
use maxcode_app_server_protocol::ThreadCompactStartParams;
use maxcode_app_server_protocol::ThreadCompactStartResponse;
use maxcode_app_server_protocol::ThreadDecrementElicitationParams;
use maxcode_app_server_protocol::ThreadDecrementElicitationResponse;
use maxcode_app_server_protocol::ThreadDeleteParams;
use maxcode_app_server_protocol::ThreadDeleteResponse;
use maxcode_app_server_protocol::ThreadDeletedNotification;
use maxcode_app_server_protocol::ThreadForkParams;
use maxcode_app_server_protocol::ThreadForkResponse;
use maxcode_app_server_protocol::ThreadGoal;
use maxcode_app_server_protocol::ThreadGoalClearParams;
use maxcode_app_server_protocol::ThreadGoalClearResponse;
use maxcode_app_server_protocol::ThreadGoalClearedNotification;
use maxcode_app_server_protocol::ThreadGoalGetParams;
use maxcode_app_server_protocol::ThreadGoalGetResponse;
use maxcode_app_server_protocol::ThreadGoalSetParams;
use maxcode_app_server_protocol::ThreadGoalSetResponse;
use maxcode_app_server_protocol::ThreadGoalStatus;
use maxcode_app_server_protocol::ThreadGoalUpdatedNotification;
use maxcode_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use maxcode_app_server_protocol::ThreadHistoryMode;
use maxcode_app_server_protocol::ThreadIncrementElicitationParams;
use maxcode_app_server_protocol::ThreadIncrementElicitationResponse;
use maxcode_app_server_protocol::ThreadInjectItemsParams;
use maxcode_app_server_protocol::ThreadInjectItemsResponse;
use maxcode_app_server_protocol::ThreadItem;
use maxcode_app_server_protocol::ThreadItemEntry;
use maxcode_app_server_protocol::ThreadItemsListParams;
use maxcode_app_server_protocol::ThreadItemsListResponse;
use maxcode_app_server_protocol::ThreadItemsReadParams;
use maxcode_app_server_protocol::ThreadItemsReadResponse;
use maxcode_app_server_protocol::ThreadListCwdFilter;
use maxcode_app_server_protocol::ThreadListParams;
use maxcode_app_server_protocol::ThreadListResponse;
use maxcode_app_server_protocol::ThreadLoadedListParams;
use maxcode_app_server_protocol::ThreadLoadedListResponse;
use maxcode_app_server_protocol::ThreadMemoryModeSetParams;
use maxcode_app_server_protocol::ThreadMemoryModeSetResponse;
use maxcode_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use maxcode_app_server_protocol::ThreadMetadataUpdateParams;
use maxcode_app_server_protocol::ThreadMetadataUpdateResponse;
use maxcode_app_server_protocol::ThreadNameUpdatedNotification;
use maxcode_app_server_protocol::ThreadProjectUpdatedNotification;
use maxcode_app_server_protocol::ThreadReadParams;
use maxcode_app_server_protocol::ThreadReadResponse;
use maxcode_app_server_protocol::ThreadRealtimeAppendAudioParams;
use maxcode_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use maxcode_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use maxcode_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use maxcode_app_server_protocol::ThreadRealtimeAppendTextParams;
use maxcode_app_server_protocol::ThreadRealtimeAppendTextResponse;
use maxcode_app_server_protocol::ThreadRealtimeListVoicesResponse;
use maxcode_app_server_protocol::ThreadRealtimeStartParams;
use maxcode_app_server_protocol::ThreadRealtimeStartResponse;
use maxcode_app_server_protocol::ThreadRealtimeStartTransport;
use maxcode_app_server_protocol::ThreadRealtimeStopParams;
use maxcode_app_server_protocol::ThreadRealtimeStopResponse;
use maxcode_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use maxcode_app_server_protocol::ThreadResumeParams;
use maxcode_app_server_protocol::ThreadResumeResponse;
use maxcode_app_server_protocol::ThreadSearchOccurrence;
use maxcode_app_server_protocol::ThreadSearchOccurrencesParams;
use maxcode_app_server_protocol::ThreadSearchOccurrencesResponse;
use maxcode_app_server_protocol::ThreadSearchParams;
use maxcode_app_server_protocol::ThreadSearchResponse;
use maxcode_app_server_protocol::ThreadSearchResult;
use maxcode_app_server_protocol::ThreadSearchSortKey;
use maxcode_app_server_protocol::ThreadSearchTextRange;
use maxcode_app_server_protocol::ThreadSetNameParams;
use maxcode_app_server_protocol::ThreadSetNameResponse;
use maxcode_app_server_protocol::ThreadSettings;
use maxcode_app_server_protocol::ThreadSettingsUpdateParams;
use maxcode_app_server_protocol::ThreadSettingsUpdateResponse;
use maxcode_app_server_protocol::ThreadShellCommandParams;
use maxcode_app_server_protocol::ThreadShellCommandResponse;
use maxcode_app_server_protocol::ThreadSortKey;
use maxcode_app_server_protocol::ThreadSourceKind;
use maxcode_app_server_protocol::ThreadStartParams;
use maxcode_app_server_protocol::ThreadStartResponse;
use maxcode_app_server_protocol::ThreadStartedNotification;
use maxcode_app_server_protocol::ThreadStatus;
use maxcode_app_server_protocol::ThreadTimelineListParams;
use maxcode_app_server_protocol::ThreadTimelineListResponse;
use maxcode_app_server_protocol::ThreadTurnsListParams;
use maxcode_app_server_protocol::ThreadTurnsListResponse;
use maxcode_app_server_protocol::ThreadUnarchiveParams;
use maxcode_app_server_protocol::ThreadUnarchiveResponse;
use maxcode_app_server_protocol::ThreadUnarchivedNotification;
use maxcode_app_server_protocol::ThreadUnsubscribeParams;
use maxcode_app_server_protocol::ThreadUnsubscribeResponse;
use maxcode_app_server_protocol::ThreadUnsubscribeStatus;
use maxcode_app_server_protocol::Turn;
use maxcode_app_server_protocol::TurnEnvironmentParams;
use maxcode_app_server_protocol::TurnError;
use maxcode_app_server_protocol::TurnInterruptParams;
use maxcode_app_server_protocol::TurnInterruptResponse;
use maxcode_app_server_protocol::TurnItemsView;
use maxcode_app_server_protocol::TurnSettingsUpdateParams;
use maxcode_app_server_protocol::TurnSettingsUpdateResponse;
use maxcode_app_server_protocol::TurnSettingsUpdateStatus;
use maxcode_app_server_protocol::TurnStartParams;
use maxcode_app_server_protocol::TurnStartResponse;
use maxcode_app_server_protocol::TurnStatus;
use maxcode_app_server_protocol::TurnSteerParams;
use maxcode_app_server_protocol::TurnSteerResponse;
use maxcode_app_server_protocol::UserInput as V2UserInput;
use maxcode_app_server_protocol::WindowsSandboxReadiness;
use maxcode_app_server_protocol::WindowsSandboxReadinessResponse;
use maxcode_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use maxcode_app_server_protocol::WindowsSandboxSetupMode;
use maxcode_app_server_protocol::WindowsSandboxSetupStartParams;
use maxcode_app_server_protocol::WindowsSandboxSetupStartResponse;
use maxcode_app_server_protocol::WorkspaceMessage;
use maxcode_app_server_protocol::WorkspaceMessageType;
use maxcode_arg0::Arg0DispatchPaths;
use maxcode_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use maxcode_backend_client::Client as BackendClient;
use maxcode_backend_client::MaxcodeWorkspaceMessage as BackendWorkspaceMessage;
use maxcode_backend_client::MaxcodeWorkspaceMessageType as BackendWorkspaceMessageType;
use maxcode_backend_client::MaxcodeWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use maxcode_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use maxcode_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use maxcode_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use maxcode_backend_client::RequestError as BackendRequestError;
use maxcode_backend_client::TokenUsageProfile;
use maxcode_chatgpt::connectors;
use maxcode_config::CloudConfigBundleLoadError;
use maxcode_config::CloudConfigBundleLoadErrorCode;
use maxcode_config::ConfigLayerStack;
use maxcode_config::loader::project_trust_key;
use maxcode_config::types::McpServerTransportConfig;
use maxcode_connectors::AppInfo;
use maxcode_core::MaxcodeThread;
use maxcode_core::MaxcodeThreadSettingsOverrides;
use maxcode_core::ForkSnapshot;
use maxcode_core::McpManager;
use maxcode_core::NewThread;
use maxcode_core::NotSubmittedReason;
#[cfg(test)]
use maxcode_core::SessionMeta;
use maxcode_core::StartThreadOptions;
use maxcode_core::SteerSubmission;
use maxcode_core::ThreadConfigSnapshot;
use maxcode_core::ThreadManager;
use maxcode_core::TurnInput;
use maxcode_core::TurnInputRequest;
use maxcode_core::TurnInputSubmission;
use maxcode_core::TurnStartOptions;
use maxcode_core::config::Config;
use maxcode_core::config::ConfigOverrides;
use maxcode_core::config::NetworkProxyAuditMetadata;
use maxcode_core::config::edit::ConfigEdit;
use maxcode_core::config::edit::ConfigEditsBuilder;
use maxcode_core::connectors::AccessibleConnectorsStatus;
use maxcode_core::exec::ExecCapturePolicy;
use maxcode_core::exec::ExecExpiration;
use maxcode_core::exec::ExecParams;
use maxcode_core::exec_env::create_env;
use maxcode_core::path_utils;
#[cfg(test)]
use maxcode_core::read_head_for_summary;
use maxcode_core::sandboxing::SandboxPermissions;
use maxcode_core::truncate_rollout_after_turn_id;
use maxcode_core::truncate_rollout_before_turn_id;
use maxcode_core::validate_environment_ids_and_cwds;
use maxcode_core::windows_sandbox::WindowsSandboxLevelExt;
use maxcode_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use maxcode_core::windows_sandbox::WindowsSandboxSetupRequest;
use maxcode_core::windows_sandbox::sandbox_setup_is_complete;
use maxcode_core_plugins::PluginInstallError as CorePluginInstallError;
use maxcode_core_plugins::PluginInstallRequest;
use maxcode_core_plugins::PluginReadRequest;
use maxcode_core_plugins::PluginUninstallError as CorePluginUninstallError;
use maxcode_core_plugins::PluginsManager;
use maxcode_core_plugins::loader::load_plugin_apps;
use maxcode_core_plugins::manifest::PluginManifestInterface;
use maxcode_core_plugins::marketplace::MarketplaceError;
use maxcode_core_plugins::marketplace::MarketplacePluginSource;
use maxcode_core_plugins::marketplace_add::MarketplaceAddError;
use maxcode_core_plugins::marketplace_add::MarketplaceAddRequest;
use maxcode_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_maxcode_home;
use maxcode_core_plugins::marketplace_remove::MarketplaceRemoveError;
use maxcode_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use maxcode_core_plugins::marketplace_remove::remove_marketplace;
use maxcode_core_plugins::remote::RemoteMarketplace;
use maxcode_core_plugins::remote::RemoteMarketplaceSource;
use maxcode_core_plugins::remote::RemotePluginCatalogError;
use maxcode_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use maxcode_core_plugins::remote::RemotePluginServiceConfig;
use maxcode_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use maxcode_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use maxcode_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use maxcode_exec_server::EnvironmentManager;
use maxcode_exec_server::EnvironmentObservedStatus;
use maxcode_exec_server::LOCAL_ENVIRONMENT_ID;
use maxcode_exec_server::LOCAL_FS;
use maxcode_features::FEATURES;
use maxcode_features::Feature;
use maxcode_features::Stage;
use maxcode_feedback::MaxcodeFeedback;
use maxcode_feedback::FeedbackAttachmentPath;
use maxcode_feedback::FeedbackUploadOptions;
use maxcode_git_utils::git_diff_to_remote;
use maxcode_git_utils::resolve_root_git_project_for_trust;
use maxcode_login::AuthManager;
use maxcode_login::MAXCODE_OPEN_APP_URL;
use maxcode_login::MaxcodeAuth;
use maxcode_login::LoginSuccessPage;
use maxcode_login::LoginSuccessPageBrand;
use maxcode_login::ServerOptions as LoginServerOptions;
use maxcode_login::ShutdownHandle;
use maxcode_login::complete_device_code_login;
use maxcode_login::login_with_api_key;
use maxcode_login::login_with_bedrock_api_key;
use maxcode_login::oauth_client_id;
use maxcode_login::request_device_code;
use maxcode_login::run_login_server;
use maxcode_mcp::McpRuntimeContext;
use maxcode_mcp::McpServerStatusSnapshot;
use maxcode_mcp::McpSnapshotDetail;
use maxcode_mcp::collect_mcp_server_status_snapshot_with_detail;
use maxcode_mcp::discover_supported_scopes;
use maxcode_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use maxcode_mcp::resolve_oauth_scopes;
use maxcode_memories_write::clear_memory_roots_contents;
use maxcode_model_provider::create_model_provider;
use maxcode_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use maxcode_protocol::ThreadId;
use maxcode_protocol::config_types::CollaborationMode;
use maxcode_protocol::config_types::ForcedLoginMethod;
use maxcode_protocol::config_types::Personality;
use maxcode_protocol::config_types::ReasoningSummary;
use maxcode_protocol::config_types::TrustLevel;
use maxcode_protocol::config_types::WindowsSandboxLevel;
use maxcode_protocol::error::MaxcodeErr;
use maxcode_protocol::error::Result as MaxcodeResult;
#[cfg(test)]
use maxcode_protocol::items::TurnItem;
use maxcode_protocol::models::ResponseItem;
use maxcode_protocol::openai_models::ReasoningEffort;
use maxcode_protocol::protocol::AgentStatus;
use maxcode_protocol::protocol::ConversationAudioParams;
use maxcode_protocol::protocol::ConversationSpeechParams;
use maxcode_protocol::protocol::ConversationStartParams;
use maxcode_protocol::protocol::ConversationStartTransport;
use maxcode_protocol::protocol::ConversationTextParams;
use maxcode_protocol::protocol::EnvironmentConfigState;
use maxcode_protocol::protocol::EventMsg;
#[cfg(test)]
use maxcode_protocol::protocol::GitInfo as CoreGitInfo;
use maxcode_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use maxcode_protocol::protocol::Op;
use maxcode_protocol::protocol::RealtimeVoicesList;
use maxcode_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use maxcode_protocol::protocol::ReviewRequest;
use maxcode_protocol::protocol::ReviewTarget as CoreReviewTarget;
use maxcode_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use maxcode_protocol::protocol::SessionMetaLine;
use maxcode_protocol::protocol::TurnEnvironmentRequest;
use maxcode_protocol::protocol::W3cTraceContext;
use maxcode_protocol::protocol::strip_user_message_prefix;
use maxcode_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use maxcode_protocol::user_input::UserInput as CoreInputItem;
use maxcode_rmcp_client::McpOAuthClientRegistration;
use maxcode_rmcp_client::StreamableHttpRedirectMode;
use maxcode_rmcp_client::perform_oauth_login_return_url;
use maxcode_rollout::InitialHistory;
use maxcode_rollout::ResumedHistory;
use maxcode_rollout::RolloutItem;
use maxcode_rollout::persisted_rollout_item;
use maxcode_rollout::state_db::StateDbHandle;
use maxcode_rollout::state_db::reconcile_rollout;
use maxcode_state::ThreadMetadata;
use maxcode_state::log_db::LogDbLayer;
use maxcode_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use maxcode_thread_store::ArchiveThreadsParams as StoreArchiveThreadsParams;
use maxcode_thread_store::ClearableField as StoreClearableField;
use maxcode_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use maxcode_thread_store::GitInfoPatch as StoreGitInfoPatch;
use maxcode_thread_store::ItemSortKey as StoreItemSortKey;
use maxcode_thread_store::ListItemsParams as StoreListItemsParams;
use maxcode_thread_store::ListItemsPosition as StoreListItemsPosition;
use maxcode_thread_store::ListThreadsParams as StoreListThreadsParams;
use maxcode_thread_store::ListTimelineParams as StoreListTimelineParams;
use maxcode_thread_store::ListTurnsParams as StoreListTurnsParams;
use maxcode_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use maxcode_thread_store::LocalThreadStore;
use maxcode_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use maxcode_thread_store::ReadThreadParams as StoreReadThreadParams;
use maxcode_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use maxcode_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use maxcode_thread_store::SortDirection as StoreSortDirection;
use maxcode_thread_store::StoredThread;
use maxcode_thread_store::StoredTurn;
use maxcode_thread_store::StoredTurnItemsView;
use maxcode_thread_store::StoredTurnStatus;
use maxcode_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use maxcode_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use maxcode_thread_store::ThreadSortKey as StoreThreadSortKey;
use maxcode_thread_store::ThreadStore;
use maxcode_thread_store::ThreadStoreError;
use maxcode_utils_absolute_path::AbsolutePathBuf;
use maxcode_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use maxcode_app_server_protocol::ServerRequest;

pub(crate) mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
pub(crate) mod config_processor;
#[cfg(test)]
#[path = "request_processors/config_reload_tests.rs"]
mod config_reload_tests;
mod diagnostics;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod feedback_rollout_history;
mod feedback_thread_index;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_event_stream;
mod mcp_processor;
mod memory_status;
mod persisted_resume_settings;
mod plugins;
mod process_exec_processor;
mod projects;
mod remote_control_processor;
mod rollout;
mod search;
mod thread_attachments;
mod thread_enrichment;
mod thread_fork_goal;
mod thread_input;
mod thread_memory;
mod thread_processor;
mod thread_queue_processor;
mod thread_read_state;
mod thread_sections;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use diagnostics::read_server_diagnostics;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_event_stream::McpEventStreamReady;
pub(crate) use mcp_event_stream::McpEventStreams;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use projects::ProjectRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use thread_processor::ThreadResumeTarget;
pub(crate) use thread_queue_processor::ThreadQueueRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::PendingThreadUnloads;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

pub(crate) fn apply_live_thread_settings(
    thread: &mut Thread,
    config_snapshot: &ThreadConfigSnapshot,
) {
    thread.model = Some(config_snapshot.model.clone());
    thread.reasoning_effort = config_snapshot.reasoning_effort.clone();
    thread.environments = Some(
        config_snapshot
            .environment_selections()
            .iter()
            .map(Into::into)
            .collect(),
    );
}

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_requests(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentRequest>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut requests = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        requests.push(TurnEnvironmentRequest {
            environment_id,
            cwd,
            workspace_roots,
            config: EnvironmentConfigState::FromThread,
        });
    }
    validate_environment_ids_and_cwds(
        &thread_manager.environment_manager(),
        requests
            .iter()
            .map(|request| (request.environment_id.as_str(), &request.cwd)),
    )
    .map_err(environment_selection_error)?;
    Ok(Some(requests))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

pub(crate) mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if let Some(item) =
            persisted_rollout_item(item, maxcode_protocol::protocol::ThreadHistoryMode::Legacy)
        {
            builder.handle_rollout_item(item.as_ref());
        }
    }
    builder.finish()
}
