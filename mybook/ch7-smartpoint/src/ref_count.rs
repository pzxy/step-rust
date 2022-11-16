#![allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;

// 2. Rc<T> ,refer count,不在预先导入模块中,只使用于单线程.并且只允许不可变借用.
// - clone 增加引用计数.不会执行深度 copy 操作,只是增加引用计数的值.
// - strong_count 获取引用计数,当所有值为 0 时释放这个值.
// - weak_count 获取引用计数,不为 0 时也可释放.

#[test]
fn ref_count_() {
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

#[test]
fn ref_cell() {
    let v = RefCell::new(5);
    println!("refCell:{}", v.borrow());
    // 可变借用,直接修改里面的值
    *v.borrow_mut() += 10;
    println!("refCell after:{}", v.borrow());

    // 常用例子,和 Rc 配合使用
    // RefCell 可以通过borrow_mut 修改内部值
    // Rc 可以通过clone 来创建增加引用计数.
    let value = Rc::new(RefCell::new(5));
    // 改变内部值
    *value.borrow_mut() += 10;
    println!("rc refCell after = {}", value.borrow());
}

// 总结：
// Rc RcCell 都是单线程
// Rc 共享所有权
// RcCell 内部可变
// Rc + RcCell 共享所有权并且内部可变。
