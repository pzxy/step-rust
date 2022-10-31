extern crate core;

use crate::accumulator::accumulator;
use crate::callfunc::call_func;
use crate::summator::summator;

mod accumulator;
mod callfunc;
mod summator;

#[test]
fn test_summator() {
    summator();
    accumulator()
}

#[test]
fn test_call_func() {
    call_func()
}
