/// select! 宏在单个任务中实现了多路复用的功能。
/// 总结：
/// 1. select最多64个分支，分支形式：<模式> = <async 表达式> => <结果处理>
/// 2. select实际上也是一个future，因为future只有放在future中才能被调用。
/// 3. select编译后的大概样子，如果这个select有两个分支的话：
/// ```rust
///  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
///         if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
///             println!("rx1 completed first with {:?}", val);
///             return Poll::Ready(());
///         }
///
///         if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
///             println!("rx2 completed first with {:?}", val);
///             return Poll::Ready(());
///         }
///
///         Poll::Pending
///     }
/// ```
/// 可以看到select编译后其实就是一个future，然后不断执行，哪个成功就返回哪个，剩下的分支就抛弃了。
/// 4. 在表达式中可以使用可变借用，也可以使用不可变借用。参考3，不可变借用可以理解，但是可变借用就不好理解了。
/// 其实也很好理解，因为select中的分支只会有一个被执行。
/// 5. 关与loop select的信息 [参考](https://course.rs/async-rust/tokio/select.html#%E5%BE%AA%E7%8E%AF)
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    // 生成一个任务，用于向 oneshot 发送一条消息
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });
    // 分支形式：<模式> = <async 表达式> => <结果处理>
    tokio::select! {
        socket = TcpStream::connect("localhost:3465") => {
            println!("Socket connected {:?}", socket);
        }
        msg = rx => {
            println!("received message first {:?}", msg);
        }
        // 收到了发送端发来的关闭信号
        // `select` 即将结束，此时，正在进行的 `some_operation()` 任务会被取消，任务自动完成，
        // tx1 被释放, 这里 tx1必须是可变的。
        //    _ = tx1.closed() => {}
        // 可以直接创建 future
        // msg = async {} => {}
        // 如果上面的分支都不执行，这里一定会执行，相当与default

         else => {
            println!("Both channels closed");
        }
    }
    // 可以有返回值，但是需要每个分支的返回值相同
    // let out = tokio::select! {
    //     res1 = computation1() => res1,
    //     res2 = computation2() => res2,
    // };
}
