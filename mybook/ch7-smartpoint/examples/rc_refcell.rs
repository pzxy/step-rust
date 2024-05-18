use std::cell::RefCell;
use std::rc::Rc;

struct Pie {
    a: isize,
}

impl Pie {
    fn eat_change(&mut self) {
        self.a += 1;
        println!("eat_change a:{}", self.a)
    }
    fn eat(&self) {
        println!("eat a:{}", self.a)
    }
}

fn main() {
    rc_mut();
    ref_count();
    ref_cell();
}

fn rc_mut() {
    println!("> rc_mut");
    let mut pie = Rc::new(Pie { a: 1 });
    {
        // 共享所有权。
        let pie1 = Rc::clone(&pie);
        let pie2 = Rc::clone(&pie1);
        let pie3 = Rc::clone(&pie2);
        pie1.eat();
        pie3.eat();
        pie2.eat();
        println!("strong:{},weak:{}", Rc::strong_count(&pie), Rc::weak_count(&pie));
    }
    // pie1 pie2 pie3离开作用域被销毁。
    println!("strong:{},weak:{}", Rc::strong_count(&pie), Rc::weak_count(&pie));
    // 但是要改变值，需要通过 Rc::get_mut，当生命周期只有一个strong计数时才能改变。
    if let Some(p) = Rc::get_mut(&mut pie) {
        p.eat_change();
    }
}

// 2. Rc<T> ,refer count,不在预先导入模块中,只使用于单线程.并且只允许不可变借用.
// - clone 增加引用计数.不会执行深度 copy 操作,只是增加引用计数的值.
// - strong_count 获取引用计数,当所有值为 0 时释放这个值.
// - weak_count 获取引用计数,不为 0 时也可释放.

fn ref_count() {
    println!("> ref_count");
    let a = Rc::new("any type");
    let _b = a.clone();
    let _c = a.clone();
    println!(
        "a strong:{},weak:{}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    // 获取到弱引用
    let weak = Rc::downgrade(&a);
    let _d = weak.clone();
    let _c = a.clone();
    println!(
        "a strong:{},weak:{}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
}

fn ref_cell() {
    println!("> ref_cell");
    let v = RefCell::new(5);
    println!("refCell:{}", v.borrow());
    // 可变借用,直接修改里面的值
    *v.borrow_mut() += 10;

    println!("refCell after:{}", v.borrow());
    println!("refCell after:{}", v.borrow());

    // 常用例子,和 Rc 配合使用
    // RefCell 可以通过borrow_mut 修改内部值
    // Rc 可以通过clone 来创建增加引用计数.
    let value = Rc::new(RefCell::new(5));
    // 改变内部值
    println!("refCell after:{}", v.borrow());

    *value.borrow_mut() += 10;
    println!("rc refCell after = {}", value.borrow());
}

// 总结：
// Rc RcCell 都是单线程
// Rc 共享所有权
// RcCell 内部可变
// Rc + RcCell 共享所有权并且内部可变。
