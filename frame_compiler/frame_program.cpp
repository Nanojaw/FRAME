#include "iostream"
#include <vector>

unsigned long long fib(unsigned long long fibIdx) {
    if((fibIdx <= 2)) {
        return 1;
    }
    else {
        return ((fib(((fibIdx) - (1)))) + (fib(((fibIdx) - (2)))));
    }
}
int main() {
    unsigned long long fibHuH = fib(8);

    std::cout << fibHuH << std::endl;

    return 0;
}