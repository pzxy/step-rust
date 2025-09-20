// 给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
//
// 请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
//
// 注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。为了应对这种情况，nums1 的初始长度为 m + n，其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。
//
//
//
// 示例 1：
//
// 输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// 输出：[1,2,2,3,5,6]
// 解释：需要合并 [1,2,3] 和 [2,5,6] 。
// 合并结果是 [1,2,2,3,5,6] ，其中斜体加粗标注的为 nums1 中的元素。
// 示例 2：
//
// 输入：nums1 = [1], m = 1, nums2 = [], n = 0
// 输出：[1]
// 解释：需要合并 [1] 和 [] 。
// 合并结果是 [1] 。
// 示例 3：
//
// 输入：nums1 = [0], m = 0, nums2 = [1], n = 1
// 输出：[1]
// 解释：需要合并的数组是 [] 和 [1] 。
// 合并结果是 [1] 。
// 注意，因为 m = 0 ，所以 nums1 中没有元素。nums1 中仅存的 0 仅仅是为了确保合并结果可以顺利存放到 nums1 中。

fn main() {
    // 测试用例 1：正常合并
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("测试用例 1: {:?}", nums1);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

    // 测试用例 2：nums1 只有一个元素，nums2 为空
    let mut nums1 = vec![1];
    let mut nums2 = vec![];
    merge(&mut nums1, 1, &mut nums2, 0);
    println!("测试用例 2: {:?}", nums1);
    assert_eq!(nums1, vec![1]);

    // 测试用例 3：nums1 为空，nums2 有一个元素
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 0, &mut nums2, 1);
    println!("测试用例 3: {:?}", nums1);
    assert_eq!(nums1, vec![1]);

    println!("所有测试用例通过！");
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut p1 = m as usize;
    let mut p2 = n as usize;
    let mut p = (m + n) as usize;

    // 从后往前合并，避免使用额外空间
    while p1 > 0 && p2 > 0 {
        if nums1[p1 - 1] > nums2[p2 - 1] {
            nums1[p - 1] = nums1[p1 - 1];
            p1 -= 1;
        } else {
            nums1[p - 1] = nums2[p2 - 1];
            p2 -= 1;
        }
        p -= 1;
    }

    // 如果 nums2 还有剩余元素，复制到 nums1 的前面
    while p2 > 0 {
        nums1[p - 1] = nums2[p2 - 1];
        p2 -= 1;
        p -= 1;
    }
}
