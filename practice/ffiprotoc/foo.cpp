#include "foo.hpp"
#include <iostream>

void foo::send(int a)
{
    std::cout << "send: " << a << std::endl;
}

void foo::recv(int b)
{
    std::cout << "recv: " << b << std::endl;
}
