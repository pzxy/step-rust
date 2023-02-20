// 生成的 bindings 代码根据 C/C++ 代码生成，里面有一些不符合 Rust 约定，我们不让编译期报警
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

// #[link(name = "foo")]
// extern "C" {
//      fn send(a: i32);
//      fn recv(a: i32);
// }

// fn main() {
//     unsafe {
//         send(2);
//     }
// }

#[link(name = "hello-world")]
extern {
    fn greet();
}
 
fn main() {
    unsafe {
        greet();
    }
}