// 使用尽可能多的方法来通过编译
fn error_easy1() {
    // 这种string 类型,没有实现 Copy strait
    let x = String::from("hello, world");
    // let x = &String::from("hello, world");
    let y = x;
    // 这里会报错,x 已经 move 了, 上面需要 x.clone(),或者 直接使用 "hello world".
    println!("{},{}", x, y);
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    unimplemented!()
}

fn never_return_fn2() -> ! {
    panic!("6789")
}

fn never_return_fn3() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.as_bytes(); //不会 moved
    let _s2 = s.into_bytes(); //会 moved,是不是只要 into 的,都会 moved
    s
}

fn error_easy2() {
    let s = String::from("hello, ");

    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world")
}

fn error_easy3() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}

// 借用,不能借用两次可变借用,如果借用同一个多次可变,要不不使用借用,要不只使用最后一次借用
