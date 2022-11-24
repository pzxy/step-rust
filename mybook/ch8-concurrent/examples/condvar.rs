use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        eprintln!("I'm a happy worker!");
        // 通知主线程
        cvar.notify_one();
    });
    // 等待工作线程的通知
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while *started == false {
        // 在 cvar.wait() 中传入这个 Mutex。这个接口需要一个 MutexGuard，
        // 以便于知道需要唤醒哪个 Mutex 下等待的线程
        started = cvar.wait(started).unwrap();
        println!("over")
    }
    eprintln!("Worker started!");
}
