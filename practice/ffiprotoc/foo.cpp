#include "foo.hpp"
#include <iostream>

foo *foo::instance()
{
    static foo ins;
    return &ins;
}

void foo::send(int a)
{
    std::cout << "send: " << a << std::endl;
}

void foo::recv(int b)
{
    std::cout << "recv: " << b << std::endl;
}
// rust call c
extern "C"
{
    void send(int a)
    {
        foo::instance()->send(a);
    }
    void recv(int a)
    {
        foo::instance()->recv(a);
    }
}