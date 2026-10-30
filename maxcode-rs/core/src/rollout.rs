use crate::config::Config;
pub use maxcode_rollout::ARCHIVED_SESSIONS_SUBDIR;
pub use maxcode_rollout::Cursor;
pub use maxcode_rollout::INTERACTIVE_SESSION_SOURCES;
pub use maxcode_rollout::RolloutRecorder;
pub use maxcode_rollout::RolloutRecorderParams;
pub use maxcode_rollout::SESSIONS_SUBDIR;
pub use maxcode_rollout::SessionMeta;
pub use maxcode_rollout::SortDirection;
pub use maxcode_rollout::ThreadItem;
pub use maxcode_rollout::ThreadSortKey;
pub use maxcode_rollout::ThreadsPage;
pub use maxcode_rollout::append_thread_name;
pub use maxcode_rollout::find_archived_thread_path_by_id_str;
#[deprecated(note = "use find_thread_path_by_id_str")]
pub use maxcode_rollout::find_conversation_path_by_id_str;
pub use maxcode_rollout::find_thread_meta_by_name_str;
pub use maxcode_rollout::find_thread_name_by_id;
pub use maxcode_rollout::find_thread_names_by_ids;
pub use maxcode_rollout::find_thread_path_by_id_str;
pub use maxcode_rollout::parse_cursor;
pub use maxcode_rollout::read_head_for_summary;
pub use maxcode_rollout::read_session_meta_line;
pub use maxcode_rollout::rollout_date_parts;

impl maxcode_rollout::RolloutConfigView for Config {
    fn maxcode_home(&self) -> &std::path::Path {
        self.maxcode_home.as_path()
    }

    fn sqlite_config(&self) -> &maxcode_state::SqliteConfig {
        self.sqlite_config()
    }

    fn cwd(&self) -> &std::path::Path {
        self.cwd.as_path()
    }

    fn model_provider_id(&self) -> &str {
        self.model_provider_id.as_str()
    }

    fn generate_memories(&self) -> bool {
        self.memories.generate_memories
    }
}

pub(crate) mod list {
    pub use maxcode_rollout::find_thread_path_by_id_str;
}

#[cfg(test)]
pub(crate) mod recorder {
    pub use maxcode_rollout::RolloutRecorder;
}

pub(crate) use crate::session_rollout_init_error::map_session_init_error;

pub(crate) mod truncation {
    pub(crate) use crate::thread_rollout_truncation::*;
}
