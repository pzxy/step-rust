// 函数分为
// 常规函数 /关联函数 方法
#![allow(unused)]
#![allow(dead_code)]
fn main(){
    func1();
}
// 定义一个元组结构体
struct Func1(i32, i32);

impl Func1 {
    // 关联函数
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    // 方法,第一个参数时这个结构体本身
    fn math(&self) -> i32 {
        Self::sum(self.0, self.1)
    }
}

// 参数是函数指针类型
fn show(_c: fn(a: i32, b: i32) -> i32) {
    println!("show")
}

fn func1() {
    let a = Func1(1, 2);
    // 因为 sum 是关联类型,所以这里要使用 Func1::sum 这样来调用
    assert_eq!(3, Func1::sum(1, 2));
    // 方法的话,可以直接这样调用
    assert_eq!(3, a.math());

    // 这两个都是函数项类型了,函数项类型本身不占空间.
    // rust 中枚举也是函数项类型
    // 函数项项默认都实现了 Copy/Clone/Sync/Send/Fn/FnMut/FnOnce 类型
    let add = Func1::sum;
    let add_math = Func1::math;
    assert_eq!(3, add(1, 2));
    // 方法,第一个参数时这个结构体本身
    assert_eq!(3, add_math(&a));

    //函数项类型作为函数参数
    // 当被传入时,会隐式转换为函数指针类型.类似下面这个操作.
    // let c: fn(a:i32,b:i32)-> i32 = add;
    // show(c);
    show(add);
}
