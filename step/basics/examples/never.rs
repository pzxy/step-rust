fn main() {
    bar()
}

fn bar() -> ! {
    //  continue 就是返回!类型.
    // 此外 loop, pnaic! 返回值都是!类型.
    panic!()
}
