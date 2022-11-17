// 过程宏
use proc_macro::TokenStream;

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse() // 返回 TokenStream，因为TokenStream实现了FromStr trait。
        .unwrap()
}
