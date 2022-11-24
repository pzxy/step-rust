// 这种套路都是一样的，先定一个AsyncRead trait，然后扩展他。
// 如果别的类型要用这个trait，就用这个类型(这里是Cursor)去实现，AsyncRead
// 然后Cursor就有了AsyncRead和AsyncReadExt的能力了。

// Read 和  AsyncRead 的区别就是AsyncRead是异步用的。
// 需要注意的是，他们都是trait，是需要别人实现的。
// 还有标准包也有一个Read，futures中也有Read


// AsyncRead和AsyncWrite差不多。
fn main() {
    futures::executor::block_on(async {
        use futures::io::{AsyncReadExt, Cursor};

        let reader1 = Cursor::new([1, 2, 3, 4]);
        let reader2 = Cursor::new([5, 6, 7, 8]);
        // chain 是AsyncReadExt中的一个方法。
        let mut reader = reader1.chain(reader2);
        let mut buffer = Vec::new();
    
        // read the value into a Vec.
        reader.read_to_end(&mut buffer).await?;
        assert_eq!(buffer, [1, 2, 3, 4, 5, 6, 7, 8]);
        Ok::<(), Box<dyn std::error::Error>>(())
    })
    .unwrap();
}
