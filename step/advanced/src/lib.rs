use crate::tracing::trance1;

mod async_wait;
mod json_serde;
pub mod logger;
mod orm_diesel;
mod pow;
mod sqlite;
mod tracing;

pub fn log_demo() {
    trance1()
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::async_wait::async_main;

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
