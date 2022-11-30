use std::path::PathBuf;

fn main() {
    let mut dir = PathBuf::from("./a");
    dir.push("asd");
    let s = dir.into_os_string().into_string().unwrap();
    println!("{:?}",s)
}

