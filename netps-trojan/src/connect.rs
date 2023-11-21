use netps_core::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::{consts, Endpoint, Password, Result};

pub async fn connect<S>(stream: &mut S, endpoint: &Endpoint, passhash: &Password) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    stream.write_all(&passhash.0).await?;
    stream.write_u16(consts::CRLF).await?;
    stream.write_u8(consts::CMD_CONNECT).await?;

    endpoint.write(stream).await?;

    stream.write_u16(consts::CRLF).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use hyper::{body::to_bytes, client::conn, Body, Request};
    // use netps_core::Endpoint;
    // use tokio::net::TcpStream;
    // use tokio_native_tls::{native_tls, TlsConnector};
    //
    // use crate::{connect, Password};

    #[tokio::test]
    async fn test() {
        // let tcp = TcpStream::connect("127.0.0.1:27655").await.unwrap();
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
        // let hash = Password::from("password");
        //
        // let name = "httpbin.org".as_bytes().to_vec();
        // let conn = connect(conn, &Endpoint::DomainName(name, 80), &hash)
        //     .await
        //     .unwrap();
        //
        // let (mut sender, conn) = conn::handshake(conn).await.unwrap();
        //
        // tokio::spawn(async move {
        //     if let Err(e) = conn.await {
        //         println!("Run: {e}")
        //     }
        // });
        //
        // let req = Request::get("http://httpbin.org/get")
        //     .header("User-Agent", "curl/8.4.0")
        //     .header("Host", "httpbin.org")
        //     .body(Body::empty())
        //     .unwrap();
        // let res = sender.send_request(req).await.unwrap();
        //
        // let body = res.into_body();
        //
        // let r = to_bytes(body).await.unwrap();
        //
        // println!("{}", String::from_utf8_lossy(&r));
        //
        // let req = Request::get("http://httpbin.org/get?key=value")
        //     .header("User-Agent", "curl/8.4.0")
        //     .header("Host", "httpbin.org")
        //     .body(Body::empty())
        //     .unwrap();
        // let res = sender.send_request(req).await.unwrap();
        //
        // let body = res.into_body();
        //
        // let r = to_bytes(body).await.unwrap();
        //
        // println!("{}", String::from_utf8_lossy(&r))
    }
}
