fn main() {
    println!("=== 为什么自定义类型通常不实现 Copy？===\n");
    
    println!("【问题 1：包含锁的类型】");
    demonstrate_mutex_problem();
    
    println!("\n【问题 2：包含共享资源的类型】");
    demonstrate_shared_resource();
    
    println!("\n【问题 3：大对象复制成本高】");
    demonstrate_large_object();
    
    println!("\n【什么时候可以实现 Copy？】");
    demonstrate_when_copy_is_ok();
    
    println!("\n【正确的做法】");
    demonstrate_correct_approach();
    
    println!("\n【总结】");
    println!("✅ 你的理解完全正确！");
    println!("✅ 自定义类型通常不实现 Copy");
    println!("✅ 只有简单、小的、不包含共享资源的类型才适合实现 Copy");
    println!("✅ 包含锁、文件句柄、网络连接等资源的类型不应该实现 Copy");
    println!("✅ 大对象也不应该实现 Copy（复制成本高）");
}

use std::sync::Mutex;

fn demonstrate_mutex_problem() {
    println!("假设 Mutex 实现了 Copy（实际上没有）：");
    println!("```rust");
    println!("let mutex = Mutex::new(42);");
    println!("let mutex2 = mutex;  // 如果 Copy，会复制");
    println!("// 问题：现在有两个 Mutex，但保护的是同一个数据？");
    println!("// 这会导致数据竞争和未定义行为！");
    println!("```");
    
    // 实际演示：Mutex 没有实现 Copy
    let _mutex = Mutex::new(42);
    // let mutex2 = mutex;  // 如果这行取消注释，mutex 会被移动
    // 如果 Mutex 实现了 Copy，会有两个 Mutex 保护同一个数据，这是错误的！
    
    println!("✅ Mutex 没有实现 Copy，这是正确的设计");
}

fn demonstrate_shared_resource() {
    println!("包含文件句柄、网络连接等资源的类型：");
    println!("```rust");
    println!("struct FileHandle {{");
    println!("    fd: i32,  // 文件描述符");
    println!("}}");
    println!("");
    println!("// 如果实现 Copy：");
    println!("let file1 = FileHandle {{ fd: 1 }};");
    println!("let file2 = file1;  // 复制");
    println!("// 问题：两个 FileHandle 指向同一个文件");
    println!("// 关闭 file1 时，file2 就失效了！");
    println!("```");
    
    println!("✅ 包含资源的类型不应该实现 Copy");
    println!("✅ 应该使用移动语义，确保只有一个所有者");
}

fn demonstrate_large_object() {
    println!("大对象复制成本高：");
    println!("```rust");
    println!("struct LargeData {{");
    println!("    data: [u8; 1024 * 1024],  // 1MB 数据");
    println!("}}");
    println!("");
    println!("// 如果实现 Copy：");
    println!("let large1 = LargeData {{ ... }};");
    println!("let large2 = large1;  // 复制 1MB 数据！");
    println!("// 问题：每次赋值都复制 1MB，性能很差");
    println!("```");
    
    println!("✅ 大对象不应该实现 Copy");
    println!("✅ 应该使用移动语义或引用");
}

fn demonstrate_when_copy_is_ok() {
    println!("【适合实现 Copy 的类型】");
    
    // 示例：简单的坐标点
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 10, y: 20 };
    let _p2 = p1;  // Copy，没问题
    println!("Point {{ x: {}, y: {} }} 实现 Copy 是合理的", p1.x, p1.y);
    println!("原因：");
    println!("  ✅ 只包含基本类型（i32 实现了 Copy）");
    println!("  ✅ 数据很小（只有两个整数）");
    println!("  ✅ 不包含共享资源");
    println!("  ✅ 复制成本低");
    
    // 示例：简单的配置
    #[derive(Debug, Clone, Copy)]
    struct Config {
        timeout: u64,
        retries: u8,
    }
    
    let config = Config { timeout: 1000, retries: 3 };
    let _config2 = config;  // Copy，没问题
    println!("\nConfig 实现 Copy 也是合理的");
    
    println!("\n【不适合实现 Copy 的类型】");
    
    // String 没有实现 Copy（正确！）
    let s1 = String::from("hello");
    let _s2 = s1;  // 移动，不是复制
    // println!("{}", s1);  // 编译错误
    println!("String 没有实现 Copy ✅");
    println!("原因：包含堆分配的内存，复制成本高");
    
    // Vec 没有实现 Copy（正确！）
    let v1 = vec![1, 2, 3];
    let _v2 = v1;  // 移动，不是复制
    // println!("{:?}", v1);  // 编译错误
    println!("Vec 没有实现 Copy ✅");
    println!("原因：包含堆分配的内存，复制成本高");
    
    // Mutex 没有实现 Copy（正确！）
    let _m1 = Mutex::new(42);
    // let m2 = m1;  // 如果取消注释，m1 会被移动
    println!("Mutex 没有实现 Copy ✅");
    println!("原因：包含锁，复制会导致多个锁保护同一个数据");
}

// 演示：如果错误地实现了 Copy 会怎样
#[derive(Debug)]
struct BadCopyExample {
    id: i32,
    // 假设这里有一个文件句柄或网络连接
    // 如果实现 Copy，复制后会有两个对象指向同一个资源
}

// 这个类型不应该实现 Copy！
// impl Copy for BadCopyExample {}  // ❌ 错误！

// 正确的做法：不实现 Copy，使用移动语义
fn demonstrate_correct_approach() {
    println!("\n【正确的做法】");
    
    #[derive(Debug)]
    struct Resource {
        id: i32,
    }
    
    impl Resource {
        fn new(id: i32) -> Self {
            Resource { id }
        }
    }
    
    // 不实现 Copy，使用移动语义
    let r1 = Resource::new(1);
    let _r2 = r1;  // 移动，r1 不能再使用
    // println!("{:?}", r1);  // 编译错误
    println!("Resource 没有实现 Copy，使用移动语义 ✅");
    
    // 如果需要多个引用，使用 Rc 或 Arc
    use std::rc::Rc;
    let shared = Rc::new(Resource::new(1));
    let _shared2 = shared.clone();  // 增加引用计数，不是复制资源本身
    println!("使用 Rc 共享资源 ✅");
}

