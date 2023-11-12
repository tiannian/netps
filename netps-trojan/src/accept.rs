use netps_core::io::{AsyncRead, AsyncReadExt, AsyncWrite};

use crate::{consts, Endpoint, Error, Password, Result, TrojanStream};

pub async fn accept<S>(stream: S) -> Result<(TrojanStream<S>, Endpoint, Password)>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut stream = stream;

    let mut hash = Password::default();

    stream.read_exact(&mut hash.0).await?;

    let spliter = stream.read_u16().await?;
    if spliter != consts::CRLF {
        return Err(Error::WrongFormat);
    }

    let cmd = stream.read_u8().await?;

    let endpoint = Endpoint::read(&mut stream).await?;

    let stream = match cmd {
        consts::CMD_CONNECT => {
            let spliter = stream.read_u16().await?;
            if spliter != consts::CRLF {
                return Err(Error::WrongFormat);
            }
            TrojanStream::Connect(stream)
        }
        consts::CMD_UDP => {
            let length = stream.read_u16().await?;
            if spliter != consts::CRLF {
                return Err(Error::WrongFormat);
            }
            TrojanStream::Udp(stream.take(length.into()))
        }
        _ => return Err(Error::UnknownCmd),
    };

    Ok((stream, endpoint, hash))
}
