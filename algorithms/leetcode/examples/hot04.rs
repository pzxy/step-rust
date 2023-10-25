//283. 移动零
// 提示
// 简单
// 2.2K
// 相关企业
// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
//
// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
//
//
//
// 示例 1:
//
// 输入: nums = [0,1,0,3,12]
// 输出: [1,3,12,0,0]
// 示例 2:
//
// 输入: nums = [0]
// 输出: [0]
//
//
// 提示:
//
// 1 <= nums.length <= 104
// -231 <= nums[i] <= 231 - 1

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes2(&mut nums);
    println!("{:?}", nums);
}


pub fn move_zeroes(nums: &mut Vec<i32>) {
    let len = nums.len();
    nums.retain(|x| *x != 0);
    println!("{:?}", nums);
    nums.resize(len, 0);
}

pub fn move_zeroes2(nums: &mut Vec<i32>) {
    let len = nums.len();
    let (mut l, mut r) = (0, 0);
    while r < len {
        if nums[r] != 0 {
            nums[l] = nums[r];
            l += 1;
        }
        r += 1;
    }
    for i in l..len {
        nums[i] = 0;
    }
}


pub fn move_zeroes3(nums: &mut Vec<i32>) {
    let mut l = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[l] = nums[i];
            l += 1;
        }
    }
    for i in l..nums.len() {
        nums[i] = 0;
    }
}