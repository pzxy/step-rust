use std::fmt::{Debug, Display, format, Formatter, write};
use std::intrinsics::offset;

// 类似接口
// 但不完全是，一般高级语言的接口是不自己实现方法的，
// 但是trait可以自己声明方法，自己实现方法。而且，trait中还能定义类型。
// 最重要的一点是，不能带着以前的东西来学习trait，要不rust为什么叫trait而不叫接口？在以前的语言中
// 都是哪个类，或者结构实现了哪个接口，这里其实实现类是主动的。是类要去实现这个接口。

// 但是在rust中，含义是：为某个类实现trait，说实现不太恰当，应该说把trait赋予给了这个类。在这里trait是主动的。
// 给了你了，你就是有了这个trait的特性了，这其中可能一些方法已经实现了，所以就没必要他，可能一些没有实现，你就需要实现他。
// 类型有了这个trait，就相当于有了一种特性，同时也获得了这个特性的中特别的方法了。
// trait作为入参，出参的时候，其实都是在过滤这种特性。
// 其实在别的语言中接口也可以这样理解，但没那个必要。
pub trait Summary {
    fn summarize(&self) -> String;
    // 默认实现，别的类型不用实现这个方法。
    fn summarize2(&self) {
        // 这里可以调用该trait别的方法。
        format!("{}", self.summarize());
    }
}


// 在类型上实现trait，可以不同类型实现同一个trait
pub struct Tweet {
    username: String,
    age: i32,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}

// 1. trait bound
pub fn notify<T: Summary>(iterm1: T, iterm2: T) {
    println!("notify:{}", iterm1.summarize())
}

// 2. 指定多个trait bound
pub fn notify2<T: Summary + Display>(iterm1: T) {
    println!("notify:{}", iterm1.summarize())
}

// 或者,这两种方式都是指定多个trait
pub fn notify21(iterm1: impl Summary + Display) {
    println!("notify:{}", iterm1.summarize())
}

// 3. 在方法签名后指定trait子句
pub fn notify3<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("notify:{}", a.summarize())
}

// 使用where子句定义trait，和c#很像
pub fn notify31<T, U>(a: T, b: U) -> String where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("notify:{}", a.summarize())
}

// 4. trait 作为返回值,要求返回类型必须实现了 Summary 这个trait
// 这里为什么不行，是因为入参，编译器肯定知道类型的，参数，编辑器也必须知道类型。
pub fn notify4(flag: bool) -> impl Summary {
    // 返回值需要实现Summary这个 trait，
    Tweet {
        username: "".to_string(),
        age: 0,
    }
    // 注意，返回的类型是必须是确定了的，也就是同一种类型。
    // 比如，这样写就不行了
    // if flag {
    //     Tweet{
    //         username: "".to_string(),
    //         age: 0
    //     }
    // }else {
    //     // 返回另外一个实现了 Summary 的类型。
    // }
}

// 5. 使用trait bound的例子
// std::cmd::PartialOrd 是用来比较大小的，所有可以比较大小的类型，都实现了这个trait
// Copy 是用来复制分配在栈上类型的数据的trait，比如基本类型，默认都实现了这个trait
// 因此，这个不能传字符串参数，因为字符串是分配在堆上，没有实现Copy trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 入参可以传字符串，将Copy改为Clone
fn largest2<T: PartialOrd + Clone>(list: &[T]) -> T {
    // 这里本质上是获取堆上的数据，移交所有权，但是不能这样做，所以必须使用clone
    // todo：这里不明白为什么不能这样做，以后学明白了再来看看，
    let mut largest = list[0].clone();
    // 这里list.iter()会发生数据的移动，如果是在堆中的数据，是没有Copy方法的。
    // 所以这里不能用&item,因为返回值是&T，&item实际上是T(解引用了),然后
    for item in list.iter() {
        // 所以这里的关键其实是 相同类型比较，和使用两次&，即是引用，也是解引用。
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

/******************************** 高级部分 ********************************/

// 6. 在 Trait 定义中使用关联类型来指定占位类型
// 关联类型(associated type) 是 trait 中的类型占位符,他可以用于 Trait 的方法签名中:
// - 可以定义出包含某些类型的 trait,而在实现前无需知道这些类型时什么.
pub trait Iterator {
    // 类型占位符
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// 7. 关联类型和泛型的区别
// 泛型:每次实现 Trait 时标注类型,可以为一个类型多次实现某个 Trait(不同的泛型参数)
struct Counter {}
// 使用不同的参数,可以多次实现某个 trait
// impl Iterator2<String> for Counter {
//     fn next(&mut self) -> Option<String> {
//         None;
//     }
// }
// impl Iterator2<u32> for Counter {
//     fn next(&mut self) -> Option<u32> {
//         None;
//     }
// }

// 关联类型: 无需标注类型,无法为单个类型多次实现某个 trait
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// 这里会报错,关联类型的 trait 一个类型,只能实现一次.
// impl Iterator for Counter{
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// 8. 默认泛型参数和运算符重载
// 可以在使用泛型参数时为泛型指定一个默认的具体类型
// 语法:<PlaceholderType=ConcreteType>
// 这种技术常用语运算符重载.
// 主要应用场景:
// - 扩展一个类型而不破坏现有代码
// - 允许在大部分用户都不需要的特定场景下进行自定义
// 虽然 Rust 不允许创建自己的运算符 以及 重载任意运算符.
// 但可以通过实现 std::ops 中列出的那些 trait 来重载一部分相应的运算符
// 例子
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn trait8() {
    assert_eq!(Point { x: 1, y: 9 } + Point { x: 2, y: 4 }, Point { x: 3, y: 3 });
}

// 另外一个实现一个毫米和米的相加,这次指定相加的类型
struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// 9. 完全限定语法(Fully Qualified Syntax),如何调用同名方法
// 完全限定语法: <Type as Trait>::function(receiver_if_method,next_arg,...)
// - 这种语法写起来比较麻烦,轻易不使用.
// - 允许忽略那些从其他上下文能推到出来的部分,
// - 当 Rust 无法区分你期望调用哪个具体实现的时候,才需要使用这种语法.
trait Pilot {
    fn fly(&self);
    fn run();
}

trait Wizard {
    fn fly(&self);
    fn run();
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("Human")
    }
    fn run() {
        println!("Human")
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("impl Pilot for Human fly")
    }

    fn run() {
        println!("impl Pilot for Human run")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("impl Wizard for Human fly")
    }
    fn run() {
        println!("impl Wizard for Human run")
    }
}

pub fn trait9() {
    let person = Human;
    //这里是调用了 human 本身的 fly 方法
    person.fly();
    // 这里会打印 Pilot 的 fly,
    // 注意:Pilot 这个 trait 会有多个实现,那他是怎么区分这个 fly 是 human 的实现呢?
    // 这里 fly 入参其实是有类型的,这个类型就是 Human
    Pilot::fly(&person);
    // 这里的 run 方法就没有参数,那么不能通过这种办法调用,
    // Pilot::run();
    // 这个时候就可以实现完全限定语法了
    <Human as Pilot>::run();
    <Human as Wizard>::run();
}

// 10. 使用 super-trait 来要求 trait 附带其他 trait 的功能. 相当于 trait 的继承概念
// 有时候需要在一个 trait 中使用其他 trait 的功能
// - 需要被依赖的 trait 也被发现
// - 那个被间接依赖的 trait 就是当前 trait 的 super-trait
use std::fmt;

// OutlinePrint 依赖于 fmt::Display 这个 trait
trait OutlinePrint: fmt::Display {
    // outline_print 中需要用到to_string()这个方法,
    // to_string() 这个方法在 Display 这个 trait 中.
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4))
    }
}

struct Trait10 {
    x: i32
}

impl OutlinePrint for Trait10 {}

// 因为OutlinePrint依赖于 fmt::Display 这个 trait,
// 因此,这里必须实现一下,否则 Trait10 就不算实现了 OutlinePrint 这个 trait
impl Display for Trait10 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

// 11. 使用 new type 模式在外部类型上实现外部 trait (类似适配器模式)
// 孤儿规则:以前学过,只有当 trait 或类型定义本地包时,才能为该类型实现这个 trait
// 可以通过 new type 模式来绕过这一规则
// - 利用 tuple struct (元组结构体)创建一个新的类型

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn trait11() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w={}", w);
}