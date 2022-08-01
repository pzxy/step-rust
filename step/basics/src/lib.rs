//! # 外层注释
//! 通常用于描述 crate 以及模块的特性.

extern crate core;

// cargo test 命令，执行所有测试
// 下面是打印信息
// 运行了一个测试
//running 1 test
// 测试函数的名字
// test tests::it_works ... ok
// 通过1个，失败0个，被忽视0个，性能测试0个，过滤的测试0个
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//      Running unittests src/main.rs (target/debug/deps/step_rust-591d145e20300c2a)
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
// 文档测试中的测试，rust可以编译出现在文档中的代码，来保证文档代码和实际代码一致。
//    Doc-tests step_rust
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
mod iterator;
mod marco;
mod pattern;
mod pointer;
mod r_func;
mod r_trait;
mod r_type;
mod r_unsafe;
mod thread;
mod r_log;

/// 这是文档注释
/// # Examples
/// ```
///   let arg = 5;
///   let answer = step_rust::add_one(arg);
///
///   assert_eq!(6,answer)
/// ```

pub fn add_two(x: i32) -> i32 {
    x + 2
}

// 这是单元测试
#[cfg(test)]
mod tests {
    use crate::iterator::{iterator31, iterator32, iterator4};
    use crate::pointer::{pointer2, pointer3, pointer4, pointer5, pointer7};
    use crate::r_log::{log1, log2};
    use crate::r_unsafe::unsafe4;
    use crate::thread::{thread1, thread2, thread3, thread33, thread_arc};

    // 加了 #[test]就是测试函数,也可以不加
    #[test]
    fn it_works() {
        let result = 2 + 2;
        // 断言 assert!(true);assert_eq!();assert_ne();
        // eq,nq失败的话，会打印值的信息。
        assert_eq!(result, 4);
        // assert!(false, "这是添加的自定义的错误信息，失败的话会打印出来");
    }

    // 测试发生 panic ，则失败
    #[test]
    // 加上这个参数，执行时会忽略这个测试，
    // 如果想执行这个测试，则 cargo test -- --ignored
    #[ignore]
    fn another() {
        panic!("Make this test fail")
    }

    // 测试发生panic ，则成功
    #[test]
    // 只有发生panic，这个测试才会通过。
    // 如果加了expected信息，那么发生expected信息必须包含panic的信息。
    #[should_panic(expected = "期待的错误信息")]
    fn another2() {
        panic!("期待的错误信息")
    }

    // 通过Result来判断是否成功
    #[test]
    // 返回 OK成功，返回Err失败。
    fn t_result() -> Result<(), String> {
        if 5 > 4 {
            Ok(())
        } else {
            Err(String::from("测试失败"))
        }
    }

    #[test]
    fn t_iterator() {
        iterator31();
        iterator32();
        iterator4();
    }

    #[test]
    fn t_pointer() {
        pointer2();
        pointer3();
        pointer4();
        pointer5();
    }

    #[test]
    fn t_pointer6() {
        // pointer6();
        pointer7()
    }

    #[test]
    fn t_thread() {
        thread1();
        thread2();
        thread3();
        thread33();
        thread_arc();
    }

    #[test]
    fn t_unsafe() {
        unsafe4()
    }


    #[test]
    fn t_log(){
        log1();
        log2();
    }
}

// 测试命令：
// 1. 并行，并发测试：测试默认是并行的，但是可以设置线程的数量，这个参数是传递给二进制文件的
// cargo test -- --test-threads=1
// 2. 显示测试打印内容：在成功的测试中也看到打印的内容
// cargo test -- --show-output
// 3. 单个测试：指定只运行单个测试
// cargo test 测试函数名
// 4. 多个测试：匹配多个测试运行,指定测试函数名的一部分，或者模块名
// cargo test 函数名一部分，或者模块名字
// 5. 忽略测试：当需要忽略耗时，或者没必要的测试时。在函数上加上#[ignore]
// 当我们需要执行忽略的测试时，运行以下命令
// cargo test -- --ignored
