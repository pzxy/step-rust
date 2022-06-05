
# 主要内容
- 通过 `release profile` 来自定义构建
- 在 `https://crates.io` 上发布库
- 通过`workspace`组织大工程
- 从 http://crate.io/来安装库.
- 使用自定义命令扩展 `cargo`

## release profile
- 是预定义的
- 可自定义:可使用不同的配置,对代码编译拥有更多的控制.
- 每个 profile 的配置都独立于其他的 profile 存在.
- Cargo 主要的两个 profile:
- dev profile:适用于开发,cargo build
- release profile:适用于发布,cargo build -release

## 自定义 profile
- 针对每个 profile ,Cargo 都提供了默认的配置
- 如果想自定义 xxx profile 的配置,可以在 Cargo.toml 里添加
`[profile.xxx]`区域,比如`[profile.dev]`,在里面覆盖默认的配置子集
- 例子:
```toml
[package]
name = "step_rust"
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
