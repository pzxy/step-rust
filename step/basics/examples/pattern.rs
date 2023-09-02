// 模式是 Rust 中特殊的语法,用于匹配复杂和简单的结构
// 将模式与匹配表达式和其他构造结合使用,可以更好的控制程序的控制流
// 模式由一下元素(的一些组合)组成:
// - 字面值
// - 解构的数组 enum struct tuple
// - 变量
// - 通配符
// - 占位符
// 想要使用模式,需要将其与某个值进行比较
// - 如果模式匹配,就可以在代码中使用这个值得相应部分

// 1. match 的 arm,必须匹配所有可能性
// 2. if let 可以匹配 match 的一种可能,不会检查全部的可能性.
// 3. while let ,只要模式继续满足条件,就会循环
#![allow(unused)]
#![allow(dead_code)]

mod marco_todo;
mod r_type;
mod r_func;

fn main(){
    _pattern3();
}
fn _pattern3() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top)
    }
    // 3
    // 2
    // 1
}

// 4. for 循环,for 循环中,模式就是for字 后面的值
fn _pattern4() {
    let v = vec![1, 2, 3];
    // (k,v) 就是模式,这是一个元组
    for (k, v) in v.iter().enumerate() {
        println!("{} is at index {}", v, k);
    }
}

// 5. let 也是模式, let Pattern = Expression;
fn _pattern5() {
    let _a = 5;
    let (_x, _y, _z) = (1, 2, 4);
    // 个数必须对应上,否则会报错
    // let (x, y) = (1, 2, 4);
}

// 6. 模式作为函数的参数
fn _foo(_x: i32) {}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x:{},y:{}", x, y)
}

fn _pattern6() {
    let s = (2, 3);
    print_coordinates(&s);
}

// 7. 模式的两种形式: 可辩驳的(refutable)(可失败的),无可辩驳的(irrefutable)(不可失败的).
// 或者说是一种情况和多种情况的区别.
// 能匹配任何值得模式是无可辩驳的:
// 比如 let x=5;这就是无可辩驳的,只有这一种确定的值.
// 对于某些可能的值,无法进行匹配的模式:可辩驳的
// 例如: if let Some(x) = a_value
// 函数参数 let 语句 ,for 循环只接受无可辩驳的模式
// if let 和 while let 接受可辩驳和无可辩驳的模式.
// 其实这些都不用记,用的多了就记住了.
fn _pattern7() {
    let a: Option<i32> = Some(5);
    // 这里会报错,因为 let 只接受不可辩驳的模式,Some 是可辩驳的,也就是说有多个值,
    //let Some(x) = a;
    if let Some(x) = a {
        println!("x:{}", x)
    }
}

// 8. 模式的匹配
struct Point {
    x: i32,
    y: i32,
}

fn _pattern8() {
    let a = 1;
    match a {
        // | 匹配多种
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        // 匹配 4 到 8 的值
        // 这种写法还可以匹配字符
        // 'a' ..='c' => println!("a b c d"),
        4..=8 => println!("4 5 6 7 8"),
        _ => println!("anything"),
    }
}

// 9. 解构 分解值
fn _pattern9() {
    let p = Point { x: 1, y: 2 };
    // 解构 结构体,这里相当于这样写
    // let Point { x: x, y: y } = p;
    let Point { x, y } = p;
    assert_eq!(0, 1);
    assert_eq!(0, 2);
    // match 匹配结构体,这里挺好的,因为可以
    match p {
        Point { x, y: 0 } => println!("x:{},y:{}", x, y),
        Point { x: 0, y } => println!("x:{},y:{}", x, y),
        Point { x, y } => println!("x:{},y:{}", x, y),
    }
    // 解构元组 ,只要对应好就能都取出来, 非常好用
    let ((_feet, _inches), Point { x: _x, y: _y }) = ((3, 10), Point { x: 3, y: -10 });
}

// 10 match 匹配 枚举enum
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn _pattern10() {
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("quit")
        }
        Message::Move { x, y } => {
            println!("move {} {}", x, y)
        }
        // 这里特别像 java 中得 lambda 表达式,text 作为匿名函数的入参
        Message::Write(text) => {
            println!("text {}",text);
        }
        // 一个一个对应取出来
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("{},{},{}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(r, g, b)) => {
            println!("{},{},{}", r, g, b);
        } // 或者使用 _ =>(),
    }
}

// 11 使用 _ 忽略值
fn _pattern11() {
    // 忽略参数,这里其实第一个入参传的值会被忽略
    // fn f1(_:i32,y:i32)
    let s = Some(5);
    match s {
        // 这里其实只要是一个 Some 就可以了,具体是什么值就忽略了他
        Some(_) => {}
        _ => {}
    }
    // 忽略值的某一部分,比如匹配的时候
    let number = (1, 2, 3, 4, 5);
    match number {
        // 用不到的就忽略
        (one, _, three, _, five) => {
            println!("{},{},{}", one, three, five)
        }
    }
    // 忽略未使用的变量
    let _x = 5;
    // 奇技淫巧
    let s2 = Some(String::from("bear"));
    // 这里Some(_s),或者别的字母,是不行的,后面的打印会报错,
    // 因为会移交所有权,导致最后的打印没有所有权
    // 这里使用了 let,会获取所有权的,使用_可以不获取.
    if let Some(_) = s2 {
        println!(" hero");
    }
    println!("{:?}", s2)
}

// 12 使用 .. 忽略剩余部分

fn _pattern12() {
    let s = Point { x: 0, y: 3 };
    match s {
        // 这里使用 .. 来省略后面的变量,如果后面有 5 个,就会省略 5 个
        Point { x, .. } => println!("x:{}", x),
    }
    let num = (1, 2, 4, 4, 1, 19);
    match num {
        // 这里省略中间的所有,
        // 但是不能发现歧义,比如   (..,two,..) 这样
        (one, .., last) => {
            println!("one:{},last:{}", one, last)
        }
    }
}

// 13, 使用 match 守卫,来提供额外的条件.
// match 守卫就是 match arm 模式后额外的 if 判断条件,想要匹配条件也必须满足
fn _pattern13() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) | Some(40) => println!("Got 50"),
        Some(n) if n == y => println!("match ,n={:?}", n),
        _ => println!("ignore"),
    }
    println!("{:?},{}", x, y);
    let flag = false;
    match 10 {
        // 匹配数字
        4 | 5 | 6 if flag == true => println!("yes"),
        _ => println!("no"),
    }
}

// 14 @ 创建一个变量,匹配后绑定匹配值
fn _pattern14() {
    let msg = Point { x: 5, y: 19 };
    match msg {
        // 匹配指定值
        Point { x: 1, y: 2 } => {
            // 这种匹配指定值的,是没有办法将变量取出来的
            // println!("x:{},y:{}", x, y)
            println!("1, 2")
        }
        // 限定范围,注意@使用,
        // 因为使用了范围,所以要获取值需要创建一个新的变量
        Point {
            x: tmp @ 1..=10,
            y: _,
        } => {
            println!("tmp:{}", tmp)
        }
        // 匹配任意值,
        // 也可以写 Point { x, y, } =>{println!("x:{},y:{}", x, x)}
        Point { x: a, y: b } => {
            println!("x:{},y:{}", a, b)
        }
    }
}

// 2023/8/31 总结：
// 模式：字面量，解构的数据枚举等、变量、通配符、占位符等。都是模式，还有就是参数也是模式，只要有匹配的地方就是模式。
// 模式的匹配， let，if let，match这些都是匹配模式用的。
// 不可辩驳的，就是一定可以匹配成功，我们使用let a = 32;的时候这是一定成功的，因为无论a后面是任何值，都一定可以成功的。所以let模式是
// 使用if let Some(x) 匹配的时候可能失败，因为可能是None，所以他是可辩驳的。

// .. 忽略剩余部分
// _ 默认匹配
// @ 匹配成功后，将匹配到的值赋予一个变量。
