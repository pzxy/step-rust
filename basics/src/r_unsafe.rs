// 不安全 rust
// rust 中得第二语言,不能保证安全
// 存在的意义:rust 是保守的,编译的时候,为了安全会过于保守编译,
// - 宁可错杀一些合法的程序,也不放过一些可能非法的代码,尽管这些代码可能是安全的.
// 但是 rust 为了安全,还是会杀掉.
// - rust 需要能够进行底层的系统编程.

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


use std::ptr::addr_of_mut;
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
    unsafe {
        dangerous()
    }
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
