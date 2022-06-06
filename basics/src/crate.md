# 发布 crate 到 crates.io 

## 1. crates.io
- 可以通过发布包来共享底代码
- crate 的注册表在`http://crates.io/`
  - 他会分发已经住的包的源代码
  - 托管代码
  
## 2. [文档注释](./lib.rs)
- 生成 html 文档
- 显示公共 API 的文档注释:如何使用 API
- 使用`///`
- 支持 Markdown
- 放置在被说明条目之前

## 3. 生成文档
1. 使用命令`cargo doc` ,它会运行 rustdoc 工具,rust 安装时自带,会将`src/lib.rs` 中的`pub`函数
生成文档,将生成的 html 文档放在 `target/doc` 目录下.
2. 或者使用`cargo doc --open`,直接通过浏览器打开.

## 4. 文档注释作为测试
实例代码块附加值:运行`cargo test`:会将把文档注释中得实例代码作为测试来运行,
从而确保实例代码是可运行的.

## 5. [为包含注释的项添加文档注释](./lib.rs)
- 符号:`//!`
- 这类注释通常用描述 crate 和模块:
- crate root 按惯例 src/lib.rs,
