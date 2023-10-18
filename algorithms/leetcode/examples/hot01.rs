use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ret = two_sum(nums.clone(), target);
    assert_ne!(ret[0], ret[1]);
    assert_eq!(nums[ret[0] as usize] + nums[ret[1] as usize], target);
}

//给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//
// 你可以按任意顺序返回答案。
//
//
//
// 示例 1：
//
// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
// 示例 2：
//
// 输入：nums = [3,2,4], target = 6
// 输出：[1,2]
// 示例 3：
//
// 输入：nums = [3,3], target = 6
// 输出：[0,1]
//
//
// 提示：
//
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// 只会存在一个有效答案
//
//
// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        if m.contains_key(&(target - nums[i])) {
            return vec![*m.get(&(target - nums[i])).unwrap(), i as i32];
        } else {
            m.insert(nums[i], i as i32);
        }
    }
    unreachable!()
}