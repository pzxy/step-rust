trait Graph {
    fn area(&self) -> u32;
}
impl Graph for Rectangle {
    fn area(&self) -> u32 {
        self.Long * self.Wide
    }
}
impl Graph for Square {
    fn area(&self) -> u32 {
        self.edge * self.edge
    }
}
struct Rectangle {
    Long: u32,
    Wide: u32,
}

struct Square {
    edge: u32,
}

struct Print<T: Graph> {
    graphs: Vec<T>,
}

impl<T> Print<T>
where
    T: Graph,
{
    pub fn print_area(&self) {
        for component in self.graphs.iter() {
            println!("{}", component.area())
        }
    }
}

fn main() {
    let r = Rectangle { Long: 2, Wide: 3 };
    let r2 = Rectangle { Long: 1, Wide: 3 };
    let r3 = Rectangle { Long: 4, Wide: 3 };
    let p = Print {
        graphs: vec![r, r2, r3],
    };
    p.print_area();

    let s = Square { edge: 2 };
    let s2 = Square { edge: 3 };
    let s3 = Square { edge: 4 };
    let p = Print {
        graphs: vec![s, s2, s3],
    };
    p.print_area()
}
