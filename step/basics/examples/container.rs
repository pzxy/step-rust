#![allow(unused)]
#![allow(dead_code)]
use std::collections::HashMap;

fn main() {

}

// tuple 元组
fn r_tuple() {
    // 可以将不同类型数据放到一起
    let s: (i32, f64, u8) = (500, 6.4, 1);
    // 获取tup元素，访问tup元素
    let (x, y, z) = s;
    println!("{} ,{} ,{}", x, y, z);
    // 通过.index获取
    println!("{} ,{} ,{}", s.0, s.1, s.2);
}

// 数组，将数据放在栈上，而不是堆上
fn r_array() {
    // 自动判断类型
    let a = ["1", "2", "3"];
    // 声明类型
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // [3,3,3,3,3]
    let c = [3; 5];
    // 访问越界，要注意,编译器会检查出来
    // println!(b[12])
}

// 切片
fn r_slice() {
    let a = [1, 2, 3, 4, 5];
    // .. 和go里面的: 性质一样.
    let b = a[..2];
}

// 语句和表达式，语句包括表达式，语句返回值是空的，表达式有返回值{},
// ;是告诉一个语句的结束。
fn r_fn() {
    let condition = true;
    // if和else都是表达式，表达式是有返回值的，所以可以放在等号的右边。
    // 表达式的最后一个值就是表达式的返回值，这里if表达式返回的是1。
    let number = if condition { 1 } else { 2 };

    let condition = true;
    println!("number:{}", number)
}

// Vector，类似go中自动扩容的切片
fn r_vector() {
    // 创建
    // let mut  v = Vec::new();
    let mut v = vec![1, 2, 3, 4];
    // 这种写法是有问题的，v中添加新的值后，vector可能扩容，
    // 引用地址可能变化，所以，这行后面不能v.push(5)
    // let first = &v[0];
    // 追加值
    v.push(5);
    // 获取
    // 这种出现越界会panic
    let third: &i32 = &v[100];
    println!("third:{}", third);
    // 这种因为判断了None的情况，所以不会panic。
    match v.get(100) {
        Some(third) => println!("third:{}", third),
        None => print!("none"),
    }
    // 改变
    for i in &mut v {
        *i += 50;
    }

    for i in &mut v {}
    // 发现值已经被改变了
    for i in v {
        println!("{}", i)
    }
    // 以上为止，vector中只是放了相同的元素，通过和enum；类型配合。vector中可以放不同的类型的值。
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// String,是对 Vec<u8>的包装
fn r_string() {
    // 更新
    let mut s = String::from("foo");
    s.push_str("bar");
    // 单个字符
    s.push('c');
    // 拼接 可以使用+，也可以如下：
    let s2 = format!("{}-{}", s, s);
    // 这是utf-8编码，所以每次字符占用一个字节，
    // 如果是unicode字符，那就是每个字符两个字节。
    let len = String::from("Hasd").len();
    // 字符串切割
    // 如果这个字符是unicode字符，每两个字节表示一个字符，我们截取三个字节。也就是一个半字符
    // 这种情况会发生panic，不能获取半个字符，所以，切割字符串的时候要慎重。
    // let sp = &s[0..3];
}

// HashMap,需要引入 use::collections::HashMap;
// 同构，所有k-v必须是同一种类型，数组存在heap上。
// HashMap性能一般，为了安全考虑，可以抵挡Dos攻击，如果要有高性能，实现hasher trait
fn r_hashMap() {
    // 创建
    // let mut scores: HashMap<String, i32> = HashMap::new();
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 10);

    // 通过元组创建，可以创建很多种类型，hashMap是其中一种。
    // let teams = vec![String::from("Blue"),String::from("Yellow")];
    // let initial_scores = vec![10,50];
    // 这里 声明未指定类型，因为可以推断出来。
    // let scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    // HashMap所有权，基本类型会Copy， String类型会转移所有权。
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores = teams.iter().zip(initial_scores).collect();
    // let field_name = String::from("Favorite Color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // 执行了这一步以后，field_name field_value的所有权，已经转移了，这两个字段后续已经无法使用了。
    // 除非使用 map.insert(&field_name, &field_value);
    // map.insert(field_name, field_value);

    // 获取hashMap
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // let k = String::from("Blue");
    // let score = scores.get(&team_name);
    // 通过Option来判断
    // match score {
    //     Some(s) => println!("{}", s),
    //     None => println!("team not exist"),
    // }

    // 遍历hashMap
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // for (k, v) in &scores {
    //     println!("{},{}", k, v);
    // }

    //更新hashMap，正常情况一样，相同就会覆盖
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // 这里会覆盖掉
    scores.insert(String::from("Blue"), 50);
    // 如果不存在再更新 entry, 并不会改变为60，因为 String::from("Blue") 已经存在了。
    scores.entry(String::from("Blue")).or_insert(60);
    println!("{:?}", scores);

    // 例子：统计字符串单词数量
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map)
}

