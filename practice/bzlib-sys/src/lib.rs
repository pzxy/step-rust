// 生成的 bindings 代码根据 C/C++ 代码生成，里面有一些不符合 Rust 约定，我们不让编译期报警
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use crate::bindings::{DjiCameraManager_DeInit, DjiCameraManager_Init};

// 包含生成的绑定
mod bindings;

// 包装函数，使Rust代码可以安全地调用C函数
pub fn init_camera_manager() -> Result<(), i32> {
    unsafe {
        let result = DjiCameraManager_Init();
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

pub fn deinit_camera_manager() -> Result<(), i32> {
    unsafe {
        let result = DjiCameraManager_DeInit();
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

// 示例用法
pub fn example_usage() {
    match init_camera_manager() {
        Ok(_) => println!("相机管理器初始化成功"),
        Err(e) => println!("相机管理器初始化失败，错误码: {}", e),
    }
    
    // 使用相机管理器...
    
    match deinit_camera_manager() {
        Ok(_) => println!("相机管理器反初始化成功"),
        Err(e) => println!("相机管理器反初始化失败，错误码: {}", e),
    }
}