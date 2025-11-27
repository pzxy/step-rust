fn main() {
    let s: u8 = 18; //变量名：类型
                    // 这种s..写法其实很有意思，会一直增加，直到u8类型溢出panic。
    for i in s.. {
        // 还有s..20 ,如果要包含20，s..=20
        println!("i: {}", i);
        if i == 20 {
            break;
        }
    }
    // ========== match 表达式 ==========
    // match 是表达式，可以返回值，必须穷尽所有可能的情况
    // 这里对 s 进行模式匹配，根据不同的值返回不同的结果
    let v = match s {
        // a if (a > 0) 是带守卫的模式匹配：
        //   - a 是模式绑定，匹配任意值并绑定到变量 a
        //   - if (a > 0) 是守卫条件，只有条件为真时才匹配
        //   - => a 是匹配成功时的返回值
        a if (a > 0) => a,
        // _ 是通配符模式，匹配所有其他情况（必须要有，因为 match 必须穷尽）
        _ => 0,
    };

    // ========== match vs if let 的区别 ==========
    // 1. match 必须处理所有可能的情况（exhaustive），if let 只需要处理一种情况
    // 2. match 是表达式，可以返回值；if let 主要用于条件执行
    // 3. 对于 Option<T> 类型，if let 更简洁：
    //    if let Some(x) = opt { ... }  vs  match opt { Some(x) => ..., None => {} }
    // 4. 对于需要处理多种模式的情况，必须使用 match
    //while 循环和 loop循环，基本上主要就是这些控制流了。以后补充
    println!("需要类型实现 Display trait: {}", v); // 需要类型实现 Display trait
    println!("需要类型实现 Debug trait: {:?}", v); // 需要类型实现 Debug trait
    println!("输出引用地址: {:p}", &v); // 输出地址
    print!("打印数，如果不足7位，则在左侧用零填充：{:07x} ", 18); // x十六进制，b二进制。
}
