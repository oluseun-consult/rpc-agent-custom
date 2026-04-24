use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use futures::{StreamExt as _, future};
use rsa::RsaPrivateKey;
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
    pub(crate) private_key_path: Option<RsaPrivateKey>,
}

impl AgentServer {
    /// Runs the agent server.
    pub async fn run(self) -> Result<(), Error> {
        let mut listener =
            tarpc::serde_transport::tcp::listen(self.socket_addr, Json::default).await?;
        listener.config_mut().max_frame_length(usize::MAX);

        #[cfg(feature = "tracing")]
        tracing::info!("Listening on: {}", listener.local_addr());

        #[cfg(not(feature = "tracing"))]
        println!("Listening on: {}", listener.local_addr());

        listener
            .filter_map(|r| future::ready(r.ok()))
            .map(server::BaseChannel::with_defaults)
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
            private_key_path: None,
        }
    }
}

#[tarpc::service]
pub(crate) trait AgentWorker {
    async fn message(user_message: Message) -> Result<String, ApiError>;
}

impl AgentWorker for AgentServer {
    /// Handles a user message by passing it to the completion provider and returning the response.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "agent.message", skip(self, _context, user_message))
    )]
    async fn message(
        self,
        _context: ::tarpc::context::Context,
        user_message: Message,
    ) -> Result<String, ApiError> {
        let prompt: String =
            user_message.to_string(self.private_key_path.ok_or(ApiError::from(
                Error::AuthenticationError("Private Key Required".to_owned()),
            ))?)?;
        self.providers.chat(&prompt).await.map_err(ApiError::from)
    }
}
