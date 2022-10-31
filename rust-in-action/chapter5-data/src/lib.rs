use crate::accumulator::accumulator;
use crate::summator::summator;

mod accumulator;
mod summator;

#[test]
fn test_summator() {
    summator();
    accumulator()
}
