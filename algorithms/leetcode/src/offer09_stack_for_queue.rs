//用两个栈实现一个队列。队列的声明如下，请实现它的两个函数 appendTail 和 deleteHead ，分别完成在队列尾部插入整数和在队列头部删除整数的功能。(若队列中没有元素，deleteHead 操作返回 -1 )
//
//
// 示例 1：
//
// 输入：
// ["CQueue","appendTail","deleteHead","deleteHead"]
// [[],[3],[],[]]
// 输出：[null,null,3,-1]
// 示例 2：
//
// 输入：
// ["CQueue","deleteHead","appendTail","appendTail","deleteHead","deleteHead"]
// [[],[],[5],[2],[],[]]
// 输出：[null,-1,null,null,5,2]
// 提示：
//
// 1 <= values <= 10000
// 最多会对appendTail、deleteHead 进行10000次调用
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::collections::HashMap;

#[derive(Default)]
struct CQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl CQueue {
    fn new() -> Self {
        Default::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.stack_in.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            for v in self.stack_in.iter().rev() {
                self.stack_out.push(*v)
            }
            self.stack_in = Vec::new()
        }
        self.stack_out.pop().unwrap_or(-1)
    }
    // 第二种方式
    fn delete_head2(&mut self) -> i32 {
        match self.stack_out.pop() {
            Some(s) => s,
            None => {
                while !self.stack_in.is_empty() {
                    self.stack_out.push(self.stack_in.pop().unwrap())
                }
                self.stack_out.pop().unwrap_or(-1)
            }
        }
    }
    fn delete_head3(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            self.stack_out = self.stack_in.iter().rev().map(|&x| x).collect();
            self.stack_in.clear()
        }
        self.stack_out.pop().unwrap_or(-1)
    }
}

// #[derive(Default)]
// struct CQueue {
//     p: Vec<i32>,
//     q: Vec<i32>,
// }
//
// impl CQueue {
//     fn new() -> Self {
//         Default::default()
//     }
//
//     fn append_tail(&mut self, value: i32) {
//         self.p.push(value);
//     }
//     fn delete_head(&mut self) -> i32 {
//         match self.q.pop() {
//             Some(s) => s,
//             None => {
//                 while !self.p.is_empty() {
//                     self.q.push(self.p.pop().unwrap())
//                 }
//                 self.q.pop().unwrap_or(-1)
//             }
//         }
//     }
// }
