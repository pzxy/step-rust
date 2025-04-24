use std::env;

fn main() {
    // 重新运行构建脚本，如果 wrapper.hpp 发生改变
    println!("cargo:rerun-if-changed=wrapper.hpp");
    
    // 打印当前工作目录，帮助调试
    println!("cargo:warning=当前工作目录: {:?}", env::current_dir().unwrap());
    
    // 使用 bindgen 生成绑定
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp") // 使用 wrapper.hpp
        .clang_arg("-I.") // 添加当前目录到包含路径
        .clang_arg("-Ipsdk_lib/include") // 添加psdk_lib/include到包含路径
        .clang_arg("-Ipsdk_lib") // 添加psdk_lib到包含路径
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks)) // 添加回调
        .layout_tests(false) // 禁用布局测试
        .derive_debug(true) // 为所有类型派生Debug
        .derive_default(true) // 为所有类型派生Default
        .derive_copy(true) // 为所有类型派生Copy
        .derive_hash(true) // 为所有类型派生Hash
        .derive_partialeq(true) // 为所有类型派生PartialEq
        .derive_eq(true) // 为所有类型派生Eq
        .derive_partialord(true) // 为所有类型派生PartialOrd
        .derive_ord(true) // 为所有类型派生Ord
        .generate()
        .expect("Unable to generate bindings");
    
    // 将生成的绑定写入 src/bindings.rs
    // let out_path = PathBuf::from("src").join("bindings.rs");
    // println!("cargo:warning=绑定文件路径: {:?}", out_path);
    
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
    
    // 链接静态库
    // 指定静态库的搜索路径
    println!("cargo:rustc-link-search=native=psdk_lib/lib");
    
    // 链接静态库
    println!("cargo:rustc-link-lib=static=payloadsdk");
    
    // 如果静态库依赖其他系统库，也需要链接
    // 例如，如果依赖 pthread 库
    println!("cargo:rustc-link-lib=pthread");
    
    // 如果静态库是用 C++ 编译的，需要链接 C++ 标准库
    println!("cargo:rustc-link-lib=stdc++");
}
