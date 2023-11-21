use netps_core::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{consts, Endpoint, Error, Password, Result};

pub struct TrojanUdpSocket<'a, S> {
    pub(crate) stream: &'a mut S,
}

impl<'a, S> TrojanUdpSocket<'a, S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn connect(stream: &'a mut S, password: &str) -> Result<TrojanUdpSocket<'a, S>> {
        let password = Password::from(password);

        stream.write_all(&password.0).await?;
        stream.write_u16(consts::CRLF).await?;
        stream.write_u8(consts::CMD_UDP).await?;

        stream.write_u8(consts::ATYPE_V4).await?;
        stream.write_all(&[0, 0, 0, 0]).await?;
        stream.write_u16(0).await?;

        stream.write_u16(consts::CRLF).await?;
        stream.flush().await?;

        Ok(Self { stream })
    }
}

impl<S> TrojanUdpSocket<'_, S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn send_to(&mut self, payload: &[u8], endpoint: &Endpoint) -> Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let stream = &mut self.stream;

        endpoint.write(stream).await?;

        stream.write_u16(payload.len() as u16).await?;

        stream.write_u16(consts::CRLF).await?;

        stream.write_all(payload).await?;

        stream.flush().await?;

        Ok(())
    }

    pub async fn recv_from(&mut self, buf: &mut [u8]) -> Result<(usize, Endpoint)> {
        let stream = &mut self.stream;

        let endpoint = Endpoint::read(stream).await?;
        let length = stream.read_u16().await?;

        let spliter = stream.read_u16().await?;

        if spliter != consts::CRLF {
            return Err(Error::WrongFormat);
        }

        let n = if buf.len() < length as usize {
            buf.len()
        } else {
            length as usize
        };

        let buf = &mut buf[..n];

        let n = stream.read_exact(buf).await?;

        Ok((n, endpoint))
    }
}

#[cfg(test)]
mod tests {
    // use tokio::{io::AsyncReadExt, net::TcpStream};
    // use tokio_native_tls::{native_tls, TlsConnector};
    //
    // use crate::{Endpoint, TrojanUdpSocket};

    #[tokio::test]
    async fn test() {
        // let tcp = TcpStream::connect("127.0.0.1:12345").await.unwrap();
        //
        // let tls = native_tls::TlsConnector::builder()
        //     .danger_accept_invalid_certs(true)
        //     .build()
        //     .unwrap();
        //
        // let tls = TlsConnector::from(tls);
        //
        // let conn = tls.connect("www.baidu.com", tcp).await.unwrap();
        //
        // let mut socket = TrojanUdpSocket::new(conn, "password");
        //
        // socket.handshake().await.unwrap();
        //
        // socket
        //     .send_to(
        //         &Endpoint::V4([219, 154, 161, 136], 27643),
        //         "hello".as_bytes(),
        //     )
        //     .await
        //     .unwrap();
        //
        // let (mut stream, endpoint) = socket.recv_from().await.unwrap();
        // let mut s = String::with_capacity(stream.limit() as usize);
        //
        // stream.read_to_string(&mut s).await.unwrap();
        //
        // println!("read {s}, from: {endpoint:?}");
        //
        // socket
        //     .send_to(
        //         &Endpoint::V6(
        //             [
        //                 0x24, 0x08, 0x82, 0x20, 0x6a, 0x35, 0x11, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        //             ],
        //             27643,
        //         ),
        //         "hello".as_bytes(),
        //     )
        //     .await
        //     .unwrap();
        //
        // let (mut stream, endpoint) = socket.recv_from().await.unwrap();
        // let mut s = String::with_capacity(stream.limit() as usize);
        //
        // stream.read_to_string(&mut s).await.unwrap();
        //
        // println!("read {s}, from: {endpoint:?}")
    }
}
