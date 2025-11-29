// 自定义类型实现 Copy 和 Clone

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 尝试只实现 Copy，不实现 Clone
// impl Copy for Point {}  // ❌ 编译错误！Copy 要求类型必须实现 Clone

// 正确的方式：先实现 Clone，再实现 Copy
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

// 现在可以实现 Copy 了
impl Copy for Point {}

fn main() {
    println!("=== Copy 为什么需要 Clone？===\n");
    
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;  // Copy（隐式）
    let p3 = p1.clone();  // Clone（显式）
    
    println!("p1: {:?}", p1);  // p1 仍然可用（Copy）
    println!("p2: {:?}", p2);  // p2 是 Copy 的结果
    println!("p3: {:?}", p3);  // p3 是 Clone 的结果
    
    println!("\n【原因 1：Copy 是 Clone 的特化】");
    println!("Copy 是'廉价'的 Clone，是 Clone 的一个特例");
    println!("Copy 表示：这个类型的 Clone 操作就是简单的位复制（bitwise copy）");
    
    println!("\n【原因 2：类型系统的一致性】");
    println!("如果一个类型是 Copy，那么它必须能够被复制");
    println!("Clone trait 定义了复制的能力，所以 Copy 类型必须实现 Clone");
    
    println!("\n【原因 3：实际使用场景】");
    println!("虽然 Copy 是隐式的，但有时也需要显式调用 clone()");
    let p4 = Point { x: 1, y: 2 };
    let p5 = p4.clone();  // 可以显式调用
    println!("p4: {:?}, p5: {:?}", p4, p5);
    
    println!("\n【使用 derive 宏（推荐方式）】");
    demonstrate_derive();
}

#[derive(Debug, Clone, Copy)]  // 使用 derive 宏自动实现
struct Point2 {
    x: i32,
    y: i32,
}

fn demonstrate_derive() {
    let p1 = Point2 { x: 100, y: 200 };
    let p2 = p1;  // Copy
    let p3 = p1.clone();  // Clone
    
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p3: {:?}", p3);
    
    println!("\n✅ 使用 #[derive(Clone, Copy)] 可以自动实现两个 trait");
    println!("✅ 编译器会生成合适的实现代码");
}

