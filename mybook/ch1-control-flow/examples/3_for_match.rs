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
    // 还有 if let 这种写法，if let Some(v) = a {},这是一个匹配语句，匹配将a的内容和Some(v)做匹配。
    let v = match s {
        // if 这种
        s if (s > 0) => s,
        _ => 0,
    };
    //while 循环和 loop循环，基本上主要就是这些控制流了。以后补充
    println!("需要类型实现 Display trait: {}", v); // 需要类型实现 Display trait
    println!("需要类型实现 Debug trait: {:?}", v); // 需要类型实现 Debug trait
    println!("输出引用地址: {:p}", &v); // 输出地址
    print!("打印数，如果不足7位，则在左侧用零填充：{:07x} ", 18); // x十六进制，b二进制。
}
