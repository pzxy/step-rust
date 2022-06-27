mod offer04;
mod offer03_find_repeat_number;

#[cfg(test)]
mod tests {
    use crate::offer04::find_number_in2_d_array;

    #[test]
    fn it_works() {
        let s = [[-5]];
        let array = find_number_in2_d_array(s, -10);
        println!("{}", array)
    }
}
