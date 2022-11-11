use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// 1. 例子
#[test]
fn thread1() {
    // 开启一个线程
    let handle = thread::spawn(|| {
        for i in 1..20 {
            println!(". thread ***-{} ", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!(". thread ===-{}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 直到等待 handle 的线程结束.
    handle.join().unwrap()
}

// 2. move 移动别的线程中数据的所有权
#[test]
fn thread2() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector:{:?}", v);
    });
    // 这里会提示报错,因为 move 已经将所有权移动到了 handle 中了.
    // drop(v);
    handle.join().unwrap();
}

// 3. 使用通信的方式来并发 channel, mpsc::channel
// multi-producer, single-consumer,多个生产者一个消费者
// 如果发送端和接收端任意一端被丢弃了,那么 Channel 就关闭了
#[test]
fn thread3() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        // 发送数据
        // send 返回 Result<T,E>,如果接收端已经被丢弃了,就会返回一个错误.
        tx.send(val).unwrap();
        // 这里会报错,因为该线程已经没有 val 的所有权了
        //println!("val:{}", val);
    });
    // recv 阻塞当前线程等待消息,
    // try_recv 不会阻塞.有就返回数据,没有就返回错误.
    // 如果发送端关闭了,都会返回错误.
    let received = rx.recv().unwrap();
    println!("Got:{}", received);
    // 一般都这样接收,go 里面也可以这样
    for received in rx {
        println!("Got2:{}", received);
    }
}

// 通过克隆创建多个发送者
#[test]
fn thread33() {
    let (tx, rx) = mpsc::channel();
    // 克隆出来一个发送者
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("form"),
            String::from("zero"),
            String::from("to"),
            String::from("hero"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got:{}", received);
    }
}

// 4. 使用共享内存的方式来并发: Mutex<T>,Arc<T>
// Mutext::new来创建数据 Mutex<T> ,Mutex<T> 是一个智能指针
// 访问数据前,通过lock 来获取锁,会阻塞当前线程,lock 可能会失败,
// 返回的是 MutexGuard,这也是一个智能指针,实现了 Deref 和 Drop

// Arc<T> atomic Rc,就是原子操作的 Rc,这里为什么不用 Rc 呢?
// 是因为 Rc<T>不能用在多线程中,因为他没有 Send 这个 trait
// Arc 和 Rc 的 api 是相同的,在标准库中默认是 Rc 不是 Arc,因为 Arc 性能有点慢
// use std::sync::{mpsc, Mutex,Arc};
#[test]
fn thread_rc() {
    // // 这里不能用 Rc,因为 Rc 没有实现 Send trait
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    // // 这里 move 会报错,因为他在循环里面,当第一个循环执行的时候,所有权
    // // 已经放到了创建的第一个线程中了,后续循环的 move 无法再移动所有权了
    // // 所以我们想到用 Rc
    // // for _ in 0..10 {
    // //     let handle = thread::spawn(move || {
    // //         let mut num = counter.lock().unwrap();
    // //
    // //         *num += 1;
    // //     });
    // //     handles.push(handle)
    // // }
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle)
    // }
    //
    // for h in handles {
    //     h.join().unwrap();
    // }
}

// 将 Rc 换成 Arc 就可以了
#[test]
fn thread_arc() {
    // 这里不能用 Rc,因为 Rc 没有实现 Send trait
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // 这里 move 会报错,因为他在循环里面,当第一个循环执行的时候,所有权
    // 已经放到了创建的第一个线程中了,后续循环的 move 无法再移动所有权了
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle)
    // }
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle)
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("counter:{}", counter.lock().unwrap())
}

// 总结比较
// RefCell<T> & Rc<T> , Mutex<T> & Arc<T>
// 使用 RefCell<T> 来改变 Rc 中内容.
// 使用 Mutex<T> 来改变 Arc 中的内容.
// 注意:Mutex<T> 有死锁的风险,

// Send & Sync trait
// - Rust 语言的并发特性较少,目前讲的并发都来自标准库,而不是语言本身.
// - 无语局限标准库的并发,可以自己实现并发放.
// - 但是在 Rust 语言中有两个并发概念:
// std::marker::Sync & std::marker::Send 这两个 trait
// 这两个 trait 叫做标签 trait,因为他们没有实现任何方法.
// 1. Send :允许线程间转移所有权
// Rust 中几乎所有的类型都实现了 Send,但是 Rc<T>没有实现 Send,他只能用于单线程情景.
// 任何完全有 Send 类型组成的类型也被标记为 Send,除了原始指针外,几乎所有的基础类型都是 Send
// 2. Sync :允许类型从线程访问
// - 实现 Sync 的类型可以安全的被多个线程引用,也就是说:如果 T 是 Sync,那么&T 就是 Send.
// 那么 T 的引用就也可以被安全的送往另外一个线程.
// - 基础类型都是 Sync
// - 完全由 Sync 类型组成的类型也是 Sync 的.
// 但是,Rc<T>不是 Sync 的,RefCell<T>和 Cell<T>家族也不是 Sync 的,而 Mutex<T>是 Sync 的.

// Send 就是为了转移所有权,Sync 是可以传引用.
//注意: 自己手动实现 Send 和 Sync 是不安全的,因为要使用不安全代码.所以实现的是要要谨慎.
