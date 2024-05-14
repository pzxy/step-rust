use std::borrow::Cow;

fn process_string(s: Cow<str>) {
    if s.len() > 10 {
        let owned_string: String = s.into_owned();
        println!("Received owned string: {}", owned_string);
        // 这里可以修改 owned_string
    } else {
        println!("Received borrowed string: {}", s);
        // 这里不能修改 s
    }
}

fn main() {
    let mut borrowed_string: Cow<str> = Cow::Borrowed("Hello, Rust!");

    borrowed_string.to_mut().push_str(" world");
    println!("c:{}", borrowed_string);
    let mut owned_string: Cow<str> = Cow::Owned("Hello, Rust!".to_string());
    owned_string.to_mut().push_str(" world");
    println!("c:{}", owned_string);

    let mut c = Cow::from("hello");
    c.to_mut().push_str(" world");
    c.to_owned();
    println!("c:{}", c);
}