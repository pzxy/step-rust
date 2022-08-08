/// Future 执行的原理
/// Future 被 poll 调用后才会执行,如果本次没有执行完,就会返回 Pending,
/// 并且安排一个wake 函数,调用 wake 后,执行器就是再次调用 poll,虽然是再次
/// 调用 poll,但是执行器通过 wake函数是知道到底哪个 Future 执行的.因此不必再轮训一次.
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_da {

        }
    }
}


