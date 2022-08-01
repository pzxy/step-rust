mod async_wait;
pub mod logger;
mod tracing;
mod tokio;

use crate::logger::log1;
use crate::tracing::trance1;

pub fn log_demo() {
    // log1()
    trance1()
}
#[cfg(test)]
mod tests {
    use crate::async_wait::async_main;
    use futures::executor::block_on;
    use crate::tokio::tokio1;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn async_wait() {
        block_on(async_main());
    }

    #[test]
    fn test_tokio(){
         tokio1()
    }
}
