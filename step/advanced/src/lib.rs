mod async_wait;
pub mod r_log;
use crate::r_log::log1;

pub fn log_demo() {
    log1()
}
#[cfg(test)]
mod tests {
    use crate::async_wait::async_main;
    use futures::executor::block_on;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn async_wait() {
        block_on(async_main());
    }
}
