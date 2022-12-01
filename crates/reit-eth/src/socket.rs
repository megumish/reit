use futures::Stream;
use reit_socket::{Socket, SocketBound};

pub struct EthernetSocket;

impl EthernetSocket {
    pub fn new() -> Self {
        Self
    }
}

pub struct EthernetBound;

impl SocketBound<EthernetStream> for EthernetBound {
    fn stream(&self) -> EthernetStream {
        todo!()
    }
}

pub struct EthernetPacket;

pub struct EthernetStream;

impl Stream for EthernetStream {
    type Item = EthernetPacket;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}

impl Socket<EthernetStream> for EthernetSocket {
    type BindingSocket = EthernetBound;

    fn bind(&self) -> Self::BindingSocket {
        todo!()
    }
}
