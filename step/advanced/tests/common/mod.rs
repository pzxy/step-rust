// 这里虽然也是一个文件，但是不是一个crate，只有tests目录下的一级文件才是crate
// 可以创建一个方法给其他测试crate使用

pub fn _setup() {
    println!("setup")
}
