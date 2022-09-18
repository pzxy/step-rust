

fn any_demo(){
    let s = (1..9).any(|x| x % 2 == 0);
    println!("{}",s)
}


#[test]
fn demo(){
    any_demo()
}