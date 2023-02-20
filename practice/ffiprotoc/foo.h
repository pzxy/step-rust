// 调用rust方法，rust处理再调用c方法。返回
#include <iostream>
class foo
{
public:
    static foo *instance();

public:
    foo();
    ~foo();

public:
    void send(int a);
    void recv(int a);
};
