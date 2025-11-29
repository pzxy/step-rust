// 演示为什么 Copy 需要 Clone

fn main() {
    println!("=== 为什么 Copy 需要 Clone？===\n");
    
    println!("【原因 1：Copy 是 Clone 的子 trait（subtrait）】");
    println!("在 Rust 标准库中，Copy trait 的定义是：");
    println!("```rust");
    println!("pub trait Copy: Clone {{");
    println!("    // Empty");
    println!("}}");
    println!("```");
    println!("这意味着：Copy 继承自 Clone，所以实现 Copy 必须先实现 Clone");
    
    println!("\n【原因 2：Copy 是'廉价'的 Clone】");
    println!("Copy 表示：这个类型的 Clone 操作就是简单的位复制（bitwise copy）");
    println!("所以 Copy 类型必须能够被 Clone，只是 Clone 的实现很简单");
    
    println!("\n【原因 3：类型系统的一致性】");
    println!("如果一个类型是 Copy，那么它必须能够被复制");
    println!("Clone trait 定义了复制的能力，所以 Copy 类型必须实现 Clone");
    
    println!("\n【实际演示】");
    demonstrate_copy_clone();
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 错误示例：只实现 Copy，不实现 Clone
// impl Copy for Point {}  // ❌ 编译错误！
// 错误信息：the trait `Clone` is not implemented for `Point`

// 正确方式 1：手动实现
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,  // i32 是 Copy，自动复制
            y: self.y,  // i32 是 Copy，自动复制
        }
    }
}

impl Copy for Point {}  // ✅ 现在可以实现了

fn demonstrate_copy_clone() {
    let p1 = Point { x: 10, y: 20 };
    
    // Copy：隐式复制
    let p2 = p1;  // 自动 Copy
    println!("p1: {:?}", p1);  // p1 仍然可用
    println!("p2: {:?}", p2);
    
    // Clone：显式复制
    let p3 = p1.clone();  // 显式调用 clone()
    println!("p3: {:?}", p3);
    
    println!("\n✅ Copy 和 Clone 都可以使用");
    println!("✅ Copy 是隐式的，Clone 是显式的");
    println!("✅ 对于 Copy 类型，clone() 的实现就是简单的位复制");
    
    // 演示 derive 宏
    let p2 = Point2 { x: 100, y: 200 };
    let p2_clone = p2.clone();
    let p2_copy = p2;
    println!("\n使用 derive 宏：");
    println!("p2: {:?}, p2_clone: {:?}, p2_copy: {:?}", p2, p2_clone, p2_copy);
}

// 正确方式 2：使用 derive 宏（推荐）
#[derive(Debug, Clone, Copy)]  // 自动实现两个 trait
struct Point2 {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_copy_requires_clone() {
        // 这个测试验证了 Copy 类型必须实现 Clone
        let p = Point2 { x: 1, y: 2 };
        let p_clone = p.clone();  // Clone 可用
        let p_copy = p;  // Copy 可用
        assert_eq!(p.x, p_clone.x);
        assert_eq!(p.x, p_copy.x);
    }
}

