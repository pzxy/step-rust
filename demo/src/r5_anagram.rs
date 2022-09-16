//An anagram is a rearrangement of letters to form a new word. Given a word and a list of candidates, select the sublist of anagrams of the given word.
//
// Given "listen" and a list of candidates like "enlists" "google" "inlets" "banana" the program should return a list containing "inlets".
//
// The solution is case insensitive, which means "WOrd" is the same as "word" or "woRd". It may help to take a peek at the std library for functions that can convert between them.
//
// The solution cannot contain the input word. A word is always an anagram of itself, which means it is not an interesting result. Given "hello" and the list ["hello", "olleh"] the answer is ["olleh"].
//
// You are going to have to adjust the function signature provided in the stub in order for the lifetimes to work out properly. This is intentional: what's there demonstrates the basics of lifetime syntax, and what's missing teaches how to interpret lifetime-related compiler errors.
//
// Try to limit case changes. Case changes are expensive in terms of time, so it's faster to minimize them.
//
// If sorting, consider sort_unstable which is typically faster than stable sorting. When applicable, unstable sorting is preferred because it is generally faster than stable sorting and it doesn't allocate auxiliary memory.

use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut ret: HashSet<&str> = HashSet::new();
    let word_map = str2map(word);
    for s in possible_anagrams.iter() {
        if s.len() != word.len() {
            continue;
        }
        if s.clone().to_uppercase().eq(&word.to_uppercase()) {
            continue;
        }
        let curr_map = str2map(s);
        ret.insert(s);
        println!("insert:{}", s);
        // compare map
        for c in s.clone().to_uppercase().chars() {
            match word_map.get(&c) {
                None => {
                    ret.remove(s);
                    break;
                }
                Some(vv) => match curr_map.get(&c) {
                    None => {
                        ret.remove(s);
                        break;
                    }
                    Some(vvv) => {
                        if vv != vvv {
                            ret.remove(s);
                            break;
                        }
                    }
                },
            }
        }
    }
    ret
}

fn str2map(s: &str) -> HashMap<char, i32> {
    let mut ret: HashMap<char, i32> = HashMap::new();
    for v in s.to_uppercase().chars() {
        match ret.get(&v) {
            None => ret.insert(v, 1),
            Some(count) => ret.insert(v, count + 1),
        };
    }
    ret
}

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagrams_for(word, inputs);
    let expected: HashSet<&str> = expected.iter().cloned().collect();
    assert_eq!(result, expected);
}

#[test]
fn demo() {
    let word = "ΑΒΓ";
    // These words don't make sense, they're just greek letters cobbled together.
    let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
    let outputs = vec!["ΒΓΑ", "γβα"];
    process_anagram_case(word, &inputs, &outputs);
}
