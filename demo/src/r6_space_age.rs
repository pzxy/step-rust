//指示
// 给定一个以秒为单位的年龄，计算某人的年龄：
//
// 水星：轨道周期 0.2408467 地球年
// 金星：轨道周期 0.61519726 地球年
// 地球：轨道周期 1.0 地球年，365.25 地球日，或 31557600 秒
// 火星：轨道周期 1.8808158 地球年
// 木星：轨道周期 11.862615 地球年
// 土星：轨道周期 29.447498 地球年
// 天王星：轨道周期 84.016846 地球年
// 海王星：轨道周期 164.79132 地球年
// Mercury: orbital period 0.2408467 Earth years
// Venus: orbital period 0.61519726 Earth years
// Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
// Mars: orbital period 1.8808158 Earth years
// Jupiter: orbital period 11.862615 Earth years
// Saturn: orbital period 29.447498 Earth years
// Uranus: orbital period 84.016846 Earth years
// Neptune: orbital period 164.79132 Earth years
// 因此，如果你被告知某人的年龄为 1,000,000,000 秒，那么你应该可以说他们的年龄为 31.69 地球岁。
//
// 如果您想知道为什么冥王星没有晋级，请观看此 youtube 视频。
//
// 在解决这个问题时你可能想阅读一些 Rust 主题：
//
// 特征，包括 From 特征和实现你自己的特征
//
// 特征的默认方法实现
//
// 宏，使用宏可以减少样板文件并增加此练习的可读性。例如， 一个宏可以同时为多种类型实现一个特征，尽管years_during在 Planet 特征本身中实现也很好。宏可以定义结构及其实现。可以在以下位置找到开始使用宏的信息：
//
// Rust 编程语言中的宏章节
// 包含有用细节的旧版宏章节
// 通过例子学

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    year: f64,
}

const DAY: u64 = 3600 * 24;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { year: ((s / DAY) as f64 / 365.25) }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.year / 0.2408467
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.year / 0.61519726
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.year
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.year / 1.8808158
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.year / 11.862615
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.year / 29.447498
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.year / 84.016846
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.year / 164.79132
    }
}

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!(
            "Your result of {} should be within {} of the expected result {}",
            actual, delta, expected
        )
    }
}


#[test]
fn earth_age() {
    let duration = Duration::from(1_000_000_000);
    assert_in_delta(31.69, Earth::years_during(&duration));
}

#[test]
#[ignore]
fn mercury_age() {
    let duration = Duration::from(2_134_835_688);
    assert_in_delta(280.88, Mercury::years_during(&duration));
}

#[test]
#[ignore]
fn venus_age() {
    let duration = Duration::from(189_839_836);
    assert_in_delta(9.78, Venus::years_during(&duration));
}

#[test]
#[ignore]
fn mars_age() {
    let duration = Duration::from(2_129_871_239);
    assert_in_delta(35.88, Mars::years_during(&duration));
}

#[test]
#[ignore]
fn jupiter_age() {
    let duration = Duration::from(901_876_382);
    assert_in_delta(2.41, Jupiter::years_during(&duration));
}

#[test]
#[ignore]
fn saturn_age() {
    let duration = Duration::from(2_000_000_000);
    assert_in_delta(2.15, Saturn::years_during(&duration));
}

#[test]
#[ignore]
fn uranus_age() {
    let duration = Duration::from(1_210_123_456);
    assert_in_delta(0.46, Uranus::years_during(&duration));
}

#[test]
#[ignore]
fn neptune_age() {
    let duration = Duration::from(1_821_023_456);
    assert_in_delta(0.35, Neptune::years_during(&duration));
}
