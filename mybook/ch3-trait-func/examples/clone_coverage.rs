fn main() {
    println!("=== 哪些类型实现了 Clone？===\n");
    
    println!("【✅ 实现了 Clone 的类型】\n");
    
    // 1. 基本类型（因为它们实现了 Copy）
    println!("1. 基本类型：");
    let x: i32 = 42;
    let _y = x.clone();  // ✅
    println!("   i32, i64, u32, u64, f32, f64, bool, char 等都实现了 Clone");
    
    // 2. String
    println!("\n2. String：");
    let s = String::from("hello");
    let _s2 = s.clone();  // ✅
    println!("   String 实现了 Clone（深拷贝）");
    
    // 3. Vec
    println!("\n3. Vec<T>（如果 T 实现了 Clone）：");
    let v = vec![1, 2, 3];
    let _v2 = v.clone();  // ✅
    println!("   Vec<i32> 实现了 Clone");
    
    // 4. 数组（如果元素实现了 Clone）
    println!("\n4. 数组（如果元素实现了 Clone）：");
    let arr = [1, 2, 3];
    let _arr2 = arr.clone();  // ✅
    println!("   [i32; 3] 实现了 Clone");
    
    // 5. 元组（如果所有元素都实现了 Clone）
    println!("\n5. 元组（如果所有元素都实现了 Clone）：");
    let tup = (1, "hello".to_string(), true);
    let _tup2 = tup.clone();  // ✅
    println!("   (i32, String, bool) 实现了 Clone");
    
    // 6. Option<T>（如果 T 实现了 Clone）
    println!("\n6. Option<T>（如果 T 实现了 Clone）：");
    let opt = Some(42);
    let _opt2 = opt.clone();  // ✅
    println!("   Option<i32> 实现了 Clone");
    
    // 7. Result<T, E>（如果 T 和 E 都实现了 Clone）
    println!("\n7. Result<T, E>（如果 T 和 E 都实现了 Clone）：");
    let res: Result<i32, String> = Ok(42);
    let _res2 = res.clone();  // ✅
    println!("   Result<i32, String> 实现了 Clone");
    
    println!("\n【❌ 没有实现 Clone 的类型】\n");
    
    // 1. 函数指针
    println!("1. 函数指针：");
    let _func: fn() = my_function;
    // let _func2 = func.clone();  // ❌ 编译错误！
    println!("   函数指针没有实现 Clone");
    
    // 2. 某些闭包
    println!("\n2. 某些闭包：");
    let x = 42;
    let _closure = move || x;  // move 闭包
    // let _closure2 = closure.clone();  // ❌ 编译错误！
    println!("   某些闭包没有实现 Clone（除非所有捕获的变量都实现了 Clone）");
    
    // 3. 自定义类型（默认不实现）
    println!("\n3. 自定义类型（默认不实现）：");
    let _p = Point { x: 10, y: 20 };
    // let _p2 = p.clone();  // ❌ 编译错误！
    println!("   自定义类型默认不实现 Clone，需要手动实现");
    
    // 4. 某些智能指针（取决于内部类型）
    println!("\n4. 某些智能指针：");
    use std::sync::Mutex;
    let _mutex = Mutex::new(42);
    // let _mutex2 = mutex.clone();  // ❌ Mutex 没有实现 Clone
    println!("   Mutex<T> 没有实现 Clone（线程安全考虑）");
    
    println!("\n【如何让自定义类型实现 Clone？】\n");
    
    println!("方法 1：使用 derive 宏（推荐）");
    let p1 = PointWithClone { x: 1, y: 2 };
    let p2 = p1.clone();  // ✅
    println!("   #[derive(Clone)] struct PointWithClone {{ ... }}");
    println!("   p1: {:?}, p2: {:?}", p1, p2);
    
    println!("\n方法 2：手动实现");
    let p3 = PointManualClone { x: 3, y: 4 };
    let p4 = p3.clone();  // ✅
    println!("   impl Clone for PointManualClone {{ ... }}");
    println!("   p3: {:?}, p4: {:?}", p3, p4);
    
    println!("\n【总结】");
    println!("❌ Clone 不是所有类型都默认实现的");
    println!("✅ 基本类型、String、Vec 等标准库类型实现了 Clone");
    println!("✅ 如果类型的所有字段都实现了 Clone，可以使用 #[derive(Clone)]");
    println!("❌ 函数指针、某些闭包、自定义类型默认不实现 Clone");
}

fn my_function() {
    println!("Hello");
}

// 自定义类型：默认不实现 Clone
struct Point {
    x: i32,
    y: i32,
}

// 方法 1：使用 derive 宏（推荐）
#[derive(Debug, Clone)]
struct PointWithClone {
    x: i32,
    y: i32,
}

// 方法 2：手动实现
#[derive(Debug)]
struct PointManualClone {
    x: i32,
    y: i32,
}

impl Clone for PointManualClone {
    fn clone(&self) -> Self {
        PointManualClone {
            x: self.x,
            y: self.y,
        }
    }
}

