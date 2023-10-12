//给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。
//
//
//
// 示例 1:
//
// 输入: nums = [1,2,3,4,5,6,7], k = 3
// 输出: [5,6,7,1,2,3,4]
// 解释:
// 向右轮转 1 步: [7,1,2,3,4,5,6]
// 向右轮转 2 步: [6,7,1,2,3,4,5]
// 向右轮转 3 步: [5,6,7,1,2,3,4]
// 示例 2:
//
// 输入：nums = [-1,-100,3,99], k = 2
// 输出：[3,99,-1,-100]
// 解释:
// 向右轮转 1 步: [99,-1,-100,3]
// 向右轮转 2 步: [3,99,-1,-100]
//
//
// 提示：
//
// 1 <= nums.length <= 105
// -231 <= nums[i] <= 231 - 1
// 0 <= k <= 105
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate2(&mut nums, k);
    println!("{:?}", nums);

    let mut nums = vec![-1, -100, 3, 99];
    let k = 2;
    rotate2(&mut nums, k);
    println!("{:?}", nums)
}

// 关键 (i+k)%n 就是下标，这个要搞清楚。
// 只适用于k是奇数，好像不仅仅如此,这个不太对。
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize;
    let mut idx = 0;
    let mut pre_value = nums[0];
    let mut tmp_value = 0;
    for _ in 0..nums.len() {
        idx = (idx + k) % nums.len();
        tmp_value = nums[idx];
        nums[idx] = pre_value;
        pre_value = tmp_value;
    }
}
// 想想成环 todo
pub fn rotate3(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    if nums.len() <= 1 || k == 0 {
        return;
    }
    let mut start_idx = 0usize;
    let mut idx = start_idx;
    let mut tmp = nums[idx];
    for _ in 0..nums.len() {
        idx = (idx + k) % nums.len();
        let new_tmp = nums[idx];
        nums[idx] = tmp;
        tmp = new_tmp;
        if idx == start_idx {
            start_idx += 1;
            idx = start_idx;
            tmp = nums[idx];
        }
    }
}


pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
    let mut tmp = vec![0; nums.len()];
    for i in 0..nums.len() {
        tmp[(i + k as usize) % nums.len()] = nums[i]
    }
    for i in 0..nums.len() {
        nums[i] = tmp[i]
    }
}