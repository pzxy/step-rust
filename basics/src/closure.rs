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
    where T: Fn(u32) -> u32,
{
    // 闭包
    calculation: T,
    // 要缓存的值
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = self.calculation(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
// vfn closure3(){
// let e = Cacher::new( | num | {
// println ! ("this is a closure");
// num
// });
// let s = e.value(2);
//
// }
