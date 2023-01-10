#include <iostream>
#include <memory>
#include <thread>
using namespace std;

class Foo {
    int x;
    public:
    Foo(int x):x{x} {}
    int getX() {
        return x;
    }
    ~Foo() {
        cout << "Now destructing Foo" << endl;
    }
};

void print_sp_uc(std::shared_ptr<Foo> sp) {
    cout << "sp uc: " << sp.use_count() << endl;
}



int main() {
    // std::shared_ptr<Foo> sp(new Foo(69));
    // cout << "sp->getX(): " << sp->getX() << endl;
    // cout << "sp.use_count(): " << sp.use_count() << endl;

    // // sharing the pointer
    // std::shared_ptr<Foo> sp1 = sp;
    // cout << "sp.use_count(): " << sp.use_count() << endl;

    // std::shared_ptr<Foo> sp2 = sp;
    // cout << "sp.use_count(): " << sp.use_count() << endl;

    std::shared_ptr<Foo> sp(new Foo(69));
    print_sp_uc(sp);
    print_sp_uc(sp);
    auto sp1 = sp;
    print_sp_uc(sp);


    return 0;
}