use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // tokio::spawn 相当于go关键字
    tokio::spawn(async {
        // 这种其实是异步编程，这里面的是顺序执行的。
        hello1().await;
        hello2().await
    });
    hello3().await
}

async fn hello1() {
    for i in 1..5 {
        sleep(Duration::from_secs(1));
        println!("hello1--{}", i)
    }
}

async fn hello2() {
    for i in 1..5 {
        sleep(Duration::from_secs(1));
        println!("hello2=={}", i)
    }
}

async fn hello3() {
    for i in 1..5 {
        sleep(Duration::from_secs(1));
        println!("hello3**{}", i)
    }
}

// 可以看到发生了异步
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
