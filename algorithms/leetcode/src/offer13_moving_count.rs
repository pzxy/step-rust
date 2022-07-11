// 地上有一个m行n列的方格，从坐标 [0,0] 到坐标 [m-1,n-1] 。一个机器人从坐标 [0, 0] 的格子开始移动，
// 它每次可以向左、右、上、下移动一格（不能移动到方格外），也不能进入行坐标和列坐标的数位之和大于k的格子。
// 例如，当k为18时，机器人能够进入方格 [35, 37] ，因为3+5+3+7=18。但它不能进入方格 [35, 38]，因为3+5+3+8=19。
// 请问该机器人能够到达多少个格子？
//
//
// 示例 1：
// 输入：m = 2, n = 3, k = 1
// 输出：3

// 示例 2：
// 输入：m = 3, n = 1, k = 0
// 输出：1

// 提示：
// 1 <= n,m <= 100
// 0 <= k<= 20
// 通过次数265,514提交次数496,968

// 这里只是举例了可以将数字转换成字符来获取每位相加的和
pub fn sum(x: i32, y: i32, k: i32) -> i32 {
    // 获取数不应该转成字符串再转回来。
    // 应该通过取余获取
    let xs = x.to_string();
    let ys = y.to_string();
    let a = xs
        .chars()
        .fold(0, |mut a, x| x.to_string().parse::<i32>().unwrap());
    let b = ys
        .chars()
        .fold(a, |mut a, x| x.to_string().parse::<i32>().unwrap());
    b
}

// 如果当前坐标各数位总和小于k且周边存在通路就加一。使用一个二维数组标记通路。
pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let k = k as usize;
    let mut dv: Vec<Vec<usize>> = vec![vec![0; n]; m];
    dv[0][0] = 1;
    let mut ret = 0;

    for i in 0..m {
        for j in 0..n {
            let mut sum = 0;
            sum = sumself(i, j);
            if sum > k {
                continue;
            }
            if i > 0 && dv[i - 1][j] == 1
                || j > 0 && dv[i][j - 1] == 1
                || i < m - 1 && dv[i + 1][j] == 1
                || j < n - 1 && dv[i][j + 1] == 1
                || dv[i][j] == 1
            {
                ret += 1;
                dv[i][j] = 1;
            }
        }
    }
    return ret;
}

fn sumself(i: usize, j: usize) -> usize {
    i % 10 + (i / 10) % 10 + i / 100 + j % 10 + (j / 10) % 10 + j / 100
}
