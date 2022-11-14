use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::Mutex;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));
    for i in 1..10 {
        let db = db.clone();
        tokio::spawn(async move {
            let mut db = db.lock().await;
            db.insert(i.to_string(), Bytes::from("value"));
            todo(i).await;
        });
    }
    sleep(Duration::from_secs(2))
}

async fn todo(key: i32) {
    println!("{} insert value", key)
}
