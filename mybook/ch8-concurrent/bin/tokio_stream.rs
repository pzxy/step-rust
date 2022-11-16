/// tokio-stream 可以让迭代器在迭代时异步进行。
/// 依赖  tokio-stream = { version = "0.1" }
/// stream 没搞明白，有点困了
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);
    // stream上调用next() 需要被pin
    tokio::pin!(stream);
    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}
