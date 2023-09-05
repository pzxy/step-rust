// 智能指针:行为和指针相似,并且有额外的元数据,和额外的功能.
// 例子: String Vec<T>,都拥有一片内存区域,而且 String 还限制都是里面必须是 utf8 类型
// rust 智能指针,必须实现 Deref Drop 这两个 trait.实现了以后就可以自动管理内存了
// 实现了 Deref ,就可以将里面的T当做引用来处理
// 实现了 Drop ,当所有变量离开作用域时,里面的指针和值都会被释放掉.
// 常见智能指针:
// Box<T>: heap 内存上分配值
// Rc<T>: 启动多重所有权的引用计数类型
// Ref<T> RefMut<T>:通过 RefCell<T>访问:在运行时而不是编译时强制借用规则的类型
#![allow(unused)]
#![allow(dead_code)]

use std::cell::RefCell;
// 2. Deref
// Box<T>其实里面是一个元组的智能指针,根据如此实现自己的只能指针 MyBox<T>,
// Deref trait 的要求,我们要实现要求,实现一个 deref 方法:1. 借用 self,返回一个执行内部数据的引用
use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::List2::{Cons2, Nil2};
use crate::List3::{Cons3, Nil3};
use crate::List::{Cons, Nil};
use crate::Node2::Cons5;
use crate::Node::{Cons4, Nil4};

// 1. Box 使用场景:
// - 在编译时,类型大小无法确定
// - 有大量数据,需要移交所有权,要保证在操作时数据不会被复制.
// - 使用某个值时,只关心实现了某个 trait,二不关心具体的类型时.
// 导入枚举值
// 5. RefCell<T> 只会在运行时检查借用规则,
// 可以通过调用方法,来修改内部的值,虽然外部是不可变的,但是内部是可变的,从而达到内部可变性.
// 通俗点讲,就是将数据放在head上,然后 refcell 中维护一个可变引用和多个不可变引用的变量,这些变量只能通过指针获取到.
// 6. 写一个循环引用,造成内存泄漏,蛇咬住自己的尾巴.

fn main() {}

enum List {
    // Cons(i32, List),这种写法,编译不会通过的, 因为循环引用,不能确定结构大小.
    Cons(i32, Box<List>),
    Nil,
}

// Box<T>主要是
fn _pointer1() {
    let _l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

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
    println!("-------pointer2------");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // assert_eq!(5,y); 这样会提示错误.
    assert_eq!(5, *y);
}

// 解引用的调用过程
fn _pointer22() {
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
    println!("-------pointer3------");

    let a = DropPointer {
        data: String::from("第一"),
    };
    let _b = DropPointer {
        data: String::from("第二"),
    };
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

// 4. Rc<T> ,refer count,不在预先导入模块中,只使用于单线程.并且只允许不可变借用.
// - clone 增加引用计数.不会执行深度 copy 操作,只是增加引用计数的值.
// - strong_count 获取引用计数,当所有值为 0 时释放这个值.
// - weak_count 获取引用计数,不为 0 时也可释放.

enum List2 {
    // Cons(i32, List),这种写法,编译不会通过的, 因为循环引用,不能确定结构大小.
    Cons2(i32, Rc<List2>),
    Nil2,
}

//    b--->3
//           \
//       a--> 5--->10--->nil
//           /
//    c---->4
pub fn pointer4() {
    println!("-------pointer4------");

    // let a = Cons(5,
    //              Box::new(Cons(10,
    //                            Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // 这里 a 会提示报错,Cons的定义是一个所有权,不是引用,上一行 a 的所有权已经被获取走了.
    // let c = Cons(4, Box::new(a));

    // Rc<T>改动以后
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!(" refer count after create a:{}", Rc::strong_count(&a));
    // rc::clone 不会执行深度的 copy 操作,只会增加指针计数.当strong_count为 0 时,会被释放掉.
    let _b = Cons2(3, Rc::clone(&a));
    println!("refer count after create b:{}", Rc::strong_count(&a));

    {
        let _c = Cons2(4, Rc::clone(&a));
        println!("refer count after create c:{}", Rc::strong_count(&a));
    }
    // 这里c 离开作用域了可以看到减少了
    println!("refer count after leave c :{}", Rc::strong_count(&a));

    // 输出:
    // refer count after create a:1
    // refer count after create b:2
    // refer count after create c:3
    // refer count after leave c :2
}

#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

pub fn pointer5() {
    println!("-------pointer5------");

    let v = RefCell::new(5);
    println!("value1:{}", v.borrow());
    // 可变借用,直接修改里面的值
    *v.borrow_mut() += 10;
    println!("plus 10, value2:{}", v.borrow());

    // 常用例子,和 Rc 配合使用
    // RefCell 可以通过borrow_mut 修改内部值
    // Rc 可以通过clone 来创建增加引用计数.

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));
    let b = Cons3(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(10)), Rc::clone(&a));
    // 改变内部值
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Cell<T> 通过复制访问数据
// Mutex<T> 跨线程访问内存可变

#[derive(Debug)]
enum Node {
    // Rc 增加计数,RefCell 修改 Rc
    Cons4(i32, RefCell<Rc<Node>>),
    Nil4,
}

impl Node {
    fn tail(&self) -> Option<&RefCell<Rc<Node>>> {
        match self {
            // Some 返回的都是引用
            Cons4(_, item) => Some(item),
            Nil4 => None,
        }
    }
}

// 这里会出现循环引用.
pub fn pointer6() {
    // a 节点
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));
    // b 节点 -> a 节点
    let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));
    if let Some(link) = a.tail() {
        // a 节点 -> b 节点
        *link.borrow_mut() = Rc::clone(&b);
    }
    // 出现堆栈溢出 overflowed  stack
    // println!("{:?}",b.tail())
}

// 7. 如何避免循环引用?
// 将 Rc<T> 换成 Weak<T> 实例的 strong_count 加 1,Rc<T>的实例只有在 strong_count 为 0 的时候才会被清理.
// Rc<T>实例通过调用 Rc::downgrade 方法可以创建的 WeakReference(弱引用):
// - 返回类型是 Weak<T>(智能指针)
// - 调用 Rc::downgrade 会为 weak_count 加 1.
// Rc<T> 使用 weak_count 来追踪存在多少 Weak<T>.
// weak_count 不为 0 并不影响 Rc<T>实例的道理
// 强引用为 0 时,弱引用会自动断开.
// 使用 Weak<T> 实例上的调用的 upgrade 方法,返回 Option<Rc<T>>,用此来判断他指向的值是否存在.

#[derive(Debug)]
enum Node2 {
    // Rc 增加计数,RefCell 修改 Rc
    Cons5(i32, RefCell<Weak<Node2>>),
    Nil5,
}

impl Node2 {
    fn tail(&self) -> Option<&RefCell<Weak<Node2>>> {
        // println!("inner:{:?}", self);
        match self {
            // Some 返回的都是引用
            Cons5(_, item) => {
                // println!("value:{:?}", item.borrow().upgrade());
                Some(item)
            }
            Nil5 => None,
        }
    }
}

// 这里虽然写法差不多,但是没有出现循环应用.
// Rc::downgrade(&a),获取 Rc 的弱引用
// link.borrow().upgrade(),获取弱引用指向的值.
// rust 中实现一个链表没必要一定要这样写,和go 里面一样写也是可以的.
pub fn pointer7() {
    // a 节点
    let a = Rc::new(Cons5(5, RefCell::new(Weak::new())));
    // b 节点 -> a 节点
    println!(
        "a strong:{},weak:{}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    let b = Rc::new(Cons5(10, RefCell::new(Rc::downgrade(&a))));
    println!(
        "a strong:{},weak:{}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b strong:{},weak:{}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );

    if let Some(link) = a.tail() {
        // a 节点 -> b 节点
        *link.borrow_mut() = Rc::downgrade(&b);
        // 获取弱引用指向的值,应该是 b 的值 10
        println!("weak pointer value:{:?}", link.borrow().upgrade());
    }
    println!(
        "a strong:{},weak:{}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b strong:{},weak:{}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    // 出现堆栈溢出 overflowed  stack
    println!("{:?}", b.tail())
}
