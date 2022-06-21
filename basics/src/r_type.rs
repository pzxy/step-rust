
// 类型
fn r_type() {
    // bool类型就是 bool
    // 整型 i32 u32 isize usize等
    // 浮点型 f32 f64
    // 字符类型比较特殊，是Unicode类型，支持emoji，比较特殊。范围 U+0000~U+D7FF , U+E000~U+10FFFF 。
    let a = '2';
    let b = '$';
    let c = '🤔';
}

// 1. r_trait 11

// 2. 使用类型别名创建类型同义词
// Rust 提供了类型别名的功能:
// - 为现有类型生产另外的名称(同义词)
// - 并不是一个独立的类型
// 使用 type 关键字
// 主要用用途:减少代码字符重复

// 原来的代码
// use std::io::Error;
// pub trait Write{
//  fn Write(&mut self,buf: &[u8])->Result<usize,Error>;
// }

// 使用别名后的代码
pub trait Write{
    // 这里 Result 之所以可以这样写,是因为在std::io中.有一个这样的别名
    // type Result<T> = Result<T,std::io::Error>
    // 或者, 我们在本文件中声明 type Result<T> = std::io::Result<T>,也可以
    // rust 中得别名和go 里面的很像,基本上和原类型一样.
 fn Write(&mut self,buf: &[u8])->std::io::Result<usize>;
}

// 3. Never 类型
// 有一个名为 ! 的特殊类型:
// - 它没有任何值,行话称为空类型(empty type)
// - 我们倾向于叫他 never 类型,因为他在不返回的函数中没充当返回类型.
// 不返回值的函数也被称为发散函数(diverging function)

// 直接使用这种类型时错误,因为我们没办法创建出 ! 类型来返回.
// fn bar() -> ! {}

// 比如 panic! 的返回类型就是 never,因为他不会返回类型.
// never 类型补充了没有返回值得代码的返回类型.
// 自己的理解是: rust 是函数式编程,函数式变成都是有返回值的,
// never 类型作为无函数返回的类型,那么统一了函数式变成.
fn type3(){
    let guess = "";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 这里 continue 类型就是!,也就是 never 类型,
            // never 类型无法产生一个可以返回的值,
            // 但是 rust 可以将 never 类型转换为任何类型
            // 这里就将 never 类型转换为了 num 的类型,也就是 u32
            Err(_) => continue,
        };
    }
}

// 4. 动态大小和 Sized Trait
// Rust 需要在编译时确定为一个特定类型的值分配多少空间.
// 动态大小的类型(Dynamically Sized Types,DST)(大小不定类型)的概念:
// - 编写代码时使用只有在运行时才能确定大小的值
// str 是动态大小的类型(注意不是&str):只有运行时才能确定字符串的长度
// - 下列代码无法正常工作,因为相同类型,但是长度不一样:
//  - let s1:str= "123";
//  - let s2:str= "123456789";
// - 使用 &str 来解决:
// 字符串切片 &str 中存的是字符串地址和字符串长度. 所以&str 类型大小一定是固定的
// 因此代码中用的都是 &str 类型
//  - str 的地址
//  - str 的长度


// 5. Rust 使用动态大小类型(大小不定类型)的通用方式
// 附带一些额外的元数据来存储动态信息的大小
// - 使用动态大小类型时总会把它的值放在某种指针后边 , 比如 4 种的 &str
// trait 也是动态大小类型
// 每个 trait 都是一个动态大小类型,可以通过名称对其进行引用/
// 为了将 trait 用作 trait 对象,必须将它放置在某种指针之后
// 例如 &dyn Trait , Box<dyn Trait> (Rc<dyn Trait>) 之后


// 6. Sized trait
// 为了处理动态大小类型(大小不定类型),
// Rust 提供了一个 Sized trait 来确定一个类型的大小在编译时是否已知.
// - 编译时可计算出大小的类型会自动实现这一 trait
// - Rust 还会为每一个泛型函数隐式的添加 Sized 约束.

// fn generic<T>(t:T){
//
// }
// 隐式的转换为这样,这样在编译时才能过滤实现了 Sized trait 的类型,
// 只有实现了 Sized trait,才能确定类型的大小
// fn generic<T: Sized>(t:T){
//
// }

// 默认情况下,泛型函数只能同于编译时已经知道大小的类型,可以通过特殊语法接触这一限制.
// 这里的 ? 表示了一种不确定性,但是 ?Sized trait只能用在 Sized 上,不能用在其他 trait
// 这里的t: &T,用的&T,因为 T 可能是也可能不是 Sized
// fn generic<T: ?Sized>(t: &T){
//
// }

// 总结:
// 别名,
// Never 类型,
// rust中相同类型只能存在一份,可以有不同类型,相同大小.
// 每种大小不定类型默认有的 Sized trait ,
// 使用 大小不定类型 时要放在 指针后面.
// 可以用 大小不定类型 限定泛型