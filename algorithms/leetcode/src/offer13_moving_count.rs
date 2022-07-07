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

pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
    let mut count = 0;
    for x in 0..m {
        if x > k {
            break;
        }
        for y in 0..n {
            if y > k {
                break;
            }
            if (x + y) > k {
                continue;
            }
            if dfs(x, y, k) == true {
                count += 1
            }
        }
    }
    count
}


pub fn dfs(x: i32, y: i32, k: i32) -> bool {
    // 获取数不应该转成字符串再转回来。
    // 应该通过取余获取
    let xs = x.to_string();
    let ys = y.to_string();
    let a = xs.chars().fold(0, |mut a, x| {
        x.to_string().parse::<i32>().unwrap()
    });
    let b = ys.chars().fold(a, |mut a, x| {
        x.to_string().parse::<i32>().unwrap()
    });

    if b > k {
        false
    } else {
        true
    }
}

pub fn dfs2(x: i32, y: i32, k: i32) -> bool {
    let i = x % 10;
}



