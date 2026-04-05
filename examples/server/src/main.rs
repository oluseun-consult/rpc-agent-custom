use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use rpc_agent::error::ApiError;
use tarpc::{client, context, serde_transport::tcp, tokio_serde::formats::Json};

#[tarpc::service]
pub trait AgentWorker {
    async fn message(user_message: String) -> Result<String, ApiError>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "127.0.0.1:5500".parse::<SocketAddr>()?;

    let transport = tcp::connect(server_addr, Json::default).await?;

    let client = AgentWorkerClient::new(client::Config::default(), transport).spawn();

    let mut ctx = context::current();
    ctx.deadline = Instant::now() + Duration::from_secs(120);
    let response = client
        .message(ctx, "what the price ticket to london".to_string())
        .await??;

    println!("Server response: {}", response);

    Ok(())
}
