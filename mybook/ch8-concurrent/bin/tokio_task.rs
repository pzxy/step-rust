use std::thread::sleep;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    // tokio::spawn 相当于go关键字
    tokio::spawn(async {
        // 一定会先执行hello1 再执行hello2
        // 如果他们中有future返回pending则会让出线程。
        hello1().await;
        hello2().await
    });
    hello3().await;
}

async fn hello1() {
    for i in 1..5 {
        println!("hello1--{}", i);
    }
    // 这里返回pending，这里会让出线程。
    time::sleep(Duration::from_secs(2)).await;
}

async fn hello2() {
    for i in 1..5 {
        println!("hello2=={}", i)
    }
}

async fn hello3() {
    for i in 1..5 {
        println!("hello3**{}", i);
        sleep(Duration::from_secs(1));
    }
}
// 执行结果：
// hello3**1
// hello1--1
// hello1--2
// hello1--3
// hello1--4
// hello3**2
// hello2==1
// hello2==2
// hello2==3
// hello2==4
// hello3**3
// hello3**4

// 执行结果分析：
// 1. hello2一定在hello1后面，因为在同一个future中(async {})。执行顺序是一定的。
// 2. 这里未必是hello3先执行，看hello1和hello3哪个先准备好。
// 3. hello1一定是连续打印完，然后hello3再执行。因为hello1执行完会sleep1秒。这时sleep future会返回pending，会暂时让出线程。
// 由于hello2要2秒后才能执行，所以hello3会抢占住线程。然后执行，hello3执行两次后刚好2秒过去，这时hello2开始执行。
// 这里hello3的睡眠是线程的sleep，是不会返回pending的，那为什么hello2能抓住时机执行呢？那是因为hello2和hello3在不同的future中。
// hello2和hello3属于不同的task，task其实就可以看成是go中的协程了。
// 因为go是自己来实现调度器的，所以他更抽象，直接让我们将协程当成线程的概念用了。我们不必关系里面的异步模型是什么。但是rust我们需要了解的。

// 易错点：
// 1. hello3 不能也用 tokio::spawn(async { hello3().await });
// 这样会直接退出的，因为主线程直接结束了。和go中一样，go全部逻辑都开协程，也是会退出的。
// 2. async方法必须使用await调用才会执行。
// 因为async标记的方法是编译后是future，future不是现在要执行的。他的执行时机就是.await标记的。
// 3. tokio::spawn() 为什么入参必须是 async {} 包装一下。
// 因为tokio::spawn是执行future的。虽然hello1也是async标记了，编译后也会是future，但是呢，他在编译后才是future。
// 而编译检查在前，此时hello1他还没转变成future代码呢。如果我们自己创建一个结构体，让这个结构实现future trait。
// 那么就不用加async {} 包装了，直接创建这个结构体传进去就行了。
// 4. rc可以在tokio::spawn()中用吗？
// 最好不要，因为 spawn 入参需要实现send trait。因为调度器可能会将当前task放到别的线程上执行。
// 但有例外：如果 rc 生命周期内没有出现 .await代码，也没问题。比如：
//         {
//             let rc = Rc::new("hello");
//             println!("{}", rc);
//         }
// 5. thread::sleep 和 tokio::time::sleep有什么区别？
// thread::sleep 会继续计算，不回让出线程让别的future执行。
// tokio::time::sleep会让立即返回，让出线程让别的future执行。等sleep时间到后，sleep future会调用wake，
// 通知调度器重新执行自己的poll方法,这次调用因为时间已经到了，所以会立即返程成功，然后往下执行。
