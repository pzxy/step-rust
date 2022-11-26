fn main() {
    use std::panic;
    // 设置回调
    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            println!(
                "===> panic occurred '{}' at {}",
                location.file(),
                location.line()
            );
        } else {
            println!("can't get location information...");
        }
    }));
    // 捕捉panic，和设置回调的顺序没有关系。都会先执行catch，然后执行回调函数。
    let _r = panic::catch_unwind(|| println!("---> oh shit"));
    panic!("panic o")
}
