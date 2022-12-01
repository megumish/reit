use futures::Stream;

pub trait Socket<S>
where
    S: Stream,
{
    type BindingSocket: SocketBound<S>;
    fn bind(&self) -> Self::BindingSocket;
}

pub trait SocketBound<S>
where
    S: Stream,
{
    fn stream(&self) -> S;
}
