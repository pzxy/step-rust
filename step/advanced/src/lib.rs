mod async_wait;

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
