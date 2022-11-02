mod hexdump;
mod read_file;
mod serde_bincode;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hexdump::hexdump;
    use crate::read_file::read_file;
    use crate::serde_bincode::serialize;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_serialize() {
        serialize()
    }
    #[test]
    fn test_hexdump() {
        let _r = hexdump();
    }
    #[test]
    fn test_read_file() {
        read_file();
    }
}
