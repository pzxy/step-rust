static mut MY_CONST: i32 = 42;
fn main() {
    unsafe {
        MY_CONST = 64;
        println!("The value of MY_CONST is: {}", MY_CONST);
    };
}
