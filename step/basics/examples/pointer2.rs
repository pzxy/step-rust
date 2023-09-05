use std::rc::Rc;

fn main() {
    box_demo();
    rc_demo();
}


fn box_demo(){
    let s = "hello_world";
    let b = Box::new(s);
    println!("{}",s);
}

fn rc_demo(){
    let s = 100;
    let mut r = Rc::new(s);
    println!("strong_count: {}",Rc::strong_count(&r));
    println!("weak_count: {}",Rc::weak_count(&r));
    println!("r: {}",r);
    // 虽然rc不可变，但是可以通过这种方式改变。
    *Rc::make_mut(&mut r) += 2;
    println!("strong_count: {}",Rc::strong_count(&r));
    println!("weak_count: {}",Rc::weak_count(&r));
    println!("r: {}",r);
}

