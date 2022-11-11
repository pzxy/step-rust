// iter 返回不可变引用&T
// iter_mut 返回可变引用
// into_iter 返回值本身，其实是复制了一份。
#[test]
fn iter_() {
    // 1.  map 里面一个匿名函数
    let a = [1, 2, 3, 4];
    // let b = &[5, 6, 7, 8];
    let m: Vec<i32> = a.iter().map(|x| x + 1).collect();
    println!("map :{:?}", m);

    // 2. zip返回的是一个是一个元组
    // zip https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5];
    let mut iter = s1.iter().zip(s2);
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    // 两个迭代长度必须一样。
    assert_eq!(iter.next(), None);

    // 3. skip 跳过2个值，copied()相当于map(x|x)
    let skip3 = a.iter().skip(2).copied().collect::<Vec<i32>>();
    println!("skip: {:?}", skip3);

    // 4.nth，获取迭代器第三个值。Option类型
    let nth4 = a.into_iter().nth(3);
    println!("nth: {:?}", nth4);

    // 5. filter 过滤某个值
    let fil: Vec<i32> = a.iter().filter(|&x| x % 2 == 0).copied().collect();
    println!("filter: {:?}", fil);

    // 6. window 不是作用于迭代器，而是作用于 切片 vector
    // 并不是将数据切分的chunk。而是一个滑动窗口，看打印就知道了。
    let arrays = a.windows(2).collect::<Vec<&[i32]>>();
    for arr in arrays {
        println!("windows: {:?}", arr)
    }

    // 7. fold

    // 8. take
}

// 小总结：
// 1. collect不能作用与迭代器类型。如果发现报错，用map或者copied方法转换一下。
// 2. zip的调用的双方必须是引用
// 3. skip和filter都必须转化一下才能collect。要经常使用就记住了，目前还没什么技巧。
