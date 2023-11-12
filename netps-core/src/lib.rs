use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

pub use tokio::io;

pub enum Endpoint {
    V4([u8; 4], u16),
    V6([u8; 16], u16),
    DomainName(Vec<u8>, u16),
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint::V4([0u8; 4], 0)
    }
}

pub struct PacketHeader {
    pub from: Endpoint,
    pub to: Endpoint,
}

pub trait AsyncCommon {
    type Error: std::error::Error;

    type Stream: AsyncRead + AsyncWrite;
}

#[async_trait]
pub trait AsyncAcceptor: AsyncCommon {
    async fn accept<S>(
        &mut self,
        stream: S,
    ) -> Result<(Option<PacketHeader>, Self::Stream), Self::Error>
    where
        S: AsyncRead + AsyncWrite + Unpin;
}

#[async_trait]
pub trait AsyncClient: AsyncCommon {
    async fn conect(&mut self, target: Endpoint) -> Result<Self::Stream, Self::Error>;
}
