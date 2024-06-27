fn main() {
    let mut a = vec![3; 5];
    for v in a.iter_mut() {
        *v = 2;
    }
    for v in a.iter() {
        println!("{:?}", v)
    }
}