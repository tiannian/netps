use crate::{Endpoint, TrojanUdpSocket};

pub enum TrojanStream<'a, S> {
    Connect(Endpoint),
    Udp(TrojanUdpSocket<'a, S>),
}
