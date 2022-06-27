// 在一个长度为 n 的数组 nums 里的所有数字都在 0～n-1 的范围内。
// 数组中某些数字是重复的，但不知道有几个数字重复了，也不知道每个数字重复了几次。请找出数组中任意一个重复的数字。
// 输入：
// [2, 3, 1, 0, 2, 5, 3]
// 输出：2 或 3


pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    let mut idx: usize = 0;
    while idx < nums.len() {
        if nums[idx] as usize == idx {
            idx += 1;
            continue
        }
        if nums[idx] == nums[nums[idx] as usize] {
            return nums[idx]
        } else {
            let t = nums[idx] as usize;
            nums.swap(idx, t);
        }
    }
    idx as i32
}