extern crate core;

mod offer03_find_repeat_number;
mod offer04_find_number_in2_d_array;
mod offer05_replace_space;
mod offer06_reverse_print;
mod offer07_build_tree;
mod offer09_stack_for_queue;
mod offer10_fib;
mod offer11_min_array;
mod offer12_char_exist;
mod offer13_moving_count;
mod offer15_hamming_weight;

#[cfg(test)]
mod tests {
    use crate::offer04_find_number_in2_d_array::find_number_in2_d_array;
    use crate::offer05_replace_space::replace_space;
    use crate::offer10_fib::{fib, fib2, fib22, fib222, fold2};
    use crate::offer12_char_exist::exist;
    use crate::offer13_moving_count::moving_count;
    use crate::offer15_hamming_weight::hamming_weight;

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

    #[test]
    fn offer12() {
        let board = vec![vec!['h', 'e', 'l', 'l', 'o'], vec!['w', 'o', 'r', 'l', 'd']];
        let ret = exist(board, "heor".to_string());
        assert_eq!(ret, true)
    }

    #[test]
    fn offer13() {
        let ret = moving_count(2, 3, 1);
        assert_eq!(ret, 3);
        let ret = moving_count(3, 1, 0);
        assert_eq!(ret, 1);
    }
    #[test]
    fn offer15() {
        for i in -3..=1 {
            println!("{}", i)
        }
        let ret = hamming_weight(11);
        assert_eq!(0.1+0.2,0.3);
        assert_eq!(ret, 3)
    }
}
