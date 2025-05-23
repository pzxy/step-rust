// 声明宏
#[macro_export]
macro_rules! my_vec {
    // 没带任何参数的 my_vec，我们创建一个空的 vec
    () => {
        std::vec::Vec::new()
    };
    // 处理 my_vec![1, 2, 3, 4]
    // expr 表达式，$el将匹配到的表达式命名为$el，,*告诉编译器可以匹配多个以逗号分割的表达式。
    // $是声明一个参数
    ($($el:expr),*) => ({
        let mut v = std::vec::Vec::new();
        // 这里$()*表示展开括号中的内容，也就是展开push。
        $(v.push($el);)*
        v
    });
    // 处理 my_vec![0; 10]，匹配冒号分割的表达式。
    ($el:expr; $n:expr) => {
        std::vec::from_elem($el, $n)
    }
}

fn main() {
    let mut v = my_vec![];
    v.push(1);
    // 调用时可以使用 [], (), {}
    let _v = my_vec!(1, 2, 3, 4);
    let _v = my_vec![1, 2, 3, 4];
    let v = my_vec! {1, 2, 3, 4};
    println!("{:?}", v);
    println!("{:?}", v);
    let v = my_vec![1; 10];
    println!("{:?}", v);
}
