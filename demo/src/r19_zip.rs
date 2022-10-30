use std::collections::HashMap;

#[test]
#[ignore]
fn test_zip() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores).collect();
    println!("zip {:?}", scores);
}
