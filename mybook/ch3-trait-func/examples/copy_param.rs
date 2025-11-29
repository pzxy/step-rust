fn main() {
    println!("=== Copy 类型作为参数传递 ===\n");
    
    println!("【Copy 类型：复制值本身，不是引用】");
    let x = 42;  // i32 实现了 Copy
    println!("调用函数前: x = {}", x);
    
    take_copy_value(x);  // 这里 x 被复制，不是传递引用
    println!("调用函数后: x = {}", x);  // x 仍然可用，因为被复制了
    
    println!("\n【对比：非 Copy 类型：移动所有权】");
    let s = String::from("hello");
    println!("调用函数前: s = {}", s);
    
    take_string(s);  // 这里 s 被移动
    // println!("调用函数后: s = {}", s);  // ❌ 编译错误！s 已被移动
    
    println!("\n【如果要传递引用，需要显式使用 &】");
    let y = 100;
    println!("调用函数前: y = {}", y);
    
    take_reference(&y);  // 显式传递引用
    println!("调用函数后: y = {}", y);  // y 仍然可用
    
    let mut z = 200;
    println!("调用函数前: z = {}", z);
    
    take_mutable_reference(&mut z);  // 显式传递可变引用
    println!("调用函数后: z = {}", z);  // z 被修改了
    
    println!("\n【内存地址验证】");
    demonstrate_memory_address();
}

// Copy 类型：接收的是值的副本
fn take_copy_value(num: i32) {
    println!("  函数内收到: num = {}", num);
    // num 是 x 的副本，修改 num 不影响 x
    // 注意：这里 num 是值，不是引用
}

// 非 Copy 类型：接收的是所有权（移动）
fn take_string(s: String) {
    println!("  函数内收到: s = {}", s);
    // s 拥有所有权，函数结束后 s 被销毁
}

// 接收引用（不可变）
fn take_reference(num: &i32) {
    println!("  函数内收到引用: num = {}", num);
    // num 是引用，指向原来的值
}

// 接收可变引用
fn take_mutable_reference(num: &mut i32) {
    println!("  函数内收到可变引用: num = {}", num);
    *num += 1;  // 通过引用修改原值
    println!("  函数内修改后: num = {}", num);
}

fn demonstrate_memory_address() {
    println!("\n【内存地址对比】");
    
    let x = 42;
    println!("x 的地址: {:p}", &x);
    
    // Copy 类型：传递的是值的副本
    fn show_address_copy(num: i32) {
        println!("  函数内 num 的地址: {:p}", &num);  // 不同的地址！
    }
    show_address_copy(x);
    
    // 引用：传递的是引用本身（引用也有地址）
    fn show_address_ref(num: &i32) {
        println!("  函数内 num（引用）的地址: {:p}", num);  // 指向原值的地址
        println!("  函数内 num（引用本身）的地址: {:p}", &num);  // 引用本身的地址
    }
    show_address_ref(&x);
    
    println!("\n结论：");
    println!("✅ Copy 类型传参：复制值，新值有新的内存地址");
    println!("✅ 引用传参：传递引用，引用指向原值的内存地址");
}

