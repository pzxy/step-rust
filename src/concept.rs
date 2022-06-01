// 借用：将引用作为参数的，就是借用，如果要修改借用的数据，需要声明mut。

// Rust 中没有null
fn r_null() {
    // Some None这是 Prelude 预导入的Option<T>枚举类型的值，所以可以直接用。
    let some_number = Some(5);
    let some_string = Some("asd");
    // 上面两个都可以推断处类型，
    // 这里直接使用None是无法推断出类型的
    //如果使用枚举要定义类型，应该这样写，但是Option<i32> 不是i32类型。
    let absent_number: Option<i32> = None;
    // 这里不能相加，检查会提示的。
    let s2 = absent_number + some_number;
}

// 基本类型，分配在栈上的数据，都实现了Copy这个trait
// 分配在heap上的数据，实现的是Clone


// 生命周期作用：避免悬垂引用(danging reference)
// 1. 楔子
fn r_life() {
    {
        let r;
        {
            let x = 5;
            // 这里会报错，因为将借用的x赋值给r后，再往下走一行，x就被释放了。
            r = &x;
        }
        // x已经被释放了，但是这里还使用x的值
        println!("r:{}", r);
    }
}

// 2. 这种写法会报错，因为这两个是借用，他们的声明周期是依据原来类型的声明周期的。
// 但是作为方法独立的存在，他是没有上下文的，入参是两个，返回是一个，所以返回值并不知道返回的是哪个类型的声明周期。
// 这里和代码逻辑没有关系。
// 可以反推出rust在编译阶段就知道了所有变量的生命周期。
fn longest(x: &str, y: &str) -> &str {
    x
}