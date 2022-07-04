mod offer03_find_repeat_number;
mod offer04_find_number_in2_d_array;
mod offer05_replace_space;
mod offer06_reverse_print;
mod offer07_build_tree;
mod offer09_stack_for_queue;
mod offer10_fib;
mod offer11_min_array;

#[cfg(test)]
mod tests {
    use crate::offer04_find_number_in2_d_array::find_number_in2_d_array;
    use crate::offer05_replace_space::replace_space;
    use crate::offer10_fib::{fib, fib2, fib22, fib222, fold2};

    #[test]
    fn it_works() {
        let s = [[-5]];
        let array = find_number_in2_d_array(s, -10);
        println!("{}", array)
    }

    #[test]
    fn offer5() {
        let ret = replace_space("we are happy".to_string());
        println!("{}", ret);
        assert_eq!(ret, "we%20are%20happy")
    }

    #[test]
    fn offer10() {
        let aa = fib22(48);
        println!("{}", aa);
    }
}
