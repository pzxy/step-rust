fn main() {
    // 1. Box 使用场景:
    // - 在编译时,类型大小无法确定
    // - 有大量数据,需要移交所有权,要保证在操作时数据不会被复制.
    // - 使用某个值时,只关心实现了某个 trait,二不关心具体的类型时.
    // 这里要说下智能指针的特点，就是将指定对象当成指针来用。
    let b = Box::new("box".to_string());
    let v = *b;
    // 这里会报错，因为上面一行发生了move，这是因为*b是String类型，它不是引用。
    // 如果*b是引用，比如是&str，则仅仅是发生借用。因此将上面的 "box".to_string() 改成 "box"，则不会有这个问题。
    println!("box:{:?}", b);
    println!("v:{}", v);
}
