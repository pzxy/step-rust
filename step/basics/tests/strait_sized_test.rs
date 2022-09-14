use std::marker::Sized;

#[derive(Clone)]
struct Animal {
    name: String,
}

trait AnimalTrait {
    fn run(&self);
    fn new(&self) -> Self where Self: Sized;
}

impl AnimalTrait for Animal {
    fn run(&self) {
        println!("animal running")
    }

    fn new(&self) -> Self {
        self.clone()
    }
}

// 这里必须是
fn trait_sized1(a: &dyn AnimalTrait) {}

#[test]
fn trait_sized() {}