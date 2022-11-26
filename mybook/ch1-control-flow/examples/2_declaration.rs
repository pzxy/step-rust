// ：在rust中表示一种约束，他在两个地方用
// 1. 声明时候，无论是方法参数，还是声明变量。
// 2. 将一个trait 继承给另外一个trait时候。
// , 在rust中表示一种相同属性的分割，或者一种顺序。
fn main() {
    // 声明，rust中的声明全部都是 变量名：类型,无论是作为函数参数，还是函数中的临时变量。
    // 这里的let表示一种绑定，将Dog这个值的所有权绑定到变量s上。
    let a: Dog = Dog {
        Name: "diandian".to_owned(),
        Age: 12,
    };
    foo(a)
}

// 声明 trait
trait Animal {
    fn run(&self) {
        println!("run")
    }
}

// 实现Debug tarit， 
#[derive(Debug)]
// 声明结构体
struct Dog {
    // , 在rust中表示一种相同属性的分割，或者一种顺序。
    Name: String,
    Age: usize,
}

// 实现Animal tarit
impl Animal for Dog {}

fn foo(a: impl Animal) {
    println!("{:?}", a.run())
}
