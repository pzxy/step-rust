// Stream 定义标准，主要是消费数据呢，继承了有两个重要的方法 
// 1. poll_next() 是相当于迭代器next方法。
// 2. size_hint() 是终止迭代。

// StreamExt 继承了 Stream trait，然后进行了扩展
// 增加了一些操作迭代器的方法，比如 next，map，fiflter这种。
use futures::{StreamExt};


fn main() {
    futures::executor::block_on(async {
        use futures::stream::{self};

        let mut stream = stream::iter(1..=3);
        // stream.size_hint();
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, None);
    });
}