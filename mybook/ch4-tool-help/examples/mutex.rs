fn main() {
    use std::sync::Mutex;

    let foo = "".to_string();
    let data = Mutex::new(foo);

    let locked = data.lock().unwrap();
    println!("locked data: {}", &locked[..]);
    // locked离开空间会自动释放锁，不用去主动解锁。
}
