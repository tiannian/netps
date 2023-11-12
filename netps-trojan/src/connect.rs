use netps_core::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, Take},
    Endpoint,
};

use crate::{
    PasswordHash, Result, TrojanStream, ATYPE_DOMAIN_NAME, ATYPE_V4, ATYPE_V6, CMD_CONNECT, CRLF,
};

pub async fn connect<S>(
    stream: S,
    endpoint: &Endpoint,
    passhash: &PasswordHash,
) -> Result<TrojanStream<S>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut stream = stream;

    stream.write_all(&passhash.0).await?;
    stream.write_u16(CRLF).await?;
    stream.write_u8(CMD_CONNECT).await?;
    match endpoint {
        Endpoint::V4(a, p) => {
            stream.write_u8(ATYPE_V4).await?;
            stream.write_all(a).await?;
            stream.write_u16(*p).await?;
        }
        Endpoint::V6(a, p) => {
            stream.write_u8(ATYPE_V6).await?;
            stream.write_all(a).await?;
            stream.write_u16(*p).await?;
        }
        Endpoint::DomainName(a, p) => {
            stream.write_u8(ATYPE_DOMAIN_NAME).await?;
            stream.write_u8(a.len() as u8).await?;
            stream.write_all(a).await?;
            stream.write_u16(*p).await?;
        }
    }
    stream.write_u16(CRLF).await?;

    Ok(TrojanStream::Connect(stream))
}
