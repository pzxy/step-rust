fn main() {
    // 声明，rust中的声明全部都是 变量名：类型
    // 无论是作为函数参数，还是函数中的临时变量。
    // 这里的let表示一种绑定，将0这个值的所有权绑定到变量s上。
    let s: u8 = 0; //变量名：类型
                   // 这种s..写法其实很有意思，会一直增加，直到u8类型溢出panic。
    for i in s.. {
        println!("Hello, world! {}", i);
    }
    // 还有 if let 这种写法
    let v = match s {
        // if 这种
        s if (s > 0) => 1,
        _ => s,
    };
    //while 循环和 loop循环，基本上主要就是这些控制流了。以后补充
}
