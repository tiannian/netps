use netps_core::io::{AsyncRead, AsyncWrite, Take};

pub enum TrojanStream<S> {
    Connect(S),
    Udp(Take<S>),
}

impl<S: AsyncRead + AsyncWrite + Unpin> TrojanStream<S> {
    pub fn to_next(self) -> Option<S> {
        match self {
            TrojanStream::Udp(s) => Some(s.into_inner()),
            _ => None,
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for TrojanStream<S> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut netps_core::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            TrojanStream::Connect(s) => {
                let pinned = std::pin::pin!(s);
                pinned.poll_read(cx, buf)
            }
            TrojanStream::Udp(s) => {
                let pinned = std::pin::pin!(s);
                pinned.poll_read(cx, buf)
            }
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for TrojanStream<S> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::result::Result<usize, std::io::Error>> {
        match self.get_mut() {
            TrojanStream::Connect(s) => {
                let pinned = std::pin::pin!(s);
                pinned.poll_write(cx, buf)
            }
            TrojanStream::Udp(s) => {
                let pinned = std::pin::pin!(s.get_mut());
                pinned.poll_write(cx, buf)
            }
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), std::io::Error>> {
        match self.get_mut() {
            TrojanStream::Connect(s) => {
                let pinned = std::pin::pin!(s);
                pinned.poll_flush(cx)
            }
            TrojanStream::Udp(s) => {
                let pinned = std::pin::pin!(s.get_mut());
                pinned.poll_flush(cx)
            }
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), std::io::Error>> {
        match self.get_mut() {
            TrojanStream::Connect(s) => {
                let pinned = std::pin::pin!(s);
                pinned.poll_shutdown(cx)
            }
            TrojanStream::Udp(s) => {
                let pinned = std::pin::pin!(s.get_mut());
                pinned.poll_shutdown(cx)
            }
        }
    }
}
