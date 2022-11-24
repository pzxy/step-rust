# 常见crates
1. clap：命令行解析
2. serde：序列化,json,yaml等
3. rand：随机数
4. smoltcp：tcp实现
5. trust-dns：dns实现
6. url：url解析和验证
7. crc：crc校验     
8. chrono：操作时间
9. rayon: 并行迭代，并且不引入静态条件。适合cpu密集型任务
10. crossbeam：一个封装的channel。有select！宏。
11. lazy_static: lazy_static!宏在执行时加载，而不是在编译时。
12. anyhow： 用的很多的错误处理库
13. dotenv： 读取环境变量的库
14. sqlx：连接数据库
15. futures：官方的future执行器。
16. tokio：三方future执行器。适合io密集型任务
17. log4rs：日志库，配置方式类似 slf4j
18. env_log: 日志库，环境变量中配置
19. parking_lot::Mutex： 性能更高的锁
20. tokio-rs/prost: 根据protobuf生成代码的库
21. axum: web应用框架
22. s2n-quic：quic协议google设计，http2的升级版本http3
23. diosxus：开发wasm前端
24. arc_swap: 文件数据热加载
25. tonic：grpc框架，为protobuf生成rust代码。
26. rusty_v8: 基于v8，deno团队用rust对原v8的封装，deno是nodejs升级版本。
27. oso：权限处理。
