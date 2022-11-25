use std::thread::sleep;
use std::time::Duration;

#[test]
#[ignore]
fn test_for_usize() {
    for i in 1.. {
        sleep(Duration::from_secs(1));
        println!("i:{}", i);
        if i == 5 {
            return;
        }
    }
}
