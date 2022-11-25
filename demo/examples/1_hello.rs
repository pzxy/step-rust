use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // 这里之所以可以调用 to_string() ,是因为fmt::Display的做作用.
        // 如果去掉: fmt::Display,这里会报错.
        let output = self.to_string();
        println!("{}", output);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    Point{x:1,y:2}.outline_print()
}
