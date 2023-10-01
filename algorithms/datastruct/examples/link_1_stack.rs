/// 1. 这些写会有问题。因为循环引用
/// recursive type `List` has infinite size
///  # Examples
/// pub enum List {
///     Empty,
///     Elem(i32, List),
/// }
///
/// 2. 有以下两个问题
/// - 最后一个节点分配在了堆上，但是它看上去根本不像一个 Node
/// - 第一个 Node 是存储在栈上的，结果一家子不能整整齐齐的待在堆上了
/// # Examples
/// pub enum List {
///     Empty,
///     Elem(i32, Box<List>),
/// }

pub struct List {
    head: Link,
}

#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> List {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            // let c = std::mem::replace(&mut a,b)， 返回a上的值c，并且将b放到a上。
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    assert_eq!(list.pop(), None);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    list.push(4);
    list.push(5);
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}
