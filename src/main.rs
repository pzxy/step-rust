use std::cmp::Ordering;
// 默认会将 prelude 导入到每个包里面。如果要使用以外的库中内容，就需要使用 use将包导入进来。
use std::io;
// trait 相当于接口，里面定义了一些方法。
use rand::Rng;

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
    // 访问越界，要注意
    println!(b[12])
}

// 切片
fn r_slice() {
    let a = [1, 2, 3, 4, 5];
    // .. 和go里面的: 性质一样.
    let b = a[..3];
    println!("{}".a[4..])
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
        print!(number)
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
