use std::cmp::Ordering;
use std::collections::HashMap;
use std::intrinsics::size_of;
// 默认会将 prelude 导入到每个包里面。如果要使用以外的库中内容，就需要使用 use将包导入进来。
use std::io;
use std::ptr::hash;
// trait 相当于接口，里面定义了一些方法。
use rand::Rng;
/*
嵌套引用
use std::io;
use std::io::Write;
上面两行相当于：
use std::io::{self,Write};
 */
fn main() {
    // 带了一个！，表示这是一个宏不是一个函数。
    println!("Hello, world!");
    // 1～101也是包头不包尾
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字: {}", secret_number);
    loop {
        println!("猜一个数");
        // 如果不加mut，则这个变量不能被二次赋值。
        // :: 表示new是String中的一个关联函数，类似java中静态方法。
        let mut guess = String::new();
        // 这里也可以取消上面的use，然后这样调用：
        // std::io::stdin().read_line 等等;
        // &表示引用，引用在rust中也是不可变的，所以要加上mut
        // read_line()返回类型io::Result，这相当于一个枚举类型，
        // 这个类型有两个字段 OK,Err,expect是对结果进行处理，如果返回是Err则中断程序并退出
        io::stdin().read_line(&mut guess).expect("无法读取");
        // 在这里重新 使用guess，是一个 shadow的概念
        //  let guess: u32 这样的写法，显示指定变量类型。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是:{}", guess);
        // 类似switch，
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),// 这种写法，是arm概念。
            Ordering::Greater => println!("Too big"),//arm
            Ordering::Equal => {
                println!("You win");//arm
                break;
            }
        }
    }
}


fn r_type() {
    // bool类型就是 bool
    // 整型 i32 u32 isize usize等
    // 浮点型 f32 f64
    // 字符类型比较特殊，是Unicode类型，支持emoji，比较特殊。范围 U+0000~U+D7FF , U+E000~U+10FFFF 。
    let a = '2';
    let b = '$';
    let c = '🤔';
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
    println!(b[12])
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
    println!("number:{}", number)
}

fn r_for_while_loop() {
    // loop 相当于go里面的for {}
    // while 就是传统的那种
    let a = [10, 20, 30, 40];
    for e in a.iter() {
        println!("value:{}", e);
    }
    // 包头不包尾，类似python shell等语法
    for number in (1..4).rev() {
        // 输出 3 2 1
        println!("{}", number)
    }
}

// 借用：将引用作为参数的，就是借用，如果要修改借用的数据，需要声明mut。


// struct
// 调试使用，rust包含调试功能，这是为这个结构体显示的选择这一个功能。
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// tuple struct
struct Color(i32, u32, i32);

fn r_struct(name: String, age: u32) {
    // 必须全部赋值，不能像go中，只声明一部分
    let user1 = User {
        // 入参字段和结构体字段一致，可以直接简写，类似name。
        name,
        age: 32,
    };
    // ..user1 使用 user1中的值，
    let user2 = User {
        name: String::from("123"),
        // 元素相同可以使用。
        ..user1
    };
    let color = Color(1, 2, 3);
    // 这样需要实现 Display
    // println!("{}", user1);
    // 这样需要实现 Debug，或者在结构体上加上 #[derive(Debug)]
    println!("{:?}", user1);
    // 打印更清晰
    println!("{:#?}", user2);
}

// 结构体的方法
// 可以有多个 impl User
impl User {
    // 获取名字的方法
    // fn getName(&self) -> String {
    //     // 这里之所以报错，是因为rust中，数据不能任意赋值
    //     // 除了基本类型和实现了Copy的trait的类型。
    //     // String在heap上，并且没主动实现copy，我们返回一个name时候，因为是借用，不能移交所有权。
    //     self.name
    // }
    fn getAge(&self) -> u32 {
        self.age
    }
    // 关联函数，关联函数不传self，可以直接通过结构体调用
    // 调用方式有点像java中静态方法，功能类似go中NewObject这种。
    // 调用方式 User::newUser("你好")
    fn newUser(name: String) -> User {
        User {
            name,
            age: 18,
        }
    }
}

// 枚举类型
// 别人可以用枚举类型。方式IpAddrKind::V4
enum IpAddrKind {
    // 数据可以是任何类型
    V4(u8, u8, u8, u8),
    V6,
    // 枚举类型是结构体
    V8 { a: u32 },
}

// 给枚举类型定义方法
impl IpAddrKind {
    fn call(&self) -> IpAddrKind {
        self::V4
    }
}

// Rust 中没有null
fn r_null() {
    // Some None这是 Prelude 预导入的Option<T>枚举类型的值，所以可以直接用。
    let some_number = Some(5);
    let some_string = Some("asd");
    // 上面两个都可以推断处类型，
    // 这里直接使用None是无法推断出类型的
    //如果使用枚举要定义类型，应该这样写，但是Option<i32> 不是i32类型。
    let absent_number: Option<i32> = None;
    // 这里不能相加，检查会提示的。
    let s2 = absent_number + some_number;
}

// if let
fn r_if_let() {
    let v = Some(8u32);
    match v {
        Some(3) => println!("three"),
        // 表示其他类型的匹配，因为枚举类型必须穷举，相当于 default
        _ => println!("other"),
    }
    // if let,只匹配一种情况，不用穷举。
    if let Some(3) = v {
        println!("three");
    } else {
        println!("other");
    }
}

// Vector，类似go中自动扩容的切片
fn r_vector() {
    // 创建
    // let mut  v = Vec::new();
    let mut v = vec![1, 2, 3, 4];
    // 这种写法是有问题的，v中添加新的值后，vector可能扩容，所以，这行后面不能v.push(5)
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
    let sp = &s[0..3];
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
    // let  initial_scores = vec![10,50];
    // 这里 声明未指定类型，因为可以推断出来。
    // let scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    // HashMap所有权，基本类型会Copy， String类型会转移所有权。

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