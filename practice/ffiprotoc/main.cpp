#include <iostream>
int main(){
    
}
// no_mangle -> interface.h
// 1. 定义数据传输类型。
// 2. cpp这边，定义一个 .h 然后在.cpp中实现它，里面extern ”C“ ，接着用 bindgen来生成代码 rust代码。
// 3. rust这边 创建一个 .rs 文件 ，里面pub extern ”C“ ，实现了后， 用 cbindgen 来生成一个 .h文件。