//Instructions
// Add the mine counts to a completed Minesweeper board.
//
// Minesweeper is a popular game where the user has to find the mines using numeric hints that indicate how many mines are directly adjacent (horizontally, vertically, diagonally) to a square.
//
// In this exercise you have to create some code that counts the number of mines adjacent to a given empty square and replaces that square with the count.
//
// The board is a rectangle composed of blank space (' ') characters. A mine is represented by an asterisk ('*') character.
//
// If a given space has no adjacent mines at all, leave that square blank.
//指示
// 将地雷计数添加到已完成的扫雷板上。
//
// Minesweeper 是一种流行的游戏，用户必须使用数字提示找到地雷，这些提示指示有多少地雷与正方形直接相邻（水平、垂直、对角线）。
//
// 在本练习中，您必须创建一些代码来计算与给定空方格相邻的地雷数量，并将该方格替换为计数。
//
// 棋盘是一个由空格 (' ') 字符组成的矩形。地雷由星号 ('*') 字符表示。
//
// 如果给定空间根本没有相邻的地雷，则将该正方形留空。
//
// 例子
// 例如，您可能会收到这样的 5 x 4 板（此处的空格用“·”字符表示，以便在屏幕上显示）：
//
// ·*·*·
// ··*··
// ··*··
// ·····
// 您的代码会将其转换为：
//
// 1*3*1
// 13*31
// ·2*2·
// ·111·

//[package]
// edition = "2021"
// name = "minesweeper"
// version = "1.1.0"
pub fn annotate2(minefield: &[&str]) -> Vec<String> {
    let mut ret = Vec::new();
    let row = minefield.len();
    if row == 0 {
        return ret;
    }
    let col = minefield[0].len();
    for r in 0..row {
        // 遍历行
        let mut row_str = String::new();
        for (c, v) in minefield[r].chars().enumerate() {
            if v == '*' {
                row_str.push("*".parse().unwrap());
                continue;
            }
            let mut count = 0;
            //  限制上下左右
            // 左上
            if r > 0 && c > 0 {
                match minefield[r - 1].as_bytes()[c - 1] {
                    b'*' => count += 1,
                    _ => ()
                }
            }

            // 向上
            if r > 0 {
                match minefield[r - 1].as_bytes()[c] {
                    b'*' => count += 1,
                    _ => ()
                }
            }
            // 右上
            if r > 0 && c + 1 < col {
                match minefield[r - 1].as_bytes()[c + 1] {
                    b'*' => count += 1,
                    _ => ()
                }
            }
            // 向右
            if c + 1 < col {
                match minefield[r].as_bytes()[c + 1] {
                    b'*' => count += 1,
                    _ => ()
                }
            }
            // 右下
            if r + 1 < row && c + 1 < col {
                match minefield[r + 1].as_bytes()[c + 1] {
                    b'*' => count += 1,
                    _ => ()
                }
            }
            // 向下
            if r + 1 < row {
                match minefield[r + 1].as_bytes()[c] {
                    b'*' => count += 1,
                    _ => ()
                }
            }
            // 左下
            if c > 0 && r + 1 < row {
                match minefield[r + 1].as_bytes()[c - 1] {
                    b'*' => count += 1,
                    _ => ()
                }
            }
            // 向左
            if c > 0 {
                match minefield[r].as_bytes()[c - 1] {
                    b'*' => count += 1,
                    _ => ()
                }
            }

            match count {
                0 => row_str.push(' '),
                _ => row_str.push(count.to_string().parse().unwrap()),
            }
        }
        ret.push(row_str);
    }

    ret
}

// 高分答案
static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1, 0), (1, 0),
    (-1, 1), (0, 1), (1, 1),
];

pub fn annotate(field: &[&str]) -> Vec<String> {
    let height = field.len() as i32;
    (0..height).map(|y| {
        let width = field[y as usize].len() as i32;
        (0..width).map(|x| {
            if field[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                match NEIGBOURHOOD_OFFSETS.iter()
                    .map(|&(ox, oy)| (x + ox, y + oy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
                    .count() {
                    0 => ' ',
                    n => (n as u8 + '0' as u8) as char
                }
            }
        }).collect()
    }).collect()
}


fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[]);
}

#[test]
#[ignore]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
#[ignore]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
#[ignore]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
#[ignore]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
#[ignore]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
#[ignore]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
#[ignore]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
#[ignore]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
#[ignore]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
#[ignore]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
#[ignore]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
