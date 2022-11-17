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

// 使用syn/quote重写过程宏-派生宏，RawBuilder -> Builder。
mod builder;
use syn::{parse_macro_input, DeriveInput};
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder::BuilderContext::from(input).render().into()
}

// 留着打印原始结构玩
// #[proc_macro_derive(Builder)]
// pub fn derive_builder(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     println!("{:#?}", input);
//     TokenStream::default()
// }


