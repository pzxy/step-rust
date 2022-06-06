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

## 6. 使用 pub use 导出方便使用的公共 api.

简单来说,就是将调用路径很深的类型,重新使用 `pub use` 声明一下,然后直接通过 `crate::类型或者方法` 来调用.
例子:

```rust
// lib.rs 中
pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}

// main.rs 中
// 这里调用需要引用这么长的名字
use crate名字::kinds::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
}


```

使用`pub use`

```rust

// lib.rs 中
pub use self::kinds::PrimaryColor;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}

// main.rs 中
// 这里可以直接crate 名字来调用
use crate名字::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
}


```

另外,这样使用`pub use`的话,生成的文档也会不同.


## 7. 发布项目到 crates.io 上
需要注意的是,发布上去的版本是不能删除的,这保证了被人可以放心的使用你的 crate 而不用担心被删除.
而且 crate 的名字是唯一的.
1. 创建 [crates.io ](https://crates.io/)账号,通过 github 账号登录
2. 用户设置-> API Tokens ->New Token
3. 运行命令 `cargo login 获取的APIToken`,这个 token 一定要保密,泄露以后要在 [https://crates.io/](https://crates.io/) 重新生成,
API token 会存储在本地`~/.cargo/credentials`中.
4. Cargo.toml 配置
    ```toml
    [package]
    # 必填
    name = "唯一的crate名字"
    # 必填
    description = "crate 描述,会出现在 crate 的搜索里面."
    # 必填:指定 license,在 http://spdx.org/licenses/ 查找,多个用 OR
    license = "MIT" 
    version = "0.1.0"
    auther = "作者"
    edition = "2021"
    ```
   [更多元数据...](https://doc.rust-lang.org/cargo/reference/manifest.html)
5. 发布 `cargo publish` 命令,首次提交需要验证下邮箱,根据提示操作就好了. 