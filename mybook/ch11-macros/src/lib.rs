// 过程宏-函数宏
use proc_macro::TokenStream;

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse() // 返回 TokenStream，因为TokenStream实现了FromStr trait。
        .unwrap()
}

// 过程宏-派生宏
mod raw_builder;
use raw_builder::BuilderContext;
#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    BuilderContext::render(input).unwrap().parse().unwrap()
}
