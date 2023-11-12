use std::env;

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let listen = env::var("LISTEN_ADDR").unwrap();

    let sock = UdpSocket::bind(listen).await.unwrap();
    let mut buf = [0; 1024];

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await.unwrap();

        println!("recv data from: {addr} {:?}", &buf[..len]);

        sock.send_to(&buf[..len], addr).await.unwrap();
    }
}
