use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}
// trait FnBox {
//     fn call_box(self: Box<Self>);
// }
//
// // 非常妙, 将所有实现了 FnOnce trait的类型,都实现 FnBox trait
// impl<F: FnOnce()> FnBox for F {
//     fn call_box(self: Box<Self>) {
//         (*self)()
//     }
// }

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
// type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// 创建一个线程池
    /// 如果创建的数量为 0 程序会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            // new thread
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 这样写有问题,join方法需要获取线程的所有权
            // 但是这里的 worker 是一个可变引用,没办法获取
            // 修改方法是将worker 中线程改为 Option,这样可以通过 take 提出来,
            // worker.thread.join().unwrap();
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job;executing.", id);
                    job()
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
