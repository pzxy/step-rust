use ch4_common::signal_demo;

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    let input = std::env::args().nth(1);
    println!("input:{:?}", input);
    signal_demo()
}
