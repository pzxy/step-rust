fn main() {
    let maybe_number: Option<i32> = Some(42);
    // if let 表示，如果maybe_number匹配出来的是Some(number)，则执行{}中的代码，否则执行else代码
    // 也可以将 let Some(number) = maybe_number 看成一个整体。
    if let Some(number) = maybe_number {
        println!("The number is: {}", number);
    } else {
        println!("No number found.");
    }
}