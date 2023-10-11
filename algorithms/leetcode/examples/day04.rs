//给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。
//
// 你可以假设数组是非空的，并且给定的数组总是存在多数元素。
//
//
//
// 示例 1：
//
// 输入：nums = [3,2,3]
// 输出：3
// 示例 2：
//
// 输入：nums = [2,2,1,1,1,2,2]
// 输出：2
//
//
// 提示：
// n == nums.length
// 1 <= n <= 5 * 104
// -109 <= nums[i] <= 109
//

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let ret = majority_element(nums);
    println!("{}", ret);

    let nums2 = vec![7, 1, 5, 3, 6, 4];
    let ret2 = max_profit3(nums2);
    println!("{}", ret2)
}

// 同归于尽法
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut count = 0;
    for i in 0..nums.len() {
        if count == 0 {
            ret = nums[i];
            count += 1;
            continue;
        }
        if nums[i] == ret {
            count += 1;
        } else {
            count -= 1
        }
    }
    return ret;
}


//给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。
//
// 你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。
//
// 返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。
//
//
//
// 示例 1：
//
// 输入：[7,1,5,3,6,4]
// 输出：5
// 解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
//      注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
// 示例 2：
//
// 输入：prices = [7,6,4,3,1]
// 输出：0
// 解释：在这种情况下, 没有交易完成, 所以最大利润为 0。
//

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut ret = 0;

    let _ = prices.iter().rev().map(|&x| {
        if x > max {
            max = x;
        } else {
            let tmp = max - x;
            if ret < tmp {
                ret = tmp
            }
        }
    }).collect::<Vec<_>>();
    return ret;
}

pub fn max_profit2(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut ret = 0;
    let length = prices.len() - 1;

    for i in 0..=length {
        if prices[length - i] > max {
            max = prices[length - i];
        } else {
            let tmp = max - prices[length - i];
            if ret < tmp {
                ret = tmp
            }
        }
    }
    return ret;
}

pub fn max_profit3(prices: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut ret = 0;
    for i in 0..prices.len() {
        if prices[i] < min {
            min = prices[i];
        } else if (prices[i] - min) > ret {
            ret = prices[i] - min;
        }
    }
    return ret;
}