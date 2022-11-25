//在一个 n * m 的二维数组中，每一行都按照从左到右递增的顺序排序，每一列都按照从上到下递增的顺序排序。
// 请完成一个高效的函数，输入这样的一个二维数组和一个整数，判断数组中是否含有该整数。
//
//
// 示例:
//
// 现有矩阵 matrix 如下：
//
// [
//   [1,   4,  7, 11, 15],
//   [2,   5,  8, 12, 19],
//   [3,   6,  9, 16, 22],
//   [10, 13, 14, 17, 24],
//   [18, 21, 23, 26, 30]
// ]

// 给定 target=5，返回true。
// 给定target=20，返回false。
// 限制：
// 0 <= n <= 1000
// 0 <= m <= 1000
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/er-wei-shu-zu-zhong-de-cha-zhao-lcof
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main(){
    let s = [[-5]];
    let array = find_number_in2_d_array(s, -10);
    println!("{}", array)
}

pub fn find_number_in2_d_array(matrix: [[i32; 1]; 1], target: i32) -> bool {
    if matrix.is_empty() {
        return false;
    }
    if matrix[0].is_empty() {
        return false;
    }
    let mut row = 0;
    let mut col = matrix[0].len() - 1;
    while row < matrix.len() && col >= 0 {
        if matrix[row][col] == target {
            return true;
        }
        if matrix[row][col] > target {
            // 这里是易错点,rust 中下标是无符号类型,不能为负数,
            if col == 0 {
                break;
            }
            // 如果没有上面检查,这里可能为负数.
            col -= 1;
        } else {
            row += 1;
        }
    }
    false
}

pub fn find_number_in2_d_array2(m: [[i32; 1]; 1], target: i32) -> bool {
    if m.is_empty() {
        return false;
    }
    if m[0].is_empty() {
        return false;
    }
    let mut row = 0;
    let mut col = m[0].len() - 1;
    while row < m.len() && col >= 0 {
        if m[row][col] == target {
            return true;
        }
        if m[row][col] > target {
            if col == 0 {
                break;
            }
            col -= 1;
        } else {
            row += 1;
        }
    }
    false
}
