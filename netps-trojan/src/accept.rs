use netps_core::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite},
    Endpoint,
};

use crate::{
    Error, PasswordHash, Result, TrojanStream, ATYPE_DOMAIN_NAME, ATYPE_V4, ATYPE_V6, CMD_CONNECT,
    CMD_UDP, CRLF,
};

pub async fn accept<S>(stream: S) -> Result<(TrojanStream<S>, Endpoint, PasswordHash)>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut stream = stream;

    let mut hash = PasswordHash::default();

    stream.read_exact(&mut hash.0).await?;

    let spliter = stream.read_u16().await?;
    if spliter != CRLF {
        return Err(Error::WrongFormat);
    }

    let cmd = stream.read_u8().await?;
    let addr_type = stream.read_u8().await?;
    let endpoint = match addr_type {
        ATYPE_V4 => {
            let mut addr = [0u8; 4];
            stream.read_exact(&mut addr).await?;
            let port = stream.read_u16().await?;
            Endpoint::V4(addr, port)
        }
        ATYPE_DOMAIN_NAME => {
            let length = stream.read_u8().await?;
            let mut addr = vec![0u8; length as usize];
            stream.read_exact(&mut addr).await?;
            let port = stream.read_u16().await?;
            Endpoint::DomainName(addr, port)
        }
        ATYPE_V6 => {
            let mut addr = [0u8; 16];
            stream.read_exact(&mut addr).await?;
            let port = stream.read_u16().await?;
            Endpoint::V6(addr, port)
        }
        _ => return Err(Error::UnknownAddressType),
    };

    let stream = match cmd {
        CMD_CONNECT => {
            let spliter = stream.read_u16().await?;
            if spliter != CRLF {
                return Err(Error::WrongFormat);
            }
            TrojanStream::Connect(stream)
        }
        CMD_UDP => {
            let length = stream.read_u16().await?;
            if spliter != CRLF {
                return Err(Error::WrongFormat);
            }
            TrojanStream::Udp(stream.take(length.into()))
        }
        _ => return Err(Error::UnknownCmd),
    };

    Ok((stream, endpoint, hash))
}
