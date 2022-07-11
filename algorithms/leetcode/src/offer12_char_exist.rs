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

pub struct point {
    x: i32,
    y: i32,
}

impl point {
    fn add(&self, p: point) -> point {
        point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

// 这是错误的，rust静态以及产量，数据要保证是是不可变的。而这是宏，宏代码编译时候会变化的
// static dirs: Vec<point> = vec![point { x: 0, y: 1 }, point { x: 1, y: 0 }, point { x: 0, y: -1 }, point { x: -1, y: 0 }];

pub fn dfs(board: &mut Vec<Vec<char>>, p: point, word: String, k: i32) -> bool {
    if board[p.x as usize][p.y as usize] != word.as_bytes()[k as usize] as char {
        return false;
    }
    if word.len() - 1 == k as usize {
        return true;
    }
    let v = board[p.x as usize][p.y as usize];
    board[p.x as usize][p.y as usize] = ' ';
    for v in vec![
        point { x: 0, y: 1 },
        point { x: 1, y: 0 },
        point { x: 0, y: -1 },
        point { x: -1, y: 0 },
    ] {
        let tmp = p.add(v);
        if check(board, &tmp) == false {
            continue;
        }
        if dfs(board, tmp, word.clone(), k + 1) == true {
            return true;
        }
    }
    board[p.x as usize][p.y as usize] = v;
    false
}

fn check(board: &mut Vec<Vec<char>>, p: &point) -> bool {
    let x = board.len();
    let y = board[0].len();
    if p.x < 0i32 || p.x >= x as i32 {
        return false;
    }
    if p.y < 0i32 || p.y >= y as i32 {
        return false;
    }
    return true;
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let b = &mut board.clone();
    for (x, m) in board.iter().enumerate() {
        for (y, n) in m.iter().enumerate() {
            if dfs(
                b,
                point {
                    x: x as i32,
                    y: y as i32,
                },
                word.clone(),
                0,
            ) == true
            {
                return true;
            }
        }
    }
    false
}

struct Solution {}

impl Solution {
    fn dfs(board: &mut Vec<Vec<char>>, chars: &Vec<char>, y: usize, x: usize, i: usize) -> bool {
        if board[y][x] != chars[i] {
            return false;
        }
        if i + 1 == chars.len() {
            return true;
        }
        let temp = board[y][x];
        board[y][x] = ' ';
        if y != 0 && Solution::dfs(board, chars, y - 1, x, i + 1)
            || x != 0 && Solution::dfs(board, chars, y, x - 1, i + 1)
            || y != board.len() - 1 && Solution::dfs(board, chars, y + 1, x, i + 1)
            || x != board[0].len() - 1 && Solution::dfs(board, chars, y, x + 1, i + 1)
        {
            return true;
        }
        board[y][x] = temp;
        false
    }

    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let chars = word.chars().collect::<Vec<char>>();
        for i in 0..m {
            for j in 0..n {
                if Solution::dfs(&mut board, &chars, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }
}
