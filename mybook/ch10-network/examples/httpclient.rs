use std::collections::HashMap;
use std::thread;
use std::time::Duration;
fn main() {
    let client = reqwest::blocking::Client::new();
    let url = "http://127.0.0.1:8000";
    loop {
        thread::sleep(Duration::from_secs(3));
        let mut context = HashMap::new();
        context.insert("name", "kwin");
        context.insert("age", "12");
        match client.post(url).json(&context).send() {
            Ok(body) => {
                println!("body: {:?}", body.text().unwrap());
            }
            Err(e) => {
                println!("error: {:?}", e);
                return;
            }
        }
    }
}
