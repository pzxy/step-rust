// 不安全 rust
// rust 中得第二语言,不能保证安全
// 存在的意义:rust 是保守的,编译的时候,为了安全会过于保守编译,
// - 宁可错杀一些合法的程序,也不放过一些可能非法的代码,尽管这些代码可能是安全的.
// 但是 rust 为了安全,还是会杀掉.
// - rust 需要能够进行底层的系统编程.

// 何时使用 unsafe 代码?
// 编译器无法保证内存安全,保证 unsafe 代码正确并不简单
// 有充足理由使用 unsafe 代码时,就可以这样做.
// 通过显示标记 unsafe,可以在出现问题是轻松定定位.

// 1. unsafe的超能力
// 使用 unsafe 关键字来切换到 unsafe Rust,开启一个块,里面放着 unsafe 代码.
// unsafe rust 里可执行四个动作(unsafe 超能力)
// - 解引用原始指针
// - 调用 unsafe 函数或方法
// - 访问或修改可变的静态变量
// - 实现 unsafe trait
// 注意:
// - unsafe 并没有关闭借用检查,或停用其他安全检查,依然要考虑借用关系
// - 任何内存安全相关的错误必须留在 unsafe 块里
// - 尽可能隔离 unsafe 代码,最好能封装在安全的抽象里面,提供安全的 Api,这样才能更好的保证安全.


use std::slice;

// 2. 解引用原始指针
// 原始指针,分为两种
// - 可变的:*mut T
// - 不可变的:*count T.意味着指针在解引用后不能直接对其进行赋值.
// 注意: 这里的*不是解引用符号,他是类型名的一部分.
//
// 与引用不同,原始指针:
// - 允许通过同时具有不可变和可变指针 或者 多个指针向同一位置的可变指针来忽略借用规则
// - 无法保证能指向合理的内存.
// - 允许为 null
// - 不实现任何自动清理
// 放弃保证安全,换区更好的性能,与其他语言或硬件接口的调用能力
//
// 使用原始指针可以:与 c 语言进行接口,构建借用检查器无法里的的安全抽象.
fn unsafe2() {
    let mut num = 5;
    // 将不可变引用转化为 不可变的原始指针 *const
    let r1 = &num as *const i32;
    // 将可变引用转化为 可变的原始指针 *mut ,这里*不表示解引用,这是一个整体
    // 因为 r2 是可变的,所以是可以通过 r2 来修改num 的值得.
    let r2 = &mut num as *mut i32;
    // 这里必须在 unsafe 不安全代码块中,使用原始指针,否则编译不过去.
    unsafe {
        println!("r1:{}", *r1);
        println!("r2:{}", *r2);
    }
    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        // 这里运行会报错,因为address地址是随便写的.
        println!("r:{}", *r)
    }
}

// 3. 调用 unsafe 函数或者方法
// 在定义方法和函数前面加上 unsafe关键字, 就是unsafe 方法或函数了
// - 调用前要手动满足一些条件(看文档),,因为rust是无法对这些条件进行验证.
// - unsafe 方法调用的时候要在 unsafe 块中进行调用
unsafe fn dangerous() {}

fn unsafe3() {
    // 这里直接调用会报错的,
    // dangerous()
    unsafe { dangerous() }
}

// 4. 创建 unsafe  安全抽象
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    // todo 这里会退出???
    assert!(mid < len);
    // 这样写会报错,不能将这个切片进行两次可变借用
    // 从规则上来说是不允许的,但是从代码的实际情况来看是没问题的.
    // (&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (
            // 从指针位置开始,获取指定长度的切片内容
            slice::from_raw_parts_mut(ptr, mid),
            // add 偏移指针
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn unsafe4() {
    let mut v = vec![1, 2, 3, 4];
    let r = &mut v[..];

    // 这里 一定不会导致程序崩溃,因为split_at_mut做了检查了,不符合条件的话,就会退出
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4]);

    // 下面这样直接使用 unsafe 是有问题,
    // 这里没做检查,并不能保证拥有 10000 个长度的切片是有效的.可能导致程序崩溃.
    // let address = 0x012345usize;
    // let r = address as *mut i32;
    // let slice: &[i32] = unsafe{
    //   slice::from_raw_parts_mut(r,10000)
    // };
}

// 5. 使用 extern 函数调用外部代码,类似 go 中 import "C"
// extern 关键字:简化创建和使用外部函数接口 FFI 的过程.
// 外部函数接口(FFI,Foreign Function Interface):它允许一种编程语言定义函数,
// 并让其他变成语言能调用这些函数
extern "C" {
    // 这里的 C 是指明的应用二进制接口 ABI(Application Binary Interface),它是定义函数在汇编层面的调用方式的
    // "C" ABI是常见的 ABI,它遵循 C 语言的 ABI
    // 任何在 extern 中定义的语言都是不安全的
    // 因为 rust 不会对他进行检查
    fn abs(input: i32) -> i32;
}

fn unsafe5() {
    unsafe {
        println!("{}", abs(-3));
    }
}

// 6. 从其他语言调用 rust 函数
// 可以说使用 extern 创建接口,其他语言铜鼓哦他们可以调用 Rust 的函数
// fn 前添加 extern 关键字,并指定 ABI
// 还需添加#[no_mangle]注解,避免 Rust 在编译时改变它的名称.

#[no_mangle]
// 这种类型调用时,不需要使用 unsafe
pub extern "C" fn call_from_c() {
    println!("call_from_c 在编译和链接后可以被 C 语言访问")
}

// 7. 访问或修改一个可变静态变量
// Rust 支持全局变量,但因为所有权机制可能产生某些问题,例如数据竞争
// 在 rust 里,全局变量叫做静态(static)变量

// 静态变量与常量类似
// 命名规范: acreaming_snake_case, 大写,下划线
// 必须标注类型
// 静态变量只能存储 static 声明周期的引用,无需显示标注.因为能够推断出来

// 常量与不可变静态变量的区别
// 静态变量:有固定的内存你地址,使用他的值总会访问同样的数据
// 常量:允许使用他们的时候对数据进行赋值.
// 静态变量:可以是可变的,访问和修改静态变量是不安全的
static HELLO_WORLD: &str = "hello,world";
// 声明一个可变的静态变量
static mut COUNTER: u32 = 0;

// 做一层包装
fn add_to_count(inc: u32) {
    // 这里要放到 unsafe 不安全代码块中,
    // 因为修改和访问可变静态变量是不安全的操作
    unsafe {
        COUNTER += inc;
    }
}

fn unsafe7() {
    println!("{}", HELLO_WORLD);
    add_to_count(3);
    // 我们访问这个可变的静态变量也要放到 unsafe 代码块中.
    // 因为可能有多线程的情况,出现数据竞争,尽可能用前面学过的只能指针来做.
    unsafe {
        println!(" COUNTER:{}", COUNTER);
    }
}

// 8. 实现不安全的 (unsafe) trait
// 当某个 trait 中至少一个方法拥有编译器无法校验的不安全因素是,就称这个 trait 是不安全的.
// 声明 unsafe trait:定义前加 unsafe 关键字
// - 该 trait 只能在 unsafe 代码块中实现
unsafe trait Foo {}

// 不安全的 trait 要在 unsafe 的 代码块中进行实现
unsafe impl Foo for i32 {}
