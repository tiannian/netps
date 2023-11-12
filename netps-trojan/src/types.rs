use netps_core::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use sha2::Digest;

use crate::{Error, Result};

#[derive(Debug)]
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

impl Endpoint {
    pub async fn write<S>(&self, stream: &mut S) -> Result<()>
    where
        S: AsyncWrite + Unpin,
    {
        match self {
            Endpoint::V4(a, p) => {
                stream.write_u8(consts::ATYPE_V4).await?;
                stream.write_all(a).await?;
                stream.write_u16(*p).await?;
            }
            Endpoint::V6(a, p) => {
                stream.write_u8(consts::ATYPE_V6).await?;
                stream.write_all(a).await?;
                stream.write_u16(*p).await?;
            }
            Endpoint::DomainName(a, p) => {
                stream.write_u8(consts::ATYPE_DOMAIN_NAME).await?;
                stream.write_u8(a.len() as u8).await?;
                stream.write_all(a).await?;
                stream.write_u16(*p).await?;
            }
        }
        Ok(())
    }

    pub async fn read<S>(stream: &mut S) -> Result<Endpoint>
    where
        S: AsyncRead + Unpin,
    {
        let addr_type = stream.read_u8().await?;

        let endpoint = match addr_type {
            consts::ATYPE_V4 => {
                let mut addr = [0u8; 4];
                stream.read_exact(&mut addr).await?;
                let port = stream.read_u16().await?;
                Endpoint::V4(addr, port)
            }
            consts::ATYPE_DOMAIN_NAME => {
                let length = stream.read_u8().await?;
                let mut addr = vec![0u8; length as usize];
                stream.read_exact(&mut addr).await?;
                let port = stream.read_u16().await?;
                Endpoint::DomainName(addr, port)
            }
            consts::ATYPE_V6 => {
                let mut addr = [0u8; 16];
                stream.read_exact(&mut addr).await?;
                let port = stream.read_u16().await?;
                Endpoint::V6(addr, port)
            }
            _ => return Err(Error::UnknownAddressType),
        };

        Ok(endpoint)
    }
}

#[derive(Debug)]
pub struct Password(pub(crate) [u8; 56]);

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        let hash = sha2::Sha224::digest(value);
        let hash = hex::encode(hash);

        let mut res = [0u8; 56];

        res.copy_from_slice(hash.as_bytes());
        Self(res)
    }
}

impl Default for Password {
    fn default() -> Self {
        Self([0u8; 56])
    }
}

pub mod consts {
    pub const CRLF: u16 = 0x0d0a;

    pub const CMD_CONNECT: u8 = 0x01;
    pub const CMD_UDP: u8 = 0x03;

    pub const ATYPE_V4: u8 = 0x01;
    pub const ATYPE_DOMAIN_NAME: u8 = 0x03;
    pub const ATYPE_V6: u8 = 0x04;
}
