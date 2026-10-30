use maxcode_core::MaxcodeThread;
use maxcode_core::NewThread;
use maxcode_core::StartIfIdleSubmission;
use maxcode_core::StartThreadOptions;
use maxcode_core::ThreadManager;
use maxcode_core::TurnInputRequest;
use maxcode_core::config::Config;
use maxcode_protocol::ThreadId;
use maxcode_protocol::error::MaxcodeErr;
use maxcode_protocol::error::Result as MaxcodeResult;
use maxcode_protocol::protocol::W3cTraceContext;
use maxcode_protocol::user_input::UserInput;
use std::sync::Arc;
use std::sync::Weak;

/// A fully resolved agent invocation.
///
/// Agent discovery owns rendering `prompt`, including any selected skill
/// references. The runtime only starts that prompt in isolated forked context.
pub struct AgentInvocation {
    pub config: Config,
    pub prompt: String,
    pub parent_trace: Option<W3cTraceContext>,
}

/// A spawned agent whose initial turn has been submitted.
pub struct AgentRun {
    pub thread_id: ThreadId,
    pub turn_id: String,
    pub thread: Arc<MaxcodeThread>,
}

/// Runs resolved agents in legacy threads forked by the owning [`ThreadManager`].
#[derive(Clone)]
pub struct AgentRunner {
    thread_manager: Weak<ThreadManager>,
}

impl AgentRunner {
    pub fn new(thread_manager: Weak<ThreadManager>) -> Self {
        Self { thread_manager }
    }

    /// Starts a resolved agent in a fork of a legacy `parent_thread_id`.
    ///
    /// Paginated parents are rejected by [`ThreadManager::spawn_legacy_subagent`].
    pub async fn start(
        &self,
        parent_thread_id: ThreadId,
        invocation: AgentInvocation,
    ) -> MaxcodeResult<AgentRun> {
        let AgentInvocation {
            config,
            prompt,
            parent_trace,
        } = invocation;
        if prompt.trim().is_empty() {
            return Err(MaxcodeErr::InvalidRequest(
                "agent prompt must not be empty".to_string(),
            ));
        }

        let thread_manager = self
            .thread_manager
            .upgrade()
            .ok_or_else(|| MaxcodeErr::UnsupportedOperation("thread manager dropped".to_string()))?;
        let NewThread {
            thread_id, thread, ..
        } = thread_manager
            .spawn_legacy_subagent(
                parent_thread_id,
                StartThreadOptions {
                    parent_trace: parent_trace.clone(),
                    ..StartThreadOptions::new(config)
                },
            )
            .await?;
        let turn_id = match thread
            .start_turn_if_idle(
                TurnInputRequest::user_input(vec![UserInput::Text {
                    text: prompt,
                    text_elements: Vec::new(),
                }])
                .with_trace(parent_trace),
            )
            .await?
        {
            StartIfIdleSubmission::Started { turn_id, .. } => turn_id,
            StartIfIdleSubmission::NotSubmitted { reason } => {
                return Err(MaxcodeErr::InvalidRequest(format!(
                    "agent prompt was not submitted: {reason:?}"
                )));
            }
        };

        Ok(AgentRun {
            thread_id,
            turn_id,
            thread,
        })
    }
}
