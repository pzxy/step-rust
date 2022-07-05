// 请设计一个函数，用来判断在一个矩阵中是否存在一条包含某字符串所有字符的路径。
// 路径可以从矩阵中的任意一格开始，每一步可以在矩阵中向左、右、上、下移动一格。
// 如果一条路径经过了矩阵的某一格，那么该路径不能再次进入该格子。
// 例如，在下面的3×4的矩阵中包含一条字符串“bfce”的路径（路径中的字母用加粗标出）。
//
// ["a","b","c","e"],
// ["s","f","c","s"],
// ["a","d","e","e"],
//
// 但矩阵中不包含字符串“abfb”的路径，因为字符串的第一个字符b占据了矩阵中的第一行第二个格子之后，路径不能再次进入这个格子。
//
//
//
// 示例 1：
//
// 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// 输出：true
// 示例 2：
//
// 输入：board = [["a","b"],["c","d"]], word = "abcd"
// 输出：false

use core::panic::BoxMeUp;

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    false
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

const dirs: Vec<point> = vec![point { x: 0, y: 1 }, point { x: 1, y: 0 }, point { x: 0, y: -1 }, point { x: -1, y: 0 }];

pub fn dfs(board: Vec<Vec<char>>, p: Point, word: String, k: i32) -> bool {
    false
}
