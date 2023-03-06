use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

fn main() {
    demo1();
    println!("----------------------");
    demo2()
}

fn demo1() {
    // 直接解析 json 字符串
    let data = r#"{
            "name": "John Doe",
            "age": 43,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let v: Value = serde_json::from_str(data).unwrap();
    println!("直接解析json字符串: {},{}", v["name"], v["phones"][0]);

    // 使用宏解析 json 字符串
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "address": {
            "street": "10 Downing Street",
            "city": "London"
        },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!(
        "使用宏解析json字符串: {},{}",
        john["name"], john["phones"][0]
    );

    // 将字符串解析成结构体
    let data2 = r#"{
            "name": "John Doe",
            "age": 43,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let vv: Root = serde_json::from_str(data2).unwrap();
    println!("将字符串解析成结构体: {:?}", vv);
    // 结构体变字符串
    let root = Root {
        name: "Di".to_string(),
        age: 0,
        address: Address {
            street: "xu".to_string(),
            city: "shanghai".to_string(),
        },
        phones: vec!["18234487249".to_string()],
    };

    let s = serde_json::to_string(&root).unwrap();
    println!("结构体变字符串: {}", s);
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    pub street: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    pub name: String,
    pub age: i64,
    pub address: Address,
    pub phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    address: Address,
}

fn demo2() {
    let json_str = r#"
        {
            "name": "Alice",
            "age": 30,
            "address": {
                "city": "New York",
                "street": "123 Main St"
            }
        }
    "#;

    let person: Person = serde_json::from_str(json_str).unwrap();

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("City: {}", person.address.city);
    println!("Street: {}", person.address.street);
    let s = serde_json::to_string(&person).unwrap();
    println!("struct to json {}",s)
}
