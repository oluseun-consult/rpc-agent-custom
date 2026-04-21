use crate::ToolWrapper;
use crate::builder::AgentServerBuilder;
use crate::error::Error;
use crate::providers::Providers;

#[test]
fn providers_from_str() {
    assert_eq!(Providers::from("ollama").to_string(), "ollama");
    assert_eq!(Providers::from("openai").to_string(), "openai");
}

#[test]
#[should_panic]
fn providers_from_str_invalid() {
    let _ = Providers::from("invalid");
}

#[test]
fn tool_wrapper_new() {
    struct DummyTool;
    impl rig::tool::Tool for DummyTool {
        const NAME: &'static str = "dummy";
        type Error = Error;
        type Args = ();
        type Output = ();
        async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
            panic!("not used")
        }
        async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
            Ok(())
        }
    }
    let _ = ToolWrapper::new(DummyTool);
}

#[tokio::test]
async fn agent_server_builder_success() {
    let builder = AgentServerBuilder::new(8080, Providers::Ollama, "sys", "model");
    let server = builder.build();
    assert!(server.await.is_ok());
}

#[tokio::test]
#[should_panic]
async fn agent_server_builder_openai_missing_api_key() {
    let builder = AgentServerBuilder::new(8080, Providers::OpenAI, "sys", "model");
    // Should panic due to missing API key
    let _ = builder.build().await.unwrap();
}

#[tokio::test]
async fn agent_server_builder_openai_with_api_key() {
    let builder = AgentServerBuilder::new(8080, Providers::OpenAI, "sys", "model").api_key("dummy");
    let server = builder.build();
    assert!(server.await.is_ok());
}
