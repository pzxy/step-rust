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
fn pattern3() {
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
fn pattern4() {
    let v = vec![1, 2, 3];
    // (k,v) 就是模式,这是一个元组
    for (k, v) in v.iter().enumerate() {
        println!("{} is at index {}", v, k);
    }
}

// 5. let 也是模式, let Pattern = Expression;
fn pattern5() {
    let a = 5;
    let (x, y, z) = (1, 2, 4);
    // 个数必须对应上,否则会报错
    // let (x, y) = (1, 2, 4);
}

// 6. 模式作为函数的参数
fn foo(x: i32) {}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x:{},y:{}", x, y)
}

fn pattern6() {
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
fn pattern7() {
    let a: Option<i32> = Some(5);
    // 这里会报错,因为 let 只接受不可辩驳的模式,Some 是可辩驳的,也就是说有多个值,
    //let Some(x) = a;
    if let Some(x) = a {
        println!("x:{}", x)
    }
}


