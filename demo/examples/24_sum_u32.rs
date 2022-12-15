fn main() {
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let s = sum_u32(&a);
    println!("{:?}", s)
}

fn sum_u32(a: &[u32]) -> Option<u32> {
    let s: u32 = a.iter().sum();
    Some(s)
}

