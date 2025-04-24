// iter 返回不可变引用&T
// iter_mut 返回可变引用
// into_iter 返回值本身，其实是复制了一份。
fn main() {
    // 1.  map 里面一个匿名函数
    let a = [1, 2, 3, 4];
    // let b = &[5, 6, 7, 8];
    let m: Vec<i32> = a.iter().map(|x| x + 1).collect();
    println!("map :{:?}", m);

    // 2. zip返回的是一个是一个元组，可以将两个不同类型的数组/切片，合并成元组，以最短的数据长度为准。
    // zip https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
    let s1 = &[1, 2, 3];
    let s2 = &["hello", "world"];
    let mut iter = s1.iter().zip(s2);
    assert_eq!(iter.next(), Some((&1, &"hello")));// &1,&&hello
    assert_eq!(iter.next(), Some((&2, &"world")));
    // 两个迭代长度必须一样。
    assert_eq!(iter.next(), None);

    // 3. skip 跳过2个值，copied()相当于map(&x|x)或者map(|x|*x),为了返回
    let skip3 = a.iter().skip(2).copied().collect::<Vec<i32>>();
    println!("skip: {:?}", skip3);//[3,4]

    // 4.nth，获取迭代器第三个值。Option类型
    let nth4 = a.into_iter().nth(2);
    println!("nth: {:?}", nth4);//3

    // 5. filter 过滤某个值
    let fil: Vec<i32> = a.iter().filter(|&x| x % 2 == 0).copied().collect();
    println!("filter: {:?}", fil);

    // 6. window 不是作用于迭代器，而是作用于 切片 vector
    // 并不是将数据切分的chunk。而是一个滑动窗口，看打印就知道了。
    let arrays = a.windows(2).collect::<Vec<&[i32]>>();
    for arr in arrays {
        println!("windows: length:{:?},value:{:?}", arr.len(), arr)
    }

    // 7. fold
    // 它有两个参数，第一个init是初始值，第二个是一个闭包，
    // 闭包的第一个参数acc是上一次闭包函数的返回值(acc初始值就是init)，闭包第二参数是数组中值的引用。
    // 大概类似于这样：只不过fold支持的是泛型不仅仅i32类型。
    // let vec = vec![1, 2, 3, 4, 5];
    // let init = 0;
    // let mut ret = init;
    // fn f(acc: i32, x: i32) -> i32 {
    //     acc + x
    // }
    // for x in vec {
    //     ret = f(ret, x)
    // }
    // 上面的代码用fold写的话就是这样。
    let vec = vec![1, 2, 3, 4, 5];
    let res = vec.iter().fold(5, |acc, x| acc + x);// 5+1+2+3+4+5=20
    // 当我们要按照顺序操作迭代器中所有值时，并且下次操作依赖于上次操作的结果时，就可以用fold。
    println!("fold: {:?}", res);

    // 8.1 take
    let vec = vec![1, 2, 3, 4, 5];
    // take 从迭代器中取前面的2个值，相当于下面这个代码。
    // for &v in vec[..2].iter() {
    //     println!("take: {:?}", v);
    // }
    // 需要注意的是，取出来的值是引用类型。
    // 这里可以对比一下skip，skip是跳过前n个值，take是获取到n个值。那我们先skip再take岂不是能获取任意值了。
    for &v in vec.iter().take(2) {
        println!("take: {:?}", v);// 1 2
    }
    // 8.2 Option的take
    let mut x = Some(2);
    // Option的take 会取走值，然后设置一个None，相当于 std::mem::replace(self, None)
    let y = x.take();//类似c++中的move
    println!("Option take x: {:?}",x); //None
    println!("Option take y: {:?}",y); // Some(2)

    // 9. map_or, 提取Option中的值，使用f处理后返回。当为None时，返回这个默认值。
    let result_1 = x.map_or(0, |v| v * 2); // result_1 = 0
    let result_2 = y.map_or(0, |v| v * 2); // result_2 = 4
    println!("map_or: {:?}, {:?}", result_1, result_2);

    // 10. any，检查迭代器中是否有元素符合要求，如果符合就返回true
    let s = (1..9).any(|x| x % 2 == 0);
    println!("{}", s)
}

// 小总结：
// 1. collect不能作用与迭代器类型。如果发现报错，用map或者copied方法转换一下。
// 2. zip的调用的双方必须是引用
// 3. skip和filter都必须转化一下才能collect。要经常使用就记住了，目前还没什么技巧。
