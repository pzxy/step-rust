

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