use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    box_demo();
    rc_demo();
    rc_demo2();
    ref_cell();
}


fn box_demo() {
    let s = "hello_world";
    let b = Box::new(s);
    println!("{}", s);
}

fn rc_demo() {
    println!(">> rc demo1");
    let s = 100;
    let mut r = Rc::new(s);
    rc_println(&r);
    // 虽然rc不可变，但是可以通过这种方式改变。
    *Rc::make_mut(&mut r) += 2;
    rc_println(&r);
}

fn rc_demo2() {
    println!(">> rc demo2");
    let s = 100;
    let r = Rc::new(s);
    rc_println(&r);
    let weak = Rc::downgrade(&r);
    let weak = Rc::downgrade(&r);
    rc_println(&r);
    let r2 = Rc::clone(&r);
    rc_println(&r2);
}

fn rc_println(r: &Rc<i32>) {
    println!("strong_count: {}", Rc::strong_count(&r));
    println!("weak_count: {}", Rc::weak_count(&r));
    println!("r: {}", r);
}

fn ref_cell(){
    println!(">> ref cell");
    let r = RefCell::new(5);
    *r.borrow_mut() += 100;
    println!("{:?}",r);


}