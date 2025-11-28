use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}
// 只要实现了 From trait，就默认实现了 into trait,这两种trait都是为了将其他类型转换成自己。而不是类型互转。这点要注意。
// 这两个trait都是为了获取 Number类型，不是引用的情况下，都会转移所有权。
// form主要是当类型已知的情况下，而into主要是当类型未知的情况下，涉及到类型推断，特别是在泛型函数中。
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// impl Into<i32> for Number {
//     fn into(self) -> i32 {
//         self.value
//     }
// }

fn main() {
    // ========== 为什么需要 From 和 Into 两个 trait？ ==========
    // 虽然它们都会消耗所有权，但使用场景和灵活性不同：
    
    // 1. From: 从其他类型转换为自己（目标类型调用）
    //    语法：TargetType::from(source_value)
    //    优点：明确知道转换的目标类型，代码更清晰
    let num1 = Number::from(12);
    println!("From: {:?}", num1);
    
    // 2. Into: 将自己转换为其他类型（源值调用）
    //    语法：source_value.into() -> TargetType
    //    优点：需要类型推断，在泛型函数中更灵活
    let num2: Number = 32.into(); // 需要类型注解
    println!("Into: {:?}", num2);
    
    // 3. 为什么需要两个？主要区别在于调用者和使用场景：
    
    // From 的使用场景：明确知道目标类型
    let s1 = String::from("hello"); // 很清楚：创建 String
    println!("String::from: {}", s1);
    
    // Into 的使用场景：泛型函数中，让调用者决定目标类型
    fn convert_to_number<T: Into<Number>>(value: T) -> Number {
        value.into() // 可以接受任何能转换为 Number 的类型
    }
    
    let num3 = convert_to_number(42); // i32 -> Number
    let num4 = convert_to_number(100); // i32 -> Number
    println!("泛型函数 convert_to_number: {:?}, {:?}", num3, num4);
    
    // 4. From 和 Into 的关系：
    //    实现了 From<T> for U，就自动实现了 Into<U> for T
    //    这是通过 Rust 的 blanket implementation 实现的：
    //    impl<T, U> Into<U> for T where U: From<T> { ... }
    
    // 5. 实际使用建议：
    //    - 明确知道目标类型时，用 From（更清晰）
    //    - 在泛型代码中，用 Into（更灵活）
    //    - 两者功能等价，选择哪个主要看代码可读性
    
    println!("\n========== 基本用法 ==========\n");
    
    // from 将另外一种类型转换为自己，参考String::from();
    // 别的类型要实现From trait，只要实现了From trait就实现了Into trait
    let num = Number::from(12);
    println!("from {:?}", num);
    // 需要指定类型。
    let v: Number = 32.into();
    println!("into {:?}", v);
    // 这两种用哪种都行，怎么方便怎么来。
}
