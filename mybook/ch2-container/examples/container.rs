// enum vec slice 结构体，元组，hashmap,BHashmap等使用遍历操作的掌握。

// use std::collections::{BTreeMap, HashMap};

use std::collections::{HashMap, BTreeMap};

fn main() {
    tuple();
    array();
    slice();
    hashmap();
}

// tuple 元组
fn tuple() {
    // 可以将不同类型数据放到一起
    let s: (i32, f64, u8) = (500, 6.4, 1);
    // 获取tup元素，访问tup元素
    let (x, y, _) = s;
    println!("tuple: {} ,{} ,{}", x, y, s.2); // 通过.index获取
}

// 数组，将数据放在栈上，而不是堆上
fn array() {
    let a = ["1", "2", "3"]; // 自动判断类型
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // 声明类型
    let c = [3; 5]; // [3,3,3,3,3]
    println!("array: a:{:?},b:{:?},c:{:?}", a, b, c); // 访问越界，要注意,编译器会检查出来
}

// 切片,
fn slice() {
    // 自动扩容切片 vector 
    let mut v = vec![1, 2, 3, 4];
    let mut v2 = vec![5, 6, 7, 8];
    v.append(&mut v2);
    let v = &v[0..3];// 1,2,3
    println!("slice: {:?}", v);
}

fn hashmap() {
    // 原生hashmap可以防止dos攻击，也就是当相同hash值的攻击有一定防御作用，但是性能不是最好。
    let mut m = HashMap::new();
    m.insert("name", "pzxy");
    for (k, v) in m.iter() {
        println!("hashMap: k:{:?},b:{:?}", k, v);
    }

    let mut tm = BTreeMap::new();
    tm.insert(1, "a");
    tm.insert(4, "b");
    tm.insert(2, "c");
    tm.insert(9, "d");
    for (k, v) in tm.range(0..6) {// 限定key的大小来打印
        // 打印指定范围数据。
        println!("k:{},v:{}", k, v)
    }
}

// 枚举，将不同类型，抽象成相同的枚举类型。
enum _EE {
    Int(i32),
    Float(f64),
    Text(String),
}

// 总结：
// 1. 元组可以将不同类型的值放到一起。go中函数可以返回多个不同的值是不是也是元组呢？
// 2. vector是切片，可以动态扩容，他增加值是push，因为他相当于一个栈结构。而hashmap增加值是insert()
// 3. 枚举可以将不同类型的值抽象成一个类型。
// 4. hashMap 安全性能略差，BTreeMap 性能好。
// 5. 
