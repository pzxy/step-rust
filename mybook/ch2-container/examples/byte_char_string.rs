// 一些基本类型操作和转换。
// 我们只讨论utf8编码。unicode并不是一个字节编码一个字符，更复杂一点。

// rust里面是非常直接的，u8的数组就是字节数组， 单引号''是char类型，双引号""是string类型。而单引号的数组，收集起来就是string。
// str 是“字符串切片类型的本体”，通常不直接使用。
// &str 是对 str 的不可变借用，用于只读视图（常见、轻量）。
// String 是拥有所有权、可变、可增长的堆分配字符串（构建与修改场景首选）。
// 它们通过借用与转换紧密关联：String 可借用为 &str；&str 可克隆为 String。
fn main() {
    // 1. byte类型,就是u8，他的[u8]其实就是string，可以互相转换
    // [u8]
    let bs: Vec<u8> = vec![97, 98, 99];
    // [u8]，byte类型转String类型。 from_utf8
    let s1 = String::from_utf8(bs); //to string
    println!("s1:{:?}", s1);

    // 2. char类型，char类型是4个字节。在go里面是rune，这里就是char
    let c2 = vec!['a', 'b', 'c', 'd']; //必须是单引号。
    println!("c2: {:?}", c2);
    let s2: String = c2.iter().collect(); // to string
    println!("s2: {:?}", s2);

    // 3. String
    let s3 = String::from("abc");
    let b3 = b"abc".to_vec(); // to vec[u8]
    println!("b3: {:?}", b3);
    let b3 = s3.as_bytes(); // to vec[u8] slice，rust中只要是切片都是引用。如果变为vec[u8]的话，使用to_vec()
    println!("b3: {:?}", b3);
    let c3: Vec<char> = s3.chars().collect(); // to vec[char]
    println!("c3: {:?}", c3);

    // 4. str 是 String的切片类型，切片都是引用，所以我们一般用的最多的就是&str
    let str = "abc";
    let s4 = str.to_owned(); //一般用to_owned(),to_string()底层也是调用的to_owned()
    println!("s4:{:?}", s4);
    let c4: Vec<char> = str.chars().collect(); // to vec[char]
    println!("c4:{:?}", c4);
    let b4 = str.as_bytes(); //  str.as_bytes().to_vec()
    println!("b4:{:?}", b4);

    // ========== as_* 方法 vs into() 的区别 ==========
    let s = String::from("hello");
    
    // 1. as_* 方法：返回引用，不消耗所有权（借用）
    //    - as_bytes() -> &[u8]：获取字节切片的引用
    //    - as_str() -> &str：获取字符串切片的引用
    //    特点：零成本转换，原值仍可使用
    let bytes_ref: &[u8] = s.as_bytes(); // 返回引用
    let str_ref: &str = s.as_str(); // 返回引用
    println!("as_bytes: {:?}, as_str: {}", bytes_ref, str_ref);
    println!("s 仍然可用: {}", s); // ✅ 可以继续使用
    
    // 2. into() 方法：消耗所有权，类型转换
    //    特点：所有权转移，原值不能再使用
    //    示例：&str -> String（通过 Into trait）
    let s2 = "world";
    let s3: String = s2.into(); // &str -> String，消耗 s2（但 &str 是 Copy，所以仍可用）
    println!("into(): {}", s3);
    println!("s2 仍可用: {}", s2); // ✅ &str 是 Copy
    
    // 3. 为什么用 as_* 而不是 into？
    //    - as_bytes() 返回 &[u8]，是"视图"转换，不涉及所有权
    //    - 如果要用 into()，需要 String -> Vec<u8>，这会消耗所有权
    //    - as_* 提供零成本的引用视图，into() 提供所有权转移
    
    // 4. 对比示例：String 转 Vec<u8>
    let s4 = String::from("test");
    
    // 方式1：as_bytes() - 返回引用，不消耗所有权
    let bytes_view = s4.as_bytes(); // &[u8]
    println!("as_bytes(): {:?}", bytes_view);
    println!("s4 仍可用: {}", s4); // ✅
    
    // 方式2：into_bytes() - 消耗所有权，获取 Vec<u8>
    let s5 = String::from("test2");
    let bytes_owned: Vec<u8> = s5.into_bytes(); // 消耗所有权
    println!("into_bytes(): {:?}", bytes_owned);
    // println!("s5: {}", s5); // ❌ 编译错误：value moved
    
    // 5. 总结：
    //    - as_* 方法：返回引用，借用视图，不消耗所有权
    //    - into() 方法：所有权转移，消耗原值
    //    - 选择原则：
    //      * 只需要读取数据 → 用 as_*（如 as_bytes(), as_str()）
    //      * 需要所有权转移 → 用 into() 或 into_*()（如 into_bytes()）
    
    println!("\n========== 其他转换方法 ==========\n");
    
    // 小总结：
    //     1. to开头的方法一般都是&str转String, &[u8]转[u8]这样的,引用转本体类型。
    //     2. 个体与数组整体类型，比如char和string，一般都是collect()，中间用bytes作为中间类型。
    //     3. as_开头的方法，基本返回的都是引用,但不一定是相同类型。
    //     4. into() 用于所有权转移的类型转换，会消耗原值。

    // 5. *const u8 和 *mut u8 ，这两个前面的是原始指针，后面的可变指针。
    // 一般不直接使用这两个，都是使用的智能指针。除非特殊情况。
    let s5 = String::from("abc");
    let s5_ptr = s5.as_ptr();
    println!("s5ptr:{:?}", s5_ptr)
}


// 2025-11-28 总结：
// 1. as一般都是强转或者返回引用类型这样的。
// 2. into都是转移所有权，其中可能涉及类型推断这些操作。有的into并不会转移所有权，那是因为触发了Copy trait。
