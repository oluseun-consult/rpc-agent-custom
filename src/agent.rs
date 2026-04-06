use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use futures::{StreamExt as _, TryStreamExt, future};
use tarpc::{
    server::{self, Channel as _, incoming::Incoming as _},
    tokio_serde::formats::Json,
};

use crate::{
    error::{ApiError, Error},
    message::Message,
    providers::CompletionProvider,
};

#[derive(Clone)]
pub struct AgentServer {
    pub(crate) socket_addr: SocketAddr,
    pub(crate) providers: Arc<Box<dyn CompletionProvider>>,
}

impl AgentServer {
    /// Runs the agent server.
    pub async fn run(self) -> Result<(), Error> {
        let mut listener =
            tarpc::serde_transport::tcp::listen(self.socket_addr, Json::default).await?;
        listener.config_mut().max_frame_length(usize::MAX);

        println!("Listening on: {}", listener.local_addr());

        listener
            .map_err(|e| eprintln!("{}", e)) // TODO: Improve error handling.
            .filter_map(|r| future::ready(r.ok()))
            .map(server::BaseChannel::with_defaults)
            // Limit channels to 1 per IP.
            .max_channels_per_key(1, |t| {
                t.transport()
                    .peer_addr()
                    .map(|addr| addr.ip())
                    .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
            })
            .map(|channel| {
                channel.execute(self.clone().serve()).for_each(|f| async {
                    tokio::spawn(f);
                })
            })
            // Max 10 channels.
            .buffer_unordered(10)
            .for_each(|_| async {})
            .await;

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn new(
        socket_addr: SocketAddr,
        providers: Arc<Box<dyn CompletionProvider>>,
    ) -> Self {
        Self {
            socket_addr,
            providers,
        }
    }
}

#[tarpc::service]
pub(crate) trait AgentWorker {
    async fn message(user_message: Message) -> Result<String, ApiError>;
}

impl AgentWorker for AgentServer {
    /// Handles a user message by passing it to the completion provider and returning the response.
    async fn message(
        self,
        _context: ::tarpc::context::Context,
        user_message: Message,
    ) -> Result<String, ApiError> {
        println!("Message received");
        self.providers
            .chat(&user_message.to_string())
            .await
            .map_err(ApiError::from)
    }
}
