// 实现一个函数，将字符串中的每个空格替换成"%20"，
// 例如： "we are happy"
// 输出:  "we%20are%20happy"

fn main(){
    let ret = replace_space("we are happy".to_string());
    println!("{}", ret);
    assert_eq!(ret, "we%20are%20happy")
}
pub fn replace_space(s: String) -> String {
    let mut ret = String::from("");
    for c in s.chars() {
        if c == ' ' {
            ret.push('%');
            ret.push('2');
            ret.push('0');
        } else {
            ret.push(c);
        }
    }
    ret
}

// fn replace_space2(s: String) -> String {
//     let mut ret = String::from("");
//     for c in s.chars() {
//         if c == ' ' {
//             ret.push('%');
//             ret.push('2');
//             ret.push('0');
//         } else {
//             ret.push(c);
//         }
//     }
//     ret
// }