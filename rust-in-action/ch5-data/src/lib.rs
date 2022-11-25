extern crate core;

pub mod accumulator;
pub mod callfunc;
pub mod summator;

#[test]
fn test_summator() {
    summator::summator();
    accumulator::accumulator()
}

#[test]
fn test_call_func() {
    callfunc::call_func()
}
