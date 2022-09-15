// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[test]
fn demo() {
    println!("{}", reverse("a2b"))
}
