fn main() {
    println!("=== 关键概念：Copy vs Move ===\n");
    
    println!("【Copy 类型：i32】");
    let x = 42;  // x 拥有值 42
    let y = x;   // 这里不是"转移所有权"，而是"复制"！
    println!("x = {}, y = {}", x, y);  // x 仍然可以使用！
    
    // 对于 Copy 类型，没有"所有权转移"的概念
    // 每次赋值都是复制，原来的变量仍然可以使用
    
    println!("\n【非 Copy 类型：String】");
    let s1 = String::from("hello");  // s1 拥有这个 String
    let s2 = s1;  // 这里才是"转移所有权"（移动）
    // println!("{}", s1);  // 编译错误！s1 已经被移动了
    println!("s2 = {}", s2);  // 只有 s2 可以使用
    
    println!("\n【Copy 类型在函数调用中】");
    let num = 100;
    take_ownership_i32(num);  // num 被复制，不是移动
    println!("num 仍然可用: {}", num);  // num 仍然可以使用！
    
    println!("\n【非 Copy 类型在函数调用中】");
    let text = String::from("world");
    take_ownership_string(text);  // text 被移动
    // println!("{}", text);  // 编译错误！text 已经被移动了
    
    println!("\n【Copy 类型在 match 中】");
    let value = 42;
    match value {
        v => println!("匹配到: {}", v),
    }
    println!("value 仍然可用: {}", value);  // value 被复制，不是移动
    
    println!("\n【非 Copy 类型在 match 中】");
    let text = String::from("hello");
    match text {
        s => println!("匹配到: {}", s),
    }
    // println!("{}", text);  // 编译错误！text 已经被移动了
    
    println!("\n【使用 ref 避免移动】");
    let text2 = String::from("hello");
    match text2 {
        ref s => println!("匹配到: {}", s),  // ref 创建引用，不移动
    }
    println!("text2 仍然可用: {}", text2);  // text2 没有被移动
}

fn take_ownership_i32(x: i32) {
    println!("函数中收到: {}", x);
    // x 离开作用域时，因为是 Copy 类型，只是复制，不影响原来的值
}

fn take_ownership_string(s: String) {
    println!("函数中收到: {}", s);
    // s 离开作用域时，String 被销毁
    // 原来的变量不能再使用
}

