pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let s1 = "str";
        let s2 = String::from("String");
        let b1 = is_string(s1);
        let b2 = is_string(s2);
        println!("b1:{},b2:{}", b1, b2)
    }
    // arena:Arena arena:TypeArena 可以即时创建对象。需要优化时可以了解下。
}

fn is_string<T: Into<String>>(pwd: T) -> bool {
    pwd.into().len() > 5
}
