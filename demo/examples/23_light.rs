// 为枚举通信号灯实际发现一个trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同；
fn main() {
    let s = SignLamp::Red;
    let ss = s.TimeOfDuration();
    println!("{:?}", ss);
}

enum SignLamp {
    Red,
    Yellow,
    Green,
}

trait Light {
    fn TimeOfDuration(&self) -> i32;
}

impl Light for SignLamp {
    fn TimeOfDuration(&self) -> i32 {
        match self {
            SignLamp::Red => 30,
            SignLamp::Green => 60,
            SignLamp::Yellow => 1,
        }
    }
}
