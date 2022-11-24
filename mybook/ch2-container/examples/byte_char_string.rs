// 一些基本类型操作和转换。
// 我们只讨论utf8编码。unicode并不是一个字节编码一个字符，更复杂一点。

fn main() {
    // 1. byte类型,就是u8，他的[u8]其实就是string，可以互相转换
    // [u8]
    let bs: Vec<u8> = vec![97, 98, 99];
    // [u8]，byte类型转String类型。 from_utf8
    let s1 = String::from_utf8(bs); //to string
    println!("s1:{:?}", s1);

    // 2. char类型，char类型是4个字节。在go里面是rune，这里就是char
    let c2: Vec<char> = vec!['a', 'b', 'c', 'd']; //必须是单引号。
    println!("c2: {:?}", c2);
    let s2: String = c2.iter().collect(); // to string
    println!("s2: {:?}", s2);

    // 3. String
    let s3 = String::from("abc");
    let b3 = b"#abc#".to_vec(); // to vec[u8]
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

    // 小总结：
    //     1. to开头的方法一般都是&str转String, &[u8]转[u8]这样的,引用转本体类型。
    //     2. 个体与数组整体类型，比如char和string，一般都是collect()，中间用bytes作为中间类型。
    //     3. as_开头的方法，基本返回的都是引用,但不一定是相同类型。

    // 5. *const u8 和 *mut u8 ，这两个前面的是原始指针，后面的可变指针。
    // 一般不直接使用这两个，都是使用的智能指针。除非特殊情况。
    let s5 = String::from("abc");
    let s5_ptr = s5.as_ptr();
    println!("s5ptr:{:?}", s5_ptr)
}
