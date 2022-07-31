mod a1_file;
mod demo1;

#[cfg(test)]

mod tests {
    use crate::a1_file::{read_file1, read_file2};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_file() {
        assert_eq!(
            read_file1().unwrap_err().to_string(),
            read_file2().unwrap_err().to_string()
        );
        println!("Success!")
    }
}
