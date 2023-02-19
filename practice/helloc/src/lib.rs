use std::{ffi::*, ptr};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
// cbindgen --config cbindgen.toml --output my_header.h
// 使用 no_mangle 禁止函数名改编，这样其它语言可以通过 C ABI 调用这个函数
#[no_mangle]
pub extern "C" fn hello_world() -> *const c_char {
    // C String 以 "\\0" 结尾，你可以把 "\\0" 去掉看看会发生什么
    "hello world!\\0".as_ptr() as *const c_char
}

#[no_mangle]
// 在这个函数结束执行时，由于字符串 s 退出作用域，所以它的堆内存会被连带 drop 掉。因此，这个函数返回的是一个悬空的指针，在 C 那侧调用时就会崩溃
pub extern "C" fn hello_bad(name: *const c_char) -> *const c_char {
    let s = unsafe { CStr::from_ptr(name).to_str().unwrap() };

    format!("hello {}!\\0", s).as_ptr() as *const c_char
}


#[no_mangle]
pub extern "C" fn hello(name: *const c_char) -> *const c_char {
    if name.is_null() {
        return ptr::null();
    }

    if let Ok(s) = unsafe { CStr::from_ptr(name).to_str() } {
        let result = format!("hello {}!", s);
        // 可以使用 unwrap，因为 result 不包含 \\0
        let s = CString::new(result).unwrap();
        // 来让 Rust 侧放弃对内存的所有权。
        s.into_raw()
        // 相当于：
        // let p = s.as_ptr();
        // std::mem::forget(s);
        // p
    } else {
        ptr::null()
    }
}


#[no_mangle]
pub extern "C" fn free_str(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s) };
    }
}