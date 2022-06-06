// 迭代器：
// 迭代器模式：对一系列项执行某些任务。
// rust迭代器：懒惰的，除非调用消费迭代器的方法，否则迭代器本身没有任何效果。
// 1. 例子
fn iterator1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for v in v1_iter {
        println!("v:{}", v);
    }
}
// 2. iterator trait
// 所有迭代器都实现了 iterator trait

// 3. 迭代器的方法
// iter()：在不可变引用上创建迭代器，（迭代器里面是引用）
// into_iter():创建的迭代器会获得所有权，以前的是不是用不了了。
// iter_mut()：迭代可变的引用。
pub fn iterator31() {
    let v1 = vec![1, 2, 3];
    // 这里只所有要加上mut，是因为v1_iter.next()方法，会修改迭代器中的值。
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

pub fn iterator32() {
    let v1 = vec![1, 2, 3];
    // 这里只所有要加上mut，是因为v1_iter.next()方法，会修改迭代器中的值。
    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    // 这里v1不能被使用，因为into_iter已经获取了所有权。v1已经没有所有权了
    // println!("v1:{}",v1.len())
}

// 4. 定义在 iterator trait 上的别的方法，叫做迭代器适配器。
// 可以把迭代器转换成不同种类的迭代器
// 可以通过链式调用使用多个迭代器适配器来执行复杂的操作。
// 例如：map，接受一个闭包，闭包作用于每个函数，产生一个新的迭代器。
// collect方法：消耗性适配器，将结果收集到集合类型中。
pub fn iterator4() {
    let v1 = vec![1, 2, 3];
    // 将值加1，迭代器是惰性的，必须消耗迭代器中的值，而collect()就是消耗里面的数据。
    // let v2: Vec<_> ，<_>是让编译器自动判断类型。
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4])
}

// filter 迭代适配器
// 入参：闭包
// 返回值：bool，如果为true，则会返回，当调用.collect()时，会收集返回的值。

// 5. 创建自定义的迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn iterator5() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        // 能够被3整除
        .filter(|x| x % 3 == 0)
        .sum();
    // 18
    println!("sum:{}", sum)
}

// 6. 零开销抽象，Zero-Cost Abstraction
// 看起来迭代器抽象了，但是编译后和底层代码没有区别的。
