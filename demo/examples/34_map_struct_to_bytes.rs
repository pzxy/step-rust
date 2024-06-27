use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

struct Node3 {
    a: u64,
    b: u32,
    c: [u32; 20],
}

struct Node2 {
    a: AtomicU64,
    b: u32,
    c: [AtomicU32; 20],
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Node {
    value: AtomicU64,
    key_offset: u32,
    key_size: u16,
    height: u16,
    pub(crate) tower: [AtomicU32; 20],
}

pub struct Area {
    n: AtomicU32,
    is_grow: bool,
    buf: RefCell<Vec<u8>>,
}

impl Area {
    pub(crate) fn get_buf(&self) -> std::cell::Ref<'_, Vec<u8>> {
        self.buf.borrow()
    }
    pub(crate) fn get_buf_mut(&self) -> std::cell::RefMut<'_, Vec<u8>> {
        self.buf.borrow_mut()
    }
    pub(crate) fn new(n: u32) -> Area {
        Area {
            n: AtomicU32::new(1),
            is_grow: false,
            buf: RefCell::new(vec![0; n as usize]),
        }
    }
    pub(crate) fn get_node_mut(&self, offset: u32) -> Option<Rc<&mut Node>> {
        if offset == 0 {
            return None;
        }
        let x = unsafe {
            mem::transmute::<&mut u8, &mut Node>(&mut self.get_buf_mut()[offset as usize])
        };

        return Some(Rc::new(x));
    }

    pub(crate) fn get_node(&self, offset: u32) -> Option<Rc<&Node>> {
        if offset == 0 {
            return None;
        }
        let x = unsafe {
            mem::transmute::<&u8, &Node>(&self.get_buf()[offset as usize])
        };

        return Some(Rc::new(x));
    }
}

fn a() {
    let area = Rc::new(Area::new(1000));
    let node1: &Node = *area.get_node(8).unwrap();
    let node2 = *node1;
    let mut node = area.get_node_mut(8).unwrap();
    {
        let n = Rc::get_mut(&mut node).unwrap();
        n.key_offset = 104;
        n.key_size = 1;
        n.height = 20;
        n.value = AtomicU64::from(8589934696);
        let x = &area.get_buf()[8..104];
        println!("new_node :{:?}", x.to_vec());
    }
    println!("new_node :{:?}", node2);
    println!("ok");
    return;
}


fn b() {
    let mut b = vec![0u8; 1000];
    let n = unsafe {
        mem::transmute::<&mut u8, &mut Node>(&mut b[8])
    };

    {
        n.key_offset = 104;
        n.key_size = 1;
        n.height = 20;
        n.value = AtomicU64::from(8589934696);
        // let x = &[8..104];
        println!("new_node :{:?}", b[8..104].to_vec());
    }
    println!("ok");
    return;
}

fn main() {
    a();
    // let mut a = vec![0u8; 1000];
    // let x = unsafe {
    //     mem::transmute::<&mut u8, &mut Node3>(&mut a[8])
    // };
    // x.a = 1;
    // x.b = 2;
    // x.c = [100u32; 20];
    // println!("{:?}", a);
    //
    // let mut b = vec![0u8; 1000];
    // let x = unsafe {
    //     mem::transmute::<&mut u8, &mut Node2>(&mut b[8])
    // };
    // x.a.fetch_add(1, Ordering::Relaxed);
    // x.b = 2;
    // for i in 0..20 {
    //     x.c[i] = AtomicU32::new(100);
    // }
    // println!("{:?}", b);
    // assert_eq!(a, b);
}