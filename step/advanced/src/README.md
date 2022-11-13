## 集成测试

- 创建集成测试目录：tests目录，和src是同级目录。
- tests目录下的每个测试文件都是单独的一个`crate`，并且需要被测试库导入。
- 无需像单元测试那样标注`#[cfg(test)]`,`tests`目录被特殊对待，只有执行`cargo test`，
  才会编译 `tests` 目录下的文件。

## 运行执行的集成测试：
- 运行`cargo test`,如果单元测试都通过，就会跑集成测试，否则是不会跑集成测试的
- 运行一个特定的集成测试: `cargo test 函数名`
- 运行某个测试文件内的所有测试: `cargo test --test 文件名`(不包括.rs)

## 针对 binary crate的测试集成

- 如果项目是binary crate，只包含src/main.rs 没有src/lib.rs,
  不能在test目录下创建集成测试，无法把main.rs的函数导入作用域，因为只有lib.rs
  才能让别人用，正常开发中，大部分都是lib.rs，main.rs中只有一些少量的胶水代码。
- 只有 library crate 才能报漏函数给其他crate用
- binary crate 意味着独立运行。