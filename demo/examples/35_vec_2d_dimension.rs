const NAME_NUMBER: usize = 3;

#[derive(Debug)]
struct Name {
    data: Vec<u8>,
}

#[derive(Debug)]
struct User {
    name: [Name; NAME_NUMBER],
}

fn new_name(sz: usize) -> Name {
    Name {
        data: vec![0; sz]
    }
}

fn main() {
    let sz = 3;
    let c = User {
        name: std::array::from_fn(|_| new_name(sz)),
    };
    println!("{:?}", c);
}