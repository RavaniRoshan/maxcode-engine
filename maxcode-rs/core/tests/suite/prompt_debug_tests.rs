use std::sync::Arc;

use anyhow::Result;
use maxcode_core::build_prompt_input;
use maxcode_core::config::ConfigBuilder;
use maxcode_core::config::ConfigOverrides;
use maxcode_extension_api::ExtensionRegistryBuilder;
use maxcode_home::MaxcodeHomeUserInstructionsProvider;
use maxcode_protocol::models::ContentItem;
use maxcode_protocol::models::ResponseItem;
use maxcode_protocol::user_input::UserInput;
use core_test_support::responses::strip_metadata;
use core_test_support::responses::strip_response_item_id;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

const TEST_INSTRUCTIONS: &str = "Global test instructions";

#[tokio::test]
async fn build_prompt_input_includes_context_and_user_message() -> Result<()> {
    let maxcode_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(maxcode_home.path().join("AGENTS.md"), TEST_INSTRUCTIONS)?;
    let config = ConfigBuilder::default()
        .maxcode_home(maxcode_home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            maxcode_self_exe: Some(std::env::current_exe()?),
            ..ConfigOverrides::default()
        })
        .build()
        .await?;
    let user_instructions_provider = Arc::new(MaxcodeHomeUserInstructionsProvider::new(
        config.maxcode_home.clone(),
    ));
    let input = build_prompt_input(
        config,
        vec![UserInput::Text {
            text: "hello from debug prompt".to_string(),
            text_elements: Vec::new(),
        }],
        /*state_db*/ None,
        Arc::new(ExtensionRegistryBuilder::new().build()),
        user_instructions_provider,
    )
    .await?;

    let expected_user_message = ResponseItem::Message {
        status: None,
        encrypted_content: None,
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "hello from debug prompt".to_string(),
        }],
        phase: None,
        internal_chat_message_metadata_passthrough: None,
    };
    assert_eq!(
        input
            .last()
            .cloned()
            .map(strip_metadata)
            .map(strip_response_item_id),
        Some(expected_user_message)
    );
    assert!(input.iter().any(|item| {
        let ResponseItem::Message { content, .. } = item else {
            return false;
        };

        content.iter().any(|content_item| {
            let (ContentItem::InputText { text } | ContentItem::OutputText { text, .. }) =
                content_item
            else {
                return false;
            };
            text.contains(TEST_INSTRUCTIONS)
        })
    }));
    Ok(())
}
