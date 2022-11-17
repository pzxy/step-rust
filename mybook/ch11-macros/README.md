[原文链接](https://time.geekbang.org/column/article/481359)
# 声明宏(declarative macro)
概念：对代码模版做简单替换。主要做的就是通过简单的接口，把不断重复的逻辑包装起来，然后在调用的地方展开而已，不涉及语法树的操作。

11个常用类型：
- item，比如一个函数、结构体、模块等。
- block，代码块。比如一系列由花括号包裹的表达式和语句。
- stmt，语句。比如一个赋值语句。
- pat，模式。
- expr，表达式。
- ty，类型。比如 Vec。
- ident，标识符。比如一个变量名。
- path，路径。比如：foo、::std::mem::replace、transmute::<_, int>。
- meta，元数据。一般是在 #[...]  和  #![...]  属性内部的数据。
- tt，单个的 token 树。
- vis，可能为空的一个  Visibility  修饰符。比如 pub、pub(crate)。

例子 [rule.rs](./examples/rule.rs)。

# 过程宏
概念：可以深度定制和生成代码。
主要包括三种：函数宏(function-lick macro)、属性宏(attribute macro)、派生宏(derive macro)。

过程宏需要单独crate，Cargo.toml配置,这样编译器才允许你使用`#[proc_macro]` 相关的宏
```toml
[lib]
proc-macro = true

```
### 1. 函数宏
看起来像函数的宏，但在编译时会进行处理。

本质：把输入的 TokenStream 处理成输出的 TokenStream。
需要写到 [lib.rs](./src/lib.rs) 中，使用例子 [query.rs](./examples/query.rs)。
### 2. 属性宏
可以在代码块上添加属性，为代码提供更多功能。

### 3. 派生宏
为结构体的 derive 属性添加新的功能。比如 `#[derive(Debug)]` 为我们的数据结构提供 `Debug trait` 的实现

例子的实现原理：用 [jinja](https://jinja.palletsprojects.com/en/3.0.x/) 定义一个模版,然后用 [askama](https://github.com/djc/askama) 库处理这个模版。

- 添加依赖
```toml
[dependencies]
anyhow = "1"
askama = "0.11"
```
- 定义模版 [builder.j2](./templates/builder.j2)。
- 处理模版 [raw_builder.rs](./src/raw_builder.rs)。
- 最后使用 [command.rs](./examples/command.rs)。