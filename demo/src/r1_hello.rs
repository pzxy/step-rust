pub fn hello() -> &'static str {
    "hello world"
}

#[test]
fn demo() {
    println!("{}", hello())
}
