// 过程宏
use macros::query;

fn main() {
    // query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id and u.age > 10);
     query!(SELECT * FROM users WHERE age > 10);
}
// TokenStream是一个迭代器，https://doc.rust-lang.org/proc_macro/enum.TokenTree.html
// pub enum TokenTree {
//     Group(Group),
//     Ident(Ident),
//     Punct(Punct),
//     Literal(Literal),
// }

// 后三个分别是 Ident（标识符）、Punct（标点符号）和 Literal（字面量）。
// 这里的 Group（组），是因为如果你的代码中包含括号，比如{} [] <> () ，那么内部的内容会被分析成一个 Group（组）。

// 输入：
// SELECT * FROM users WHERE age > 10)
// 或者 query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id and u.age > 10);
// 输出：
// TokenStream [
//     Ident {
//         ident: "SELECT",
//         span: #0 bytes(55..61),
//     },
//     Punct {
//         ch: '*',
//         spacing: Alone,
//         span: #0 bytes(62..63),
//     },
//     Ident {
//         ident: "FROM",
//         span: #0 bytes(64..68),
//     },
//     Ident {
//         ident: "users",
//         span: #0 bytes(69..74),
//     },
//     Ident {
//         ident: "WHERE",
//         span: #0 bytes(75..80),
//     },
//     Ident {
//         ident: "age",
//         span: #0 bytes(81..84),
//     },
//     Punct {
//         ch: '>',
//         spacing: Alone,
//         span: #0 bytes(85..86),
//     },
//     Literal {
//         kind: Integer,
//         symbol: "10",
//         suffix: None,
//         span: #0 bytes(87..89),
//     },
// ]
