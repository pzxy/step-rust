use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}
// 只要实现了 From trait，就默认实现了 into trait,这两种trait都是为了将其他类型转换成自己。而不是类型互转。这点要注意。
// 这两个trait都是为了获取 Number类型，不是引用的情况下，都会转移所有权。
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
    // from 将另外一种类型转换为自己，参考String::from();
    // 别的类型要实现From trait，只要实现了From trait就实现了Into trait
    let num = Number::from(12);
    println!("from {:?}", num);
    // 需要指定类型。
    let v: Number = 32.into();
    println!("into {:?}", v);
    // 这两种用哪种都行，怎么方便怎么来。
}
