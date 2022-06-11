// 智能指针:行为和指针相似,并且有额外的元数据,和额外的功能.
// 例子: String Vec<T>,都拥有一片内存区域,而且 String 还限制都是里面必须是 utf8 类型
// rust 智能指针,必须实现 Deref Drop 这两个 trait.
// 实现了 Deref ,就可以将里面的T当做引用来处理
// 实现了 Drop ,当所有变量离开作用域时,里面的指针和值都会被释放掉.
// 常见智能指针:
// Box<T>: heap 内存上分配值
// Rc<T>: 启动多重所有权的引用计数类型
// Ref<T> RefMut<T>:通过 RefCell<T>访问:在运行时而不是编译时强制借用规则的类型

// 1. Box 使用场景:
// - 在编译时,类型大小无法确定
// - 有大量数据,需要移交所有权,要保证在操作时数据不会被复制.
// - 使用某个值时,只关心实现了某个 trait,二不关心具体的类型时.
use crate::pointer::List::{Cons, Nil};

enum List {
    // Cons(i32, List),这种写法,编译不会通过的, 因为循环引用,不能确定结构大小.
    Cons(i32, Box<List>),
    Nil,
}

// Box<T>主要是
fn pointer1() {
    let l = Cons(1,
                 Box::new(Cons(2,
                               Box::new(Cons(3,
                                             Box::new(Nil))))));
}

// 2. Deref
// Box<T>其实里面是一个元组的智能指针,根据如此实现自己的只能指针 MyBox<T>,
// Deref trait 的要求,我们要实现要求,实现一个 deref 方法:1. 借用 self,返回一个执行内部数据的引用
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // strait 关联类型.
    type Target = T;
    // 返回的是一个引用
    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn pointer2() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // assert_eq!(5,y); 这样会提示错误.
    assert_eq!(5, *y);
}

// 解引用的调用过程
fn pointer22() {
    let m = MyBox::new(String::from("Rust"));
    // &m 相当于 &MyBox<String>
    // 然后默认调用 deref 就变成了 &String
    // 接着,String 也实现了 Deref trait ,因此调用 deref,最后变为 &str.
    // 这里的调用规则:这里类比 MyBox和 String,MyBox 中就是 T,String就是 U
    // - T: Deref<Target=U>,允许 &T 转换为 U
    // - T: DerefMut<Target=U>,允许 &mut T 转换为&mut U
    // - T: Deref<Target=U>,允许 &mut T 转换为&U ,但是反过来不行.
    point_print(&m);
    // 如果不使用解引用,这样也是一样的效果
    point_print(&(*m)[..]);
}

fn point_print(s: &str) {
    println!("{}", s);
}


// 3. Drop
// drop 在已导入模块中,直接可以用.
struct DropPointer {
    data: String,
}

impl Drop for DropPointer {
    fn drop(&mut self) {
        println!("DropPointer:{}", self.data)
    }
}

pub fn pointer3() {
    let a = DropPointer { data: String::from("第一") };
    let b = DropPointer { data: String::from("第二") };
    println!("pointer3 over");
    // 输出结果,执行顺序类似 golang 里面的 defer,先进后出.
    // pointer3 over
    // DropPointer:第二
    // DropPointer:第一

    // 这里也可以提前清理这个值,因为 drop 在与导入模块中,所以直接亦可以用.
    // drop()可以安全使用,不用担心调多次,
    drop(a);
    // 这时打印出来的顺序就是:
    // pointer3 over
    // DropPointer:第一
    // DropPointer:第二
}


// 4. Rc<T> 不在预先导入模块中,只使用于单线程.并且只能通过不可变引用.
// - clone 增加引用计数.不会执行深度 copy 操作,只是增加引用计数的值.
// - strong_count 获取引用计数,当所有值为 0 时释放这个值.
// - weak_count 获取引用计数,不为 0 时也可释放.
//    b--->3
//           \
//       a--> 5--->10--->nil
//           /
//    c---->4
use std::rc::Rc;
// 导入枚举值
use crate::pointer::List2::{Cons2, Nil2};

enum List2 {
    // Cons(i32, List),这种写法,编译不会通过的, 因为循环引用,不能确定结构大小.
    Cons2(i32, Rc<List2>),
    Nil2,
}

pub fn pointer4() {
    // let a = Cons(5,
    //              Box::new(Cons(10,
    //                            Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // 这里 a 会提示报错,Cons的定义是一个所有权,不是引用,上一行 a 的所有权已经被获取走了.
    // let c = Cons(4, Box::new(a));

    // Rc<T>改动以后
    let a = Rc::new(Cons2(5,
                          Rc::new(Cons2(10,
                                        Rc::new(Nil2)))));
    println!(" refer count after create a:{}", Rc::strong_count(&a));
    // rc::clone 不会执行深度的 copy 操作,只会增加指针计数.当strong_count为 0 时,会被释放掉.
    let b = Cons2(3, Rc::clone(&a));
    println!("refer count after create b:{}", Rc::strong_count(&a));

    {
        let c = Cons2(4, Rc::clone(&a));
        println!("refer count  after create c:{}", Rc::strong_count(&a));
    }
    // 这里c 离开作用域了可以看到减少了
    println!("refer count after leave c :{}", Rc::strong_count(&a));

    //输出:
    // refer count after create a:1
    // refer count after create b:2
    // refer count  after create c:3
    // refer count after leave c :2
}
