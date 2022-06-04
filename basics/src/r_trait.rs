use std::fmt::{Debug, Display, format};

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
    fn summarize2(&self) -> String {
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
            largest = item;
        }
    }
    largest
}



