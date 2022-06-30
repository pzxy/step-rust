mod offer04_find_number_in2_d_array;
mod offer03_find_repeat_number;
mod offer05_replace_space;
mod offer06_reverse_print;
mod offer07_build_tree;

#[cfg(test)]
mod tests {
    use crate::offer04_find_number_in2_d_array::find_number_in2_d_array;
    use crate::offer05_replace_space::replace_space;

    #[test]
    fn it_works() {
        let s = [[-5]];
        let array = find_number_in2_d_array(s, -10);
        println!("{}", array)
    }

    #[test]
    fn offer5() {
        let ret = replace_space("we are happy".to_string());
        println!("{}",ret);
        assert_eq!(ret, "we%20are%20happy")
    }

}
