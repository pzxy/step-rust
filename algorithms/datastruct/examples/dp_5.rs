use std::cmp::{max, min};

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let sum_trap = trap(height);
    println!("{}", sum_trap);
}

// https://leetcode.cn/problems/trapping-rain-water/description/
//输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
// 输出：6
// 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
// 示例 2：
//
// 输入：height = [4,2,0,3,2,5]
// 输出：9

pub fn trap(height: Vec<i32>) -> i32 {
    let mut left_max_height = vec![0; height.len()];
    let mut right_max_height = vec![0; height.len()];
    let length = height.len();

    let mut tmp_max = 0;
    for i in 0..length{
        tmp_max = max(tmp_max, height[i]);
        left_max_height[i] = tmp_max;
    }

    let mut tmp_max = 0;
    for i in 0..length{
        tmp_max = max(tmp_max, height[length-i-1]);
        right_max_height[length-i-1] = tmp_max;
    }

    let mut ret = 0;
    for i in 0..length{
        let min_value = min(left_max_height[i], right_max_height[i]);
        if min_value > height[i] {
            ret += min_value - height[i];
        }
    }
    return ret;
}