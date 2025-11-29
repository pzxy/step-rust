fn main() {
    println!("=== Clone 不是所有类型都默认实现的 ===\n");
    
    println!("【实现了 Clone 的类型】");
    
    // 基本类型都实现了 Clone（因为它们也实现了 Copy）
    let x = 42;
    let y = x.clone();  // ✅ i32 实现了 Clone
    println!("i32: x={}, y={}", x, y);
    
    let s = String::from("hello");
    let s2 = s.clone();  // ✅ String 实现了 Clone
    println!("String: s={}, s2={}", s, s2);
    
    let v = vec![1, 2, 3];
    let v2 = v.clone();  // ✅ Vec 实现了 Clone
    println!("Vec: v={:?}, v2={:?}", v, v2);
    
    println!("\n【没有实现 Clone 的类型】");
    
    // 函数指针没有实现 Clone
    let func: fn() = my_function;
    // let func2 = func.clone();  // ❌ 编译错误！函数指针没有实现 Clone
    
    // 闭包没有实现 Clone（除非捕获的变量都是 Clone）
    let x = 42;
    let closure = || println!("{}", x);
    // let closure2 = closure.clone();  // ❌ 编译错误！闭包默认没有实现 Clone
    
    // 某些智能指针没有实现 Clone
    use std::rc::Rc;
    let rc = Rc::new(42);
    let rc2 = rc.clone();  // ✅ Rc 实现了 Clone（但这是引用计数，不是深拷贝）
    println!("Rc: rc={}, rc2={}", rc, rc2);
    
    println!("\n【自定义类型：默认不实现 Clone】");
    
    // 自定义结构体默认不实现 Clone
    let p = Point { x: 10, y: 20 };
    // let p2 = p.clone();  // ❌ 编译错误！Point 没有实现 Clone
    
    // 需要手动实现或使用 derive
    let p2 = PointWithClone { x: 10, y: 20 };
    let p3 = p2.clone();  // ✅ 实现了 Clone
    println!("PointWithClone: p2={:?}, p3={:?}", p2, p3);
    
    println!("\n【总结】");
    println!("❌ Clone 不是所有类型都默认实现的");
    println!("✅ 基本类型（i32, bool, char 等）实现了 Clone");
    println!("✅ 标准库中的集合类型（String, Vec, HashMap 等）实现了 Clone");
    println!("❌ 自定义类型需要手动实现 Clone（或使用 #[derive(Clone)]）");
    println!("❌ 函数指针、某些闭包等没有实现 Clone");
}

fn my_function() {
    println!("Hello");
}

// 自定义类型：默认不实现 Clone
struct Point {
    x: i32,
    y: i32,
}

// 手动实现 Clone
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

// 或者使用 derive 宏（推荐）
#[derive(Debug, Clone)]
struct PointWithClone {
    x: i32,
    y: i32,
}

