fn main() {
    println!("=== 参数传递的三种方式 ===\n");
    
    println!("【方式 1：Copy 类型 - 复制值本身】");
    let x = 42;
    println!("调用前: x = {}, 地址: {:p}", x, &x);
    
    pass_by_copy(x);  // 复制值
    println!("调用后: x = {}, 地址: {:p}", x, &x);  // x 不变，地址不变
    
    println!("\n【方式 2：传递引用 - 传递引用本身】");
    let y = 100;
    println!("调用前: y = {}, 地址: {:p}", y, &y);
    
    pass_by_reference(&y);  // 传递引用
    println!("调用后: y = {}, 地址: {:p}", y, &y);  // y 不变，地址不变
    
    println!("\n【方式 3：传递可变引用 - 可以修改原值】");
    let mut z = 200;
    println!("调用前: z = {}, 地址: {:p}", z, &z);
    
    pass_by_mutable_reference(&mut z);  // 传递可变引用
    println!("调用后: z = {}, 地址: {:p}", z, &z);  // z 被修改了，地址不变
    
    println!("\n【关键区别总结】");
    println!("1. Copy 类型传参：");
    println!("   - 复制值本身（不是引用）");
    println!("   - 函数内的参数有新的内存地址");
    println!("   - 修改函数内的参数不影响原值");
    
    println!("\n2. 引用传参（&）：");
    println!("   - 传递引用（指向原值）");
    println!("   - 函数内的参数指向原值的内存地址");
    println!("   - 不能通过引用修改原值（除非是 &mut）");
    
    println!("\n3. 可变引用传参（&mut）：");
    println!("   - 传递可变引用（指向原值）");
    println!("   - 函数内的参数指向原值的内存地址");
    println!("   - 可以通过引用修改原值");
    
    demonstrate_detailed();
}

// Copy 类型：接收值的副本
fn pass_by_copy(num: i32) {
    println!("  函数内: num = {}, 地址: {:p}", num, &num);
    // num 是 x 的副本，有新的内存地址
    // 修改 num 不会影响 x
}

// 接收不可变引用
fn pass_by_reference(num: &i32) {
    println!("  函数内: num = {}, 指向的地址: {:p}", num, num);
    println!("  函数内: num（引用本身）的地址: {:p}", &num);
    // num 是引用，指向原值的内存地址
    // 不能修改原值（因为是不可变引用）
}

// 接收可变引用
fn pass_by_mutable_reference(num: &mut i32) {
    println!("  函数内修改前: num = {}, 指向的地址: {:p}", num, num);
    *num += 1;  // 通过引用修改原值
    println!("  函数内修改后: num = {}", num);
}

fn demonstrate_detailed() {
    println!("\n【详细对比：Copy vs 引用】");
    
    let value = 42;
    
    // Copy：值被复制
    fn copy_example(v: i32) {
        let v2 = v;  // 再次复制
        println!("    copy_example: v={}, v2={}", v, v2);
    }
    copy_example(value);
    println!("   原值仍然可用: value={}", value);
    
    // 引用：传递引用
    fn ref_example(v: &i32) {
        let v2 = v;  // v2 也是引用，指向同一个值
        println!("    ref_example: v={}, v2={}", v, v2);
        println!("    v 和 v2 指向同一个地址: {:p} == {:p}", v, v2);
    }
    ref_example(&value);
    println!("   原值仍然可用: value={}", value);
    
    println!("\n【重要理解】");
    println!("❌ Copy 类型传参不是'复制引用'");
    println!("✅ Copy 类型传参是'复制值本身'");
    println!("✅ 如果要传递引用，必须显式使用 & 或 &mut");
}

