/// trait 是一种特性，有别的语言的接口功能，但要换种方式来理解。

// rust中的trait要当作一种属性来理解，可以将这种属性赋予不用的struct。
// 这样可以更好的理解impl {trait} for {struct}中的for这个关键字
fn main() {

}

trait Foo {
    fn foo(&self);
}
// 1. 下面这两种写法是一样的。
#[allow(unused)]
fn my_function1<T>(f: T)
    where
        T: Foo
{
    f.foo();
}

#[allow(unused)]
fn my_function2(f: impl Foo) {
    f.foo();
}

// 2. 使用trait来约束泛型，java中可以使用extends关键值来做到同样的效果。c++可以用特化来对泛型做特别的处理。
struct MyStruct<T>
    where
        T: Foo
{
    foo: T
}