use crate::AgentServer;
use crate::agent::AgentWorker;
use crate::error::Error;
use crate::providers::CompletionProvider;
use rig::tool::Tool;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use crate::tools::NoTool;

struct DummyCompletionProvider;

#[async_trait::async_trait]
impl CompletionProvider for DummyCompletionProvider {
    async fn chat(&self, prompt: &str) -> Result<String, Error> {
        Ok(format!("dummy: {}", prompt))
    }
}

#[tokio::test]
async fn completion_provider_trait_works() {
    let provider = DummyCompletionProvider;
    let result = provider.chat("hello").await;
    assert_eq!(result.unwrap(), "dummy: hello");
}

#[test]
#[should_panic]
fn notool_definition_panics() {
    let notool = NoTool;
    let _ = futures::executor::block_on(notool.definition("test".to_string()));
}

#[test]
#[should_panic]
fn notool_call_panics() {
    let notool = NoTool;
    let _ = futures::executor::block_on(notool.call(()));
}

struct DummyProvider;

#[async_trait::async_trait]
impl CompletionProvider for DummyProvider {
    async fn chat(&self, prompt: &str) -> Result<String, Error> {
        Ok(format!("MOCK: {}", prompt))
    }
}

#[tokio::test]
async fn agent_server_mock_message() {
    let provider = Arc::new(Box::new(DummyProvider) as Box<dyn CompletionProvider>);
    let agent = AgentServer::new(SocketAddr::from((Ipv4Addr::LOCALHOST, 12345)), provider);
    let ctx = tarpc::context::current();
    let result = agent.message(ctx, "test mock".to_string()).await;
    assert_eq!(result.unwrap(), "MOCK: test mock");
}
