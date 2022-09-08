// 借用：将引用作为参数的，就是借用，如果要修改借用的数据，需要声明mut。

use std::fmt::Display;

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

// 基本类型，分配在栈上的数据，都实现了Copy这个trait
// 分配在heap上的数据，实现的是Clone

/*
生命周期
*/
// 生命周期作用：避免悬垂引用(danging reference)
// 1. 楔子
fn r_life() {
    {
        let r;
        {
            let x = 5;
            // 这里会报错，因为将借用的x赋值给r后，再往下走一行，x就被释放了。
            r = &x;
        }
        // x已经被释放了，但是这里还使用x的值
        println!("r:{}", r);
    }
}

// 2. 这种写法会报错，因为这两个是借用，他们的声明周期是依据原来类型的声明周期的。
// 但是作为方法独立的存在，他是没有上下文的，入参是两个，返回是一个，所以返回值并不知道返回的是哪个类型的声明周期。
// 这里和代码逻辑没有关系。
// 可以反推出rust在编译阶段就知道了所有变量的生命周期。
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 3. 显示声明周期
// 这里这个'后面不一定是a，可以是任何字符，这里只是习惯写法。
// 这里只是说明，这个函数有'a这个声明周期，后面的参数和返回值使用这个'a，只是强调他们的声明周期谁一致的。
// 确切的说，是两个参数中生命周期最小的那个。这点非常重要。
fn longest3<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 例子3.1
fn longest3Main1() {
    let string1 = String::from("hello");
    let result;
    {
        // 这里string2是字符串字面值，他是静态的的，他的声明周期是全局的，在整个程序运行期间都有效。
        let string2 = "word";
        result = longest(string1.as_str(), string2);
    }
    // 这里没有问题，就因为string2是静态的，所以无论longest返回哪个值，最小生命周期都可以到这里。
    println!("The longest string is {}", result)
}

// 例子3.2
fn longest3Main2() {
    let string1 = String::from("hello");
    let result;
    {
        let string2 = String::from(" nice");
        result = longest(string1.as_str(), string2.as_str());
    }
    // 这里会有问题，因为参数string2的生命周期不够长，所以返回值，最小的生命周期已经失效了。
    println!("The longest string is {}", result)
}

// 例子3.3
fn longest33<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("asd");
    // 返回值是一个引用，这里创建出来的result，这个函数返回时已经被释放了。
    // 但是，result返回以后 这个值还是会被使用。就会引起悬垂指针
    // 可以将返回值改为String类型，这样就会移交所有权。或者使用静态类型。
    result
}

// 4. 生命周期省略规则
fn r_life4(x: &str) -> &str {
    let s = "sd";
}
// 4.1.  每个声明引用类型的参数都有自己的生命周期，单个参数有一个，多个参数有多个
// fn r_life41<'a>(x: &'a str) -> &str {
//     let s= "sd";
// }
// fn r_life41<'a,'b>(x: &'a str,y:&'b str) -> &str {
//     let s= "sd";
// }

// 4.2.  如果只有一个输入生命周期参数，那么该生命周期被付给所有的输出生命周期参数
// fn r_life42<'a>(x: &'a str) -> &'a str {
//     let s= "sd";
// }

// 4.3.  如果有多个输入生命周期参数，那么其中一个&self 或 &mut self ，那么 self
//的生命周期就会被赋给所有的输出生命周期。

struct ImportantExcerpt<'a> {
    // 如果要给这个值赋值，那么这个值的生命周期不能小于 结构体ImportantExcerpt对象 的生命周期
    part: &'a str,
}

// 给结构体方法
impl<'a> ImportantExcerpt<'a> {
    // 因为输入生命周期有self，所以返回值的生命周期和self是一致的。
    fn demo43(&self, name: &str) -> &str {
        self.part
    }
}

// 5. 生命周期 泛型 trait bound
// 生命周期也是泛型的一种
fn longest5<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display,
{
    println!("Announcement {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
