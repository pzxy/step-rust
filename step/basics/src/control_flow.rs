fn r_for_while_loop() {
    // loop 相当于go里面的for {}
    // while 就是传统的那种
    let a = [10, 20, 30, 40];
    for e in a.iter() {
        println!("value:{}", e);
    }
    // 包头不包尾，类似python shell等语法
    for number in (1..4).rev() {
        // 输出 3 2 1
        println!("{}", number)
    }
}

// if let
fn r_if_let() {
    let v = Some(8u32);
    match v {
        Some(3) => println!("three"),
        // 表示其他类型的匹配，因为枚举类型必须穷举，相当于 default
        _ => println!("other"),
    }
    // if let,只匹配一种情况，不用穷举。
    if let Some(3) = v {
        println!("three");
    } else {
        println!("other");
    }
}
