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
    // 买股票最佳时机1
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let ret = majority_element(nums);
    println!("{}", ret);

    let nums2 = vec![7, 1, 5, 3, 6, 4];
    let ret2 = max_profit13(nums2);
    println!("{}", ret2);

    // 买股票最佳时机2
    let nums3 = vec![6, 1, 3, 2, 4, 7];
    let ret3 = max_profit20(nums3);
    println!("{}", ret3);
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

pub fn max_profit12(prices: Vec<i32>) -> i32 {
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

pub fn max_profit13(prices: Vec<i32>) -> i32 {
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

//给你一个整数数组 prices ，其中 prices[i] 表示某支股票第 i 天的价格。
//
// 在每一天，你可以决定是否购买和/或出售股票。你在任何时候 最多 只能持有 一股 股票。你也可以先购买，然后在 同一天 出售。
//
// 返回 你能获得的 最大 利润 。
//
//
//
// 示例 1：
//
// 输入：prices = [7,1,5,3,6,4]
// 输出：7
// 解释：在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5 - 1 = 4 。
//      随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6 - 3 = 3 。
//      总利润为 4 + 3 = 7 。
// 示例 2：
//
// 输入：prices = [1,2,3,4,5]
// 输出：4
// 解释：在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5 - 1 = 4 。
//      总利润为 4 。
// 示例 3：
//
// 输入：prices = [7,6,4,3,1]
// 输出：0
// 解释：在这种情况下, 交易无法获得正利润，所以不参与交易可以获得最大利润，最大利润为 0 。
//

// 买股票最佳时机二
// [6,1,3,2,4,7]
pub fn max_profit20(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }
    let mut min = prices[0];
    let mut tmp = 0;
    let mut ret = 0;
    for i in 1..prices.len() {
        if prices[i - 1] > prices[i] {// 这一步是触发更新ret的操作，可能无法触发，所以要在else最后一次循环中触发。
            ret += tmp;
            min = prices[i];
            tmp = 0;
        } else {
            tmp = prices[i] - min;
            if i == prices.len() - 1 {
                ret += tmp;
            }
        }
        println!("tmp:{},min:{},ret:{}", tmp, min, ret);
    }
    return ret;
}

// 买股票最佳时机二
// [6,1,3,2,4,7]
// 贪心算法，有賺就卖。
pub fn max_profit21(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }
    let mut tmp = 0;
    let mut ret = 0;
    for i in 1..prices.len() {
        tmp = prices[i] - prices[i - 1];
        if tmp > 0 {// 这一步是触发更新ret的操作，可能无法触发，所以要在else最后一次循环中触发。
            ret += tmp;
        }
    }
    return ret;
}
