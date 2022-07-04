//把一个数组最开始的若干个元素搬到数组的末尾，我们称之为数组的旋转。
//
// 给你一个可能存在重复元素值的数组numbers，它原来是一个升序排列的数组，并按上述情形进行了一次旋转。
// 请返回旋转数组的最小元素。例如，数组[3,4,5,1,2] 为 [1,2,3,4,5] 的一次旋转，该数组的最小值为 1。
//
// 注意，数组 [a[0], a[1], a[2], ..., a[n-1]] 旋转一次 的结果为数组 [a[n-1], a[0], a[1], a[2], ..., a[n-2]] 。
//
//
// 示例 1：
//
// 输入：numbers = [∑,1,2]
// 输出：1
// 示例 2：
//
// 输入：numbers = [2,2,2,0,1]
// 输出：0
//
// 提示：
//
// n == numbers.length
// 1 <= n <= 5000
// -5000 <= numbers[i] <= 5000
// numbers 原来是一个升序排序的数组，并进行了 1 至 n 次旋转
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/xuan-zhuan-shu-zu-de-zui-xiao-shu-zi-lcof
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

// 【3，1，3】
pub fn min_array(numbers: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        // 这样会超出时间限制，我也不知道问题在哪里。
        // let min = left + (right - left) >> 1;
        let min = (left + right) / 2;
        if numbers[min] > numbers[right] {
            left = min + 1
        } else if numbers[min] < numbers[right] {
            right = min
        } else {
            right -= 1
        }
    }
    return numbers[left];
}

pub fn min_array2(numbers: Vec<i32>) -> i32 {
    let mut p = 0;
    let mut q = numbers.len() - 1;

    while q > p {
        let m = (p + q) / 2;
        // m 一定在左边的数组中
        if numbers[m] > numbers[q] {
            p = m + 1;
            // m 一定落在右边的数组中
        } else if numbers[m] < numbers[q] {
            q = m;
            // 无法判断落在左边还是右边，缩小范围
        } else {
            q -= 1;
        }
    }
    return numbers[p];
}



