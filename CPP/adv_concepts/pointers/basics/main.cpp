#include<iostream>

int main () {
    int num = 10;
    int* ptr = &num;
    int deref_ptr = *ptr;

    std::cout << num << std::endl;
    std::cout << ptr << std::endl;
    std::cout << deref_ptr << std::endl;
}