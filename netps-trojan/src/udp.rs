use netps_core::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, Take};

use crate::{consts, Endpoint, Error, Password, Result};

pub struct TrojanUdpSocket<S> {
    stream: S,
    password: Password,
}

impl<S> TrojanUdpSocket<S> {
    pub fn new(stream: S, password: &str) -> Self {
        let password = Password::from(password);

        Self { stream, password }
    }
}

impl<S> TrojanUdpSocket<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn handshake(&mut self) -> Result<()> {
        let stream = &mut self.stream;

        stream.write_all(&self.password.0).await?;
        stream.write_u16(consts::CRLF).await?;
        stream.write_u8(consts::CMD_UDP).await?;

        stream.write_u8(consts::ATYPE_V4).await?;
        stream.write_all(&[0, 0, 0, 0]).await?;
        stream.write_u16(0).await?;

        stream.write_u16(consts::CRLF).await?;
        stream.flush().await?;

        Ok(())
    }

    pub async fn send_to(&mut self, endpoint: &Endpoint, payload: &[u8]) -> Result<()>
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

    pub async fn recv_from(&mut self) -> Result<(Take<&'_ mut S>, Endpoint)> {
        let stream = &mut self.stream;

        let endpoint = Endpoint::read(stream).await?;
        let length = stream.read_u16().await?;

        let spliter = stream.read_u16().await?;

        if spliter != consts::CRLF {
            return Err(Error::WrongFormat);
        }

        Ok((stream.take(length.into()), endpoint))
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
