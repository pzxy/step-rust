fn main() {
    println!("=== Rust 中的 Copy 类型 ===\n");
    
    println!("【基本类型（都实现了 Copy）】");
    
    // 整数类型
    let a: i8 = 1;
    let b = a;  // 复制
    println!("i8: a={}, b={}", a, b);  // a 仍然可用
    
    let a: i16 = 2;
    let b = a;
    println!("i16: a={}, b={}", a, b);
    
    let a: i32 = 3;
    let b = a;
    println!("i32: a={}, b={}", a, b);
    
    let a: i64 = 4;
    let b = a;
    println!("i64: a={}, b={}", a, b);
    
    let a: i128 = 5;
    let b = a;
    println!("i128: a={}, b={}", a, b);
    
    let a: isize = 6;
    let b = a;
    println!("isize: a={}, b={}", a, b);
    
    // 无符号整数类型
    let a: u8 = 7;
    let b = a;
    println!("u8: a={}, b={}", a, b);
    
    let a: u16 = 8;
    let b = a;
    println!("u16: a={}, b={}", a, b);
    
    let a: u32 = 9;
    let b = a;
    println!("u32: a={}, b={}", a, b);
    
    let a: u64 = 10;
    let b = a;
    println!("u64: a={}, b={}", a, b);
    
    let a: u128 = 11;
    let b = a;
    println!("u128: a={}, b={}", a, b);
    
    let a: usize = 12;
    let b = a;
    println!("usize: a={}, b={}", a, b);
    
    // 浮点数类型
    let a: f32 = 3.14;
    let b = a;
    println!("f32: a={}, b={}", a, b);
    
    let a: f64 = 2.71;
    let b = a;
    println!("f64: a={}, b={}", a, b);
    
    // 布尔类型
    let a: bool = true;
    let b = a;
    println!("bool: a={}, b={}", a, b);
    
    // 字符类型
    let a: char = '中';
    let b = a;
    println!("char: a={}, b={}", a, b);
    
    // 数组（如果元素类型是 Copy）
    let a: [i32; 3] = [1, 2, 3];
    let b = a;  // 数组复制
    println!("array: a={:?}, b={:?}", a, b);  // a 仍然可用
    
    // 元组（如果所有元素都是 Copy）
    let a: (i32, bool, char) = (42, true, 'x');
    let b = a;  // 元组复制
    println!("tuple: a={:?}, b={:?}", a, b);  // a 仍然可用
    
    println!("\n【非 Copy 类型（会被转移所有权）】");
    
    // String - 不是基本类型，没有实现 Copy
    let s1 = String::from("hello");
    let s2 = s1;  // 移动
    // println!("{}", s1);  // ❌ 编译错误！
    println!("String: s2={}", s2);
    
    // Vec - 不是基本类型，没有实现 Copy
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // 移动
    // println!("{:?}", v1);  // ❌ 编译错误！
    println!("Vec: v2={:?}", v2);
    
    // 包含非 Copy 类型的元组
    let t1 = (String::from("hello"), 42);
    let t2 = t1;  // 移动（因为包含 String）
    // println!("{:?}", t1);  // ❌ 编译错误！
    println!("tuple with String: t2={:?}", t2);
    
    println!("\n【总结】");
    println!("✅ 基本类型（整数、浮点数、bool、char）都实现了 Copy");
    println!("✅ 数组和元组（如果元素都是 Copy）也实现了 Copy");
    println!("❌ String、Vec、Box 等没有实现 Copy，会被转移所有权");
    println!("❌ 包含非 Copy 类型的复合类型也不会实现 Copy");
}

