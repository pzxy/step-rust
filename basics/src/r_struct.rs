
// struct
// 调试使用，rust包含调试功能，这是为这个结构体显示的选择这一个功能。
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// tuple struct
struct Color(i32, u32, i32);
// 元组结构体只包含一个元素时就是 newType 模式
// 可以定义分数的一些方法,可以和 u32进行区分
struct Score(u32);

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
// 单元结构体,不占用空间
struct Unit;

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