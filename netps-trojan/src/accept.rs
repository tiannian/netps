use netps_core::io::{AsyncRead, AsyncReadExt, AsyncWrite};

use crate::{consts, Endpoint, Error, Password, Result, TrojanStream, TrojanUdpSocket};

pub async fn accept<S>(stream: &mut S) -> Result<(TrojanStream<S>, Password)>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut hash = Password::default();

    stream.read_exact(&mut hash.0).await?;

    let spliter = stream.read_u16().await?;
    if spliter != consts::CRLF {
        return Err(Error::WrongFormat);
    }

    let cmd = stream.read_u8().await?;

    let endpoint = Endpoint::read(stream).await?;

    let stream = match cmd {
        consts::CMD_CONNECT => {
            let spliter = stream.read_u16().await?;
            if spliter != consts::CRLF {
                return Err(Error::WrongFormat);
            }
            TrojanStream::Connect(endpoint)
        }
        consts::CMD_UDP => {
            let spliter = stream.read_u16().await?;
            if spliter != consts::CRLF {
                return Err(Error::WrongFormat);
            }
            TrojanStream::Udp(TrojanUdpSocket { stream })
        }
        _ => return Err(Error::UnknownCmd),
    };

    Ok((stream, hash))
}
