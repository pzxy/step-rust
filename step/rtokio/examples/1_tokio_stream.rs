// tokio中的stream trait和 future 包中的 stream trait是一样。
// tokio 重新定义了 StreamExt，接口是一样的。
// 不过看实现不一样，这个以后研究一样。
use tokio_stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(vec![1, 2, 3]);
    
    while let Some(value) = stream.next().await {
        println!("Got {}", value);
    }
}
