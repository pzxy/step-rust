// rust 中所有都是表达式
// 只是为了方便,称加了;的为语句,其他称为表达式。语句必须在表达式中。表达式可以嵌套。
// 因为如果是;结尾会返回一个值(),这是 Unit 类型,可以理解成什么都没有，但对于 rust 来说,这就是一个类型
// 之所以这么做,是因为 rust 要在编译期,确定所有函数入参出参的类型,包括闭包也是一种类型.
//下面代码可以正常运行的。
fn main() {
    // `{}`是块表达式
    {
        ()
    }
    {
        use std::vec::Vec;
        ()
    }
    ();
    &{};
}
