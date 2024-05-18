// 所有权，借用，生命周期的关系。

fn main() {
    // 下面的这些对于基本数字类型除外（不包括String）。
    // 1. 所有权
    owned_demo();
    // 2. 通过借用规避所有权转移问题，此外还可以通过Rc智能指针来共享所有权。
    borrow_demo();
    // 3. 通过借用规避所有权转移问题，并可修改借用。
    mut_borrow_demo();
    // 4. 所有的引用都有生命周期，目的是为了防止悬垂指针的问题。
    // 所有的引用都需要标记生命周期。不过，所有rust有一套规则，符合规则的话，可以自动标记引用的生命周期。
    // 此外生命周期是编译的时候就已经确定了。
}

fn owned_demo() {
    println!("> owned demo");
    // 1. let可以将一个数据和一个变量绑定，绑定以后该变量就拥有了该数据的所有权。
    let a = String::from("hello");
    // 2. 赋值，传参数，打印(打印也是将变量传进去了)，都会转移所有权。导致之前的变量失效
    let a2 = a;
    // 这里无法打印，因为a的所有权被转移
    // println!("a:{}",a);
    println!("a2:{}", a2);
}

fn borrow_demo() {
    println!("> borrow demo");
    let a = String::from("hello");
    // 只读借用，任意使用，基本没什么规则。
    let a2 = &a;
    println!("a1:{}", a);
    println!("a2:{}", a2);
}

fn mut_borrow_demo() {
    // 可变借用规则：
    // 1. Rust 只允许同时存在一个可变引用或者多个不可变引用，不允许可变引用和不可变引用同时存在。
    // 2. 一个引用永远也不会比它的所有者存活得更久。
    // 总之：一句话就是不允许同一个对象的，两个可变变量的生命周期交叉存在。
    println!("> mut borrow demo");
    // 1. 可变借用使用有非常多的限制。这里代码和borrow_demo()一样，但是无法编译通过。
    // 因为a本身也是可变的，则就导致有有a和a2两个可变的，并且他们的生命周期交叉了。
    // let mut a = String::from("hello");
    // let a2 = &mut a;
    // println!("a1:{}", a);
    // println!("a2:{}", a2);

    // 2. 在可变借用的生命周期内，只能使用可变借用。
    let mut a = String::from("hello");
    let a2 = &mut a;
    a2.push_str(" world");
    println!("a2:{}", a2);// 这里要先使用可变的a2，再使用a；
    println!("a1:{}", a);
}
