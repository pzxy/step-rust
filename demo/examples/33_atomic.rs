use std::thread::{self, JoinHandle};
use std::sync::atomic::{Ordering, AtomicBool};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}
// 在producer中，可以保证指令执行最后的执行结果是没有问题的。
// 但是不能保证B一定比A先执行，因为该线程中后面没用到DATA的值。所以A在B前后执行都没有问题。
fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;                                 // A
        }
        READY.store(true, Ordering::Relaxed);           // B
    })
}
// 这里C执行成功后，执行D，读出来不一定是100，因为producer不能保证A一定比B后执行。
fn consumer() -> JoinHandle<()> {
    thread::spawn(move || unsafe {
        while !READY.load(Ordering::Relaxed) {}         // C

        // assert_eq!(100, unsafe { DATA });               // D
        if DATA != 100 {
            panic!("asd")
        }
    })
}

fn main() {
    loop {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}