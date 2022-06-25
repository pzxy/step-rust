use step_rust;
use crate::common::setup;

//导入本目录下的模块
mod common;

// 这里没有写cfg是因为，tests目录会被特殊对待
// 只有执行cargo  tests 时候这个目录下的文件才会被编译。
#[test]
fn demo() {
    // 只能调用lib.rs文件下的pub方法
    // 或者tests目录下的其他模块，通过mod引用进来，来调用。

    setup();
    assert_eq!(125, step_rust::add_two(123))
}
