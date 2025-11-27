fn main() {
    let maybe_number: Option<i32> = Some(42);
    // if let 表示，如果maybe_number匹配出来的是Some(number)，则执行{}中的代码，否则执行else代码
    // 这种if let match语法糖，是只匹配一种的情况，其他情况都是else。而单纯的match是必须穷尽所有可能的情况。
    if let Some(number) = maybe_number {
        println!("The number is: {}", number);
    } else {
        println!("No number found.");
    }
}