///  [高分答案](https://exercism.org/tracks/rust/exercises/space-age/solutions/tokenrove)

fn main(){}
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self { Duration((s as f64) / (31557600 as f64)) }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 { d.0 / Self::period() }
}
macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n { fn period() -> f64 { $p } }
    }
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

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
