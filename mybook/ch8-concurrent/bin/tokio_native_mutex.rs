use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));
    for i in 1..10 {
        let db = db.clone();
        tokio::spawn(async move {
            // 这里会报错，因为MutexGuard这个锁没有实现 send tait
            // future可能会放到不同的线程中去执行，所以编译器提前报错了。不过如果这个锁生命周期内没有 .await就没问题。
            let mut db = db.lock().unwrap();
            db.insert(i.to_string(), Bytes::from("value"));
            todo(i).await;
        });
    }
    sleep(Duration::from_secs(2))
}

async fn todo(key: i32) {
    println!("{} insert value", key)
}
