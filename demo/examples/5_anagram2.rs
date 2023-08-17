use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let low = word.to_lowercase();
    let mut low_s = low.chars().collect::<Vec<_>>();
    low_s.sort_unstable();
    let r = possible_anagrams
        .iter()
        .filter(|p| {
            let low_p = p.to_lowercase();
            let mut cs = low_p.chars().collect::<Vec<_>>();
            cs.sort();
            cs == low_s && low != low_p
        })
        .copied();
    HashSet::from_iter(r)
}

pub fn anagrams_for2<'a>(word: &'a str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let low = word.to_lowercase();
    let mut low_s = low.chars().collect::<Vec<_>>();
    low_s.sort();
    let r = possible_anagrams
        .iter()
        .filter(|p| {
            let low_p = p.to_lowercase();
            let mut cs = low_p.chars().collect::<Vec<_>>();
            cs.sort();
            cs == low_s && low != low_p
        })
        .copied();
    HashSet::from_iter(r)
}

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagrams_for(word, inputs);
    let expected: HashSet<&str> = expected.iter().cloned().collect();
    assert_eq!(result, expected);
}

fn main() {
    let word = "ΑΒΓ";
    // These words don't make sense, they're just greek letters cobbled together.
    let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
    let outputs = vec!["ΒΓΑ", "γβα"];
    process_anagram_case(word, &inputs, &outputs);
}
