// 调用rust方法，rust处理再调用c方法。返回
class foo
{
public:
    foo();
    ~foo();

public:
    void send(int a);
    void recv(int a);
};
