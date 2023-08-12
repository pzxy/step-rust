use std::{cell::RefCell, rc::Rc};

fn main(){
    let mut v = Rc::new(RefCell::new(5));
    let v2 = Rc::new(RefCell::new(6));
     v=v2;
    println!("123 {:?}",v);
}