fn main() {
    // 1. Box 使用场景:
    // - 在编译时,类型大小无法确定
    // - 有大量数据,需要移交所有权,要保证在操作时数据不会被复制.
    // - 使用某个值时,只关心实现了某个 trait,二不关心具体的类型时.
    let b = Box::new("value");
    let v = *b;
    println!("box:{}", b);
    println!("v:{}", v);
}
