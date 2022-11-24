# 导读

## Sink 与 Stream
这两个都和程序外界操作有关，比如文件系统，操作系统等。
- stream 语义为接受的数据。
- sink 语义为下沉，表示可以不马上发送，总的来说理解成发送就行了。

## Read、Write、AsyncRead、AsyncWrite

AsyncRead依靠原生Read，AsyncWriter也是。
就类似go中的reader和writer一样。

## codec 中的 Encoder、Decoder 

Decoder 与 Encoder用于编码解码，可以自定义实现。
例如tokio中LinesCodec就是编码解码字符串，遇到换行符则为一个frame。
我们也可以自定实现编码解码二进制类型。

## Sink、Stream 和 AsyncRead、AsyncWrite 还有 Decoder 、Encode的关系。

codec可以将实现了AsyncRead/AsyncWrite的结构转换为Stream/Sink，得到的Stream和Sink可以以帧Frame为读写单位。