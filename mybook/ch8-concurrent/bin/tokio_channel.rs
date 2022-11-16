use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Sender<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Sender<Option<Bytes>>,
    },
}

struct DB {
    db: HashMap<String, Bytes>,
}

impl DB {
    fn new() -> Self {
        DB { db: HashMap::new() }
    }
    fn set(&mut self, key: String, value: Bytes) -> Option<Bytes> {
        self.db.insert(key, value)
    }
    fn get(&self, key: &str) -> Option<&Bytes> {
        self.db.get(key)
    }
}
// 使用 mpsc 添加或者查询k-v数据。
// 通过 oneshot 返回添加或者查询结果。
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    // 添加数据1
    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Command::Set {
            key: String::from("k1"),
            value: Bytes::from("不要以千金之躯,当众矢之鹄,舍盘石之安,就虎尾之危。"),
            resp: resp_tx,
        })
        .await
        .unwrap();
        let res = resp_rx.await.unwrap();
        println!(
            "send1:Set Result={:?}",
            String::from_utf8(res.unwrap().to_vec())
        )
    });

    // 添加数据2
    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: String::from("k2"),
            value: Bytes::from("清风不识字,何故乱翻书"),
            resp: resp_tx,
        })
        .await
        .unwrap();
        let res = resp_rx.await.unwrap();
        println!(
            "send2:Set Result={:?}",
            String::from_utf8(res.unwrap().to_vec())
        )
    });

    // 接受数据
    tokio::spawn(async move {
        let mut db = DB::new();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => match db.get(&key) {
                    Some(v) => {
                        resp.send(Some(v.clone())).unwrap();
                    }
                    None => {
                        resp.send(None).unwrap();
                    }
                },
                Set { key, value, resp } => {
                    db.set(key, value.clone());
                    resp.send(Some(value.clone())).unwrap();
                }
            }
        }
    });

    // 查询数据，睡眠3秒后再查询，保证数据先添加成功
    sleep(Duration::from_secs(3));
    let (resp_tx, resp_rx) = oneshot::channel();
    tx3.send(Command::Get {
        key: String::from("k1"),
        resp: resp_tx,
    })
    .await
    .unwrap();
    let res = resp_rx.await.unwrap();
    println!("Get={:?}", String::from_utf8(res.unwrap().to_vec()));
}
