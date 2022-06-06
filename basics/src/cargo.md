
# 主要内容
- 通过 `release profile` 来自定义构建
- 在 `https://crates.io` 上发布库
- 通过`workspace`组织大工程
- 从 http://crate.io/来安装库.
- 使用自定义命令扩展 `cargo`

## 1. release profile
- 是预定义的
- 可自定义:可使用不同的配置,对代码编译拥有更多的控制.
- 每个 profile 的配置都独立于其他的 profile 存在.
- Cargo 主要的两个 profile:
- dev profile:适用于开发,cargo build
- release profile:适用于发布,cargo build -release

## 2. 自定义 profile
- 针对每个 profile ,Cargo 都提供了默认的配置
- 如果想自定义 xxx profile 的配置,可以在 Cargo.toml 里添加
`[profile.xxx]`区域,比如`[profile.dev]`,在里面覆盖默认的配置子集
- 例子:
```toml
[package]
name = "basics"
version = "0.1.0"
edition = "2021"

[dependencies]

[profile.dev]
# opt-level 是优化变异时的优化等级.
# 这里会覆盖默认的配置,如果我们使用 cargo build 就会执行这个配置
opt-level = 1

[profile.release]
# 使用 cargo build --release 会执行这个配置.
ope-level = 3
```
- 对于每个配置的默认值和完整选项,请参见:[https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/).

## 3. 工作空间
工作空间用于管理多个项目, 而且所有crate 是使用的相同 crate 包的版本都是兼容的.rust 会自己处理,具体看 cargo.lock 这个文件就知道了.
```toml
# Cargo.toml 文件
# 可以看到 basics crate 依赖 ,rand 0.3.23,但是 rand 0.3.23 又依赖于 "rand 0.4.6"
# 所以归根道理,两个 crate 是依赖的相同的版本.
[[package]]
name = "basics"
version = "0.1.0"
dependencies = [
 "rand 0.3.23",
]

[[package]]
name = "rand"
version = "0.3.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "64ac302d8f83c0c1974bf758f6b041c6c8ada916fbb44a609158ca8b064cc76c"
dependencies = [
 "libc",
 "rand 0.4.6",
]
```
- 创建一个 package ,然后创建`Cargo.toml`文件.
- 在package中,创建一个 crate basics,`cargo new basics --lib`.
- package-Cargo.toml文件中配置,这样配置以后,全局就只有一个 target 了,一个 Cargo.lock 了.
  ```toml
  # 这个文件是没有 package 和 dependencies 的
  [workspace]
  members = [
     "basics"
  ]
  ```
- 在 crate step 中引用别的 carte basics
  ```toml
  [dependencies]
   basics = { path = "../basics" }
  ```
- 当执行`cargo test`时,会跑所有的 crate 的测试用例,可以使用`cargo test -p crate名字`来运行指定 crate 的用例.
