// use crate::common::setup;

use crate::common::{async_read, async_read_end, async_write};

//导入本目录下的模块
mod common;

// 这里没有写cfg是因为，tests目录会被特殊对待
// 只有执行cargo  test 时候这个目录下的文件才会被编译。
#[test]
fn demo() {
    // 只能调用lib.rs文件下的pub方法
    // 或者tests目录下的其他模块，通过mod引用进来，来调用。
    // setup();
    //    let _ = async_read();
    //   let _ = async_read_end();
    async_write()
}



