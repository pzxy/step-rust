#[cfg(test)]

mod tests {
    use crate::read_file;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_file() {
        assert_eq!(read_file().unwrap_err().to_string(), "");
        println!("Success!")
    }
}
