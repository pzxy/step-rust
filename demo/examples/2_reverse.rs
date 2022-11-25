// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    println!("{}", reverse("a2b"))
}
