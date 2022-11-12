use futures::channel::mpsc::unbounded;
use futures::SinkExt;
use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {}

// 需求1 简单发送，接受功能。
// 消费者生产者直接要有共享对列

#[test]
fn channel_should_work() {
    let (mut s, mut r) = unbounded();
    s.send("hello world!".to_string()).unwrap();
    let msg = r.recv().unwrap();
    assert_eq!(msg, "hello world!");
}

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

// 需求2
//由于需要的是 MPSC，所以，我们允许多个 sender 往 channel 里发送数据，用单元测试 2 来描述这个需求：

#[test]
fn multiple_senders_should_work() {
    let (mut s, mut r) = unbounded();
    let mut s1 = s.clone();
    let mut s2 = s.clone();
    let t = thread::spawn(move || {
        s.send(1).unwrap();
    });
    let t1 = thread::spawn(move || {
        s1.send(2).unwrap();
    });
    let t2 = thread::spawn(move || {
        s2.send(3).unwrap();
    });
    for handle in [t, t1, t2] {
        handle.join().unwrap();
    }

    let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
    // 在这个测试里，数据到达的顺序是不确定的，所以我们排个序再 assert
    result.sort();

    assert_eq!(result, [1, 2, 3]);
}

//需求 3
// 接下来考虑当队列空的时候，receiver 所在的线程会被阻塞这个需求。
// 那么，如何对这个需求进行测试呢？这并不简单，我们没有比较直观的方式来检测线程的状态。
// 不过，我们可以通过检测“线程是否退出”来间接判断线程是否被阻塞。
// 理由很简单，如果线程没有继续工作，又没有退出，那么一定被阻塞住了。
// 阻塞住之后，我们继续发送数据，消费者所在的线程会被唤醒，继续工作，所以最终队列长度应该为 0

#[test]
fn receiver_should_be_blocked_when_nothing_to_read() {
    let (mut s, r) = unbounded();
    let mut s1 = s.clone();
    thread::spawn(move || {
        for (idx, i) in r.into_iter().enumerate() {
            // 如果读到数据，确保它和发送的数据一致
            assert_eq!(idx, i);
        }
        // 读不到应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
        assert!(false);
    });

    thread::spawn(move || {
        for i in 0..100usize {
            s.send(i).unwrap();
        }
    });

    // 1ms 足够让生产者发完 100 个消息，消费者消费完 100 个消息并阻塞
    thread::sleep(Duration::from_millis(1));

    // 再次发送数据，唤醒消费者
    for i in 100..200usize {
        s1.send(i).unwrap();
    }

    // 留点时间让 receiver 处理
    thread::sleep(Duration::from_millis(1));

    // 如果 receiver 被正常唤醒处理，那么队列里的数据会都被读完
    assert_eq!(s1.total_queued_items(), 0);
}

//需求 4
// 顺着刚才的多个 sender 想，如果现在所有 Sender 都退出作用域，Receiver 继续接收，到没有数据可读了，该怎么处理？
// 是不是应该产生一个错误，让调用者知道，现在 channel 的另一侧已经没有生产者了，再读也读不出数据了？

#[test]
fn last_sender_drop_should_error_when_receive() {
    let (s, mut r) = unbounded();
    let s1 = s.clone();
    let senders = [s, s1];
    let total = senders.len();

    // sender 即用即抛
    for mut sender in senders {
        thread::spawn(move || {
            sender.send("hello").unwrap();
            // sender 在此被丢弃
        })
        .join()
        .unwrap();
    }

    // 虽然没有 sender 了，接收者依然可以接受已经在队列里的数据
    for _ in 0..total {
        r.recv().unwrap();
    }

    // 然而，读取更多数据时会出错
    assert!(r.recv().is_err());
}

//需求 5
// 既然没有 Sender 了要报错，那么如果没有 Receiver 了，Sender 发送时是不是也应该错误返回？
// 这个需求和上面类似，就不赘述了。看构造的单元测试 5：

#[test]
fn receiver_drop_should_error_when_send() {
    let (mut s1, mut s2) = {
        let (s, _) = unbounded();
        let s1 = s.clone();
        let s2 = s.clone();
        (s1, s2)
    };

    assert!(s1.send(1).is_err());
    assert!(s2.send(1).is_err());
}
