//输入某二叉树的前序遍历和中序遍历的结果，请重建该二叉树。假设输入的前序遍历和中序遍历的结果中都不含重复的数字。
//
//
//
// 例如，给出
//
// 前序遍历 preorder =[3,9,20,15,7]
// 中序遍历 inorder = [9,3,15,20,7]
// 返回如下的二叉树：
//
//     3
//    / \
//   9  20
//     /  \
//    15   7
//
//
// 限制：
//
// 0 <= 节点个数 <= 5000

//前序是为了找到根结点，中序是为了确定左右子树。
//但是，如果是[3,9,null,null,20,15,7]这种将空节点也存去的数组。那么可以直接构建，不需要要其他操作了。

use std::cell::RefCell;
use std::rc::Rc;


fn main(){}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    // treeNode 会被其他 treeNode对象多个同时引用,所以要用 refCell 包起来.
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
// 空间换时间

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() != inorder.len() {
        return None;
    }
    if preorder.len() > 5000 {
        return None;
    }
    for (i, root) in inorder.iter().enumerate() {
        if *root == preorder[0] {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: *root,
                left: build_tree(preorder[1..i + 1].to_vec(), inorder[0..i].to_vec()),
                right: build_tree(preorder[i + 1..].to_vec(), inorder[i + 1..].to_vec()),
            })));
        }
    }
    None
}

// 时间换空间
fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    let mut root = TreeNode::new(preorder[0]);
    let index = inorder.iter().position(|x| x == &preorder[0]).unwrap();

    root.left = build(&preorder[1..index + 1], &inorder[0..index]);
    root.right = build(&preorder[index + 1..], &inorder[index + 1..]);
    Some(Rc::new(RefCell::new(root)))
}

// impl Solution {
//     pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
//         build(&preorder[..], &inorder[..])
//     }
// }
//
