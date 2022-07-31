// 闭包：闭包的本质是将栈上的数据放到了堆上，从个延长声明周期。

// 1. 声明一个闭包
fn closure1() {
    // 定义了一个匿名函数
    let closure_1 = |num| {
        println!("this is a closure");
    };
    // 执行匿名函数
    closure_1(3)
}

// 2. 闭包的类型推断
// 函数：fn add_v1 (x: u32) -> u32 {x + 1}
// 闭包：fn add_v2 |x: u32| -> u32 {x + 1};
// 闭包：fn add_v3 |x|      ->     {x + 1};
// 闭包：fn add_v4 |x|      ->      x + 1 ;

// 如果没有写类型，编译器会把闭包推出唯一的类型。
fn closure2() {
    // 定义了一个匿名函数
    let closure_1 = |num| {
        println!("this is a closure");
    };
    // 执行匿名函数
    closure_1(3);
    // 这里会报错，因为closure_1(3);
    // 已经判断出了这个函数的的入参数是int类型了，不能在使用其他类型了。
    // closure_1("3");
}

// 3. fn trait
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // 闭包
    calculation: T,
    // 要缓存的值
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // fn value(&mut self, arg: u32) -> u32 {
    //     match self.value {
    //         Some(v) => v,
    //         None => {
    //             let v = self.calculation(arg);
    //             self.value = Some(v);
    //             v
    //         }
    //     }
    // }
}
// vfn closure3(){
// let e = Cacher::new( | num | {
// println ! ("this is a closure");
// num
// });
// let s = e.value(2);
//
// }

/******************************** 高级部分 ********************************/

// 4. 函数指针
// 可以将函数传递给其他函数
// 函数在传递过程中会被强制转换成 fn 类型
// fn 类型就是 函数指针(function pointer)
fn add_one(x: i32) -> i32 {
    x + 1
}

// 第一个参数是一个函数指针,
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn closure4() {
    let answer = do_twice(add_one, 5);
    println!("{}", answer)
}

// 5. 函数指针与闭包的不同
// fn 是一个类型,不是一个 trait
// - 可以直接指定 fn 为参数类型,不用声明一个以 Fn trait 为约束的泛型参数
// 函数指针实现了全部 3 种闭包 trait(Fn,FnMut,FnOnce):
// - 总是可以把函数指针用作参数,传递给一个接受闭包的函数
// - 所以,常用搭配闭包的 trait 的泛型来编写函数:这样可以同时接受闭包和普通函数
// 某些情景,指向接受 fn 而不接受闭包:
// - 与外部不支持闭包的代码交互:c 函数
fn closure5() {
    let listOfNumbers = vec![1, 2, 3];
    let listOfStrings: Vec<String> = listOfNumbers
        .iter()
        // 这里也可以这样写,因为我们查看 map 函数,发现 F 其实是:FnMut(Self::Item) -> B
        // 是三种闭包 trait 中的一种.
        //.map(ToString::to_string)
        .map(|i| i.to_string())
        .collect();

    enum Status {
        Value(u32),
        Stop,
    }
    // 初始化 Value,其实用的是下面这个函数
    // 所以 map 可以将这个函数传进去.
    // let v = Status::Value(3);
    let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

//   fn map<B, F>(self, f: F) -> Map<Self, F>
//     where
//         Self: Sized,
//         F: FnMut(Self::Item) -> B,
//     {
//         Map::new(self, f)
//     }

// 6. 返回闭包
// 闭包使用 trait 进行表达,无法再函数中直接返回一个闭包,
// 可以将一个实现了该 trait 的具体类型作为返回值.

// 不能返回一个闭包,因为在编译时,不能判断返回值的大小
// 也是 r_type 中 5, 必须实现了 Sized Trait 的类型才能判断出大小
// fn return_closure()-> Fn(i32) -> i32 {
//     |x| x+1
// }

// 不能判断大小时,可以放在指针后面,比如 Box 中.
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// rust 闭包的本质,捕获上下文参数
// fn counter(i:i32) -> fn(i32) -> i32 {
//     fn inc(n:i32)->i32 {
//         // i 是counter函数的参数,所以不能被 inc 内部直接访问
//         n+i
//     }
//     inc
// }

// 这里返回值是实现了 FnMut 这个 trait 的一个 匿名函数
// 这种情况才是rust 中得闭包,rust 中闭包,也就是可以捕获上下文参数的函数,必须实现 Fn,FnMut FnOnce 三个 trait
fn counter(i: i32) -> impl FnMut(i32) -> i32 {
    // 将所有权,转移到闭包中
    for v in 1..10 {
        if v == 5 {
            v
        } else {
            v + 1
        }
    }
    move |n| n + i
}

fn counter222(i: i32) -> i32 {
    // 将所有权,转移到闭包中
    for v in 1..10 {
        if v == 5 {
        } else {
            v + 1
        }
    }
    5
}
