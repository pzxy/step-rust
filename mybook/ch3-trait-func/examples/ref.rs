fn main() {
    let mut value = 42;

    // 使用 `ref` 创建一个引用绑定
    match value {
        ref mut v => {//可以去掉 mut，去掉mut的话，这个v就是不可变的。
            println!("ret mut reference: {}", v);
            *v += 1; // 这里 `v` 是 `value` 的引用
        }
    }
    println!("value:{}", value);

    // ref和&在语义上是一样的。ref只能用在匹配中，但是&更为灵活。
    // 上面的和下面这样是同样的效果
    match &mut value {
        v => {
            println!("& mut reference: {}", v);
            *v += 1;
        }
    }
    println!("value:{}", value);
}