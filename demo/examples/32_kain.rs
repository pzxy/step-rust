fn main() {
    let tup = (1, "2", 3.0);
    let (x, _, z) = tup;
    println!("{x},{},{tup}", tup.1);
}
