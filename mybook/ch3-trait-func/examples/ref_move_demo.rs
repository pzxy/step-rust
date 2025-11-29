fn main() {
    println!("=== 关键点：match value 是否会移动？===");
    
    println!("\n【情况 1】i32 (实现了 Copy trait)");
    let mut value = 42;
    
    // i32 实现了 Copy，所以 match value 是复制，不是移动
    match value {
        ref mut v => {
            println!("ref mut reference: {}", v);
            *v += 1; // 修改的是 value 的引用
        }
    }
    // value 仍然可以使用，因为 i32 实现了 Copy（复制语义）
    println!("value 仍然可用: {}", value);
    
    println!("\n【情况 2】String (没有实现 Copy trait)");
    let mut value = String::from("hello");
    
    // String 没有实现 Copy
    // 使用 ref mut 时，在模式中创建引用，这样不会移动 value
    match value {
        ref mut v => {
            println!("ref mut reference: {}", v);
            v.push_str(" world");
        }
    }
    // 这里 value 仍然可以使用！
    // 因为 ref mut 在模式中创建引用，match 匹配的是引用，不会移动 value
    println!("value 仍然可用: {}", value);
    
    println!("\n【情况 3】如果不用 ref，String 会被移动");
    let value2 = String::from("test");
    
    // 如果直接 match value2（不用 ref），value2 会被移动
    match value2 {
        s => {
            println!("匹配到: {}", s);
            // s 拥有 value2 的所有权
        }
    }
    // println!("{}", value2); // 编译错误！value2 已被移动
    
    println!("\n【情况 4】对比：match &mut value（推荐方式）");
    let mut value3 = String::from("hello");
    
    // 使用 &mut value3，不会移动 value3
    match &mut value3 {
        v => {
            println!("&mut reference: {}", v);
            v.push_str(" rust");
        }
    }
    println!("value3: {}", value3); // 可以正常使用
    
    println!("\n=== 总结 ===");
    println!("1. match value: 对于 Copy 类型（如 i32），会复制；对于非 Copy 类型（如 String），会移动");
    println!("2. match value {{ ref mut v => ... }}: ref mut 在模式中创建引用，不会移动 value");
    println!("3. match &mut value: 先创建引用再匹配，不会移动 value（推荐方式）");
}
