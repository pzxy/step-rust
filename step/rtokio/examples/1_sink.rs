// Sink是生成数据，里面有四个方法，但是没有实现，我们直接用不到，我们主要用到的还是 SinkExt的方法
// poll_ready , poll_send, poll_flush, poll_close 

// SinkExt,是对Sink的的扩展，会自动继承Sink。然后调用Sink里面方法。
// 如果我们要使用sink中的方法，需要自己定义结构体实现。然后我们就可以用SinkExt和Sink的所有方法了。

// 因为 impl<T: ?Sized, Item> SinkExt<Item> for T where T: Sink<Item> {} ，
// Stream和StreamExt,AsyncRead和AyncReadExt都是一样的道理。

// SinkExt有四个常用方法：
// send()：写入Sink并flush
// feed()：写入Sink但不flush
// flush()：将已经写入Sink的数据flush
// send_all()：将给定的Stream中的零或多个数据全部写入Sink并一次或多次flush(自动决定何时flush)
fn main() {
    futures::executor::block_on(async {
        use futures::channel::mpsc;
        use futures::sink::SinkExt;
        use futures::stream::{self, StreamExt};

        let (tx, rx) = mpsc::channel(5);
        
        let mut tx = tx.with_flat_map(|x| stream::iter(vec![Ok(42); x]));
        // 这里 sender 实现了 sink
        tx.send(5).await.unwrap();
        drop(tx);
        let received: Vec<i32> = rx.collect().await;
        assert_eq!(received, vec![42, 42, 42, 42, 42]);
    });
}
