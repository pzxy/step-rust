fn main() {
    use std::sync::Mutex;

    let data = Mutex::new(0);
    {
        // locked离开空间会自动释放锁，不用去主动解锁,因此一般放到{}中。
        let mut locked = data.lock().unwrap();
        *locked += 1;//这里只是可以修改这个值的例子，实际上使用锁的时候，这个值并不用关心。
        println!("locked data: {}", &locked);
    }
}
