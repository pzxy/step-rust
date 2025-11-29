fn main() {
    println!("=== Copy vs Clone ===\n");
    
    println!("【Copy：隐式复制（自动发生）】");
    let x = 42;  // i32 实现了 Copy
    let y = x;   // 这里自动发生 Copy，不需要显式调用
    println!("x = {}, y = {}", x, y);
    
    // Copy 发生在以下情况（都是自动的）：
    // 1. 赋值
    let a = 10;
    let b = a;  // 自动 Copy
    
    // 2. 函数传参
    take_i32(a);  // 自动 Copy，a 仍然可用
    println!("a 仍然可用: {}", a);
    
    // 3. match 表达式
    match a {
        v => println!("匹配到: {}", v),  // 自动 Copy
    }
    println!("a 仍然可用: {}", a);
    
    // 4. 返回值
    let result = return_i32();
    println!("result: {}", result);
    
    println!("\n【Clone：显式复制（需要主动调用）】");
    let s1 = String::from("hello");  // String 实现了 Clone，但没有实现 Copy
    let s2 = s1.clone();  // 必须显式调用 .clone()
    println!("s1 = {}, s2 = {}", s1, s2);  // 两个都可以使用
    
    // 如果不调用 clone()，会发生移动
    let s3 = String::from("world");
    let s4 = s3;  // 移动，不是复制
    // println!("{}", s3);  // ❌ 编译错误！s3 已被移动
    println!("s4 = {}", s4);
    
    println!("\n【Copy 和 Clone 的关系】");
    println!("1. Copy 是标记 trait（marker trait），没有方法");
    println!("2. Copy 是隐式的，自动发生");
    println!("3. Copy 是浅拷贝（bitwise copy）");
    println!("4. Clone 有 clone() 方法，需要显式调用");
    println!("5. Clone 可以是深拷贝");
    println!("6. 所有 Copy 类型都自动实现了 Clone");
    
    // 验证：Copy 类型也可以调用 clone()（但通常不需要）
    let num = 100;
    let num2 = num.clone();  // 可以调用，但通常直接赋值就够了
    println!("num = {}, num2 = {}", num, num2);
    
    println!("\n【实际使用建议】");
    println!("✅ Copy 类型：直接赋值，让编译器自动 Copy");
    println!("✅ Clone 类型：需要复制时显式调用 .clone()");
    println!("✅ 或者使用引用（&）来避免移动和复制");
    
    // 使用引用避免复制
    let text = String::from("hello world");
    print_string(&text);  // 传递引用，不移动也不复制
    println!("text 仍然可用: {}", text);
}

fn take_i32(x: i32) {
    println!("函数收到: {}", x);
    // x 离开作用域时，因为是 Copy，不影响原来的值
}

fn return_i32() -> i32 {
    42  // 返回值时也会 Copy
}

fn print_string(s: &String) {
    println!("打印字符串: {}", s);
    // 使用引用，不获取所有权
}

