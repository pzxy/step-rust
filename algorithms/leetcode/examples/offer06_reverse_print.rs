//输入一个链表的头节点，从尾到头反过来返回每个节点的值（用数组返回）。
//
//
//
// 示例 1：
//
// 输入：head = [1,3,2]
// 输出：[2,3,1]

// Definition for singly-linked list.

fn main(){}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    // 内联,编译后会内联到其他代码中.
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut p = head;
    // 构建一个栈，用来存储链表中每个节点的值
    let mut stack = Vec::new();

    // 构建一个指针，指向链表的头结点位置，从它开始向后遍历
    // 不断的遍历原链表中的每个节点，直到为 null
    while let Some(mut cur) = p {
        p = cur.next.take();
        // 把每个节点的值加入到栈中
        stack.push(cur.val);
        // curNode 向后移动
    }
    stack.iter().rev().map(|&x| x).collect::<Vec<i32>>()
}

pub fn reverse_print2(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut p = head;
    let mut stack = Vec::new();
    while let Some(mut cur) = p {
        p = cur.next.take();
        stack.push(cur.val)
    }
    stack.iter().rev().map(|&x| x).collect()
}

pub fn reverse_print3(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut p = head;
    let mut stack = Vec::new();
    while let Some(mut cur) = p {
        p = cur.next;
        stack.push(cur.val);
    }
    stack.iter().map(|&x| x).collect()
}

