/// enum Link {
///     Empty,
///     More(Box<Node>)
/// }
/// 和 Option<Box<Node>> 非常像

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

pub struct List {
    head: Link,
}

// 类型别名，type alias
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            // next: mem::replace(&mut self.head, None),
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match mem::replace(&mut self.head, None) 相当于 match self.head.take()
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        // 下面这个代码相当于上面的代码。map如果是Node的话用闭包f()的处理。
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

// impl Drop for List {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = boxed_node.next.take();
//         }
//     }
// }
