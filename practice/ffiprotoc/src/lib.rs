// 生成的 bindings 代码根据 C/C++ 代码生成，里面有一些不符合 Rust 约定，我们不让编译期报警
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
mod bindings;

pub use bindings::*;

pub fn send(a: i32) {
    unsafe {
        let mut f = foo::new();
        f.send(a);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        send(2);
    }
}

pub struct Bar {}
#[allow(unused)]
impl Bar {
    fn add(a: i32) -> i32 {
        return a + 3;
    }
}
