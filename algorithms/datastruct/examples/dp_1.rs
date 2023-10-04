use std::cmp::max;

// 选出不相邻的数字的最大和 arr = [1,2,4,1,7,8,3]
fn main() {
    let arr = vec![1, 2, 4, 1, 7, 8, 3];
    let ret = dp_1(arr);
    println!("{}", ret);
}

fn dp_1(arr: Vec<i32>) -> i32 {
    let mut opt = vec![0; arr.len()];
    opt[0] = arr[0];
    opt[1] = max(arr[0], arr[1]);
    for i in 2..arr.len() {
        let a = opt[i - 2] + arr[i];
        let b = opt[i - 1];
        opt[i] = max(a, b);
    }
    opt[opt.len() - 1]
}
