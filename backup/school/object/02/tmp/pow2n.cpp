#include <iostream>

int pow2n (int n);

int main(void) {
  std::cout << pow2n (18) << std::endl;
}
int pow2n (int n) {
  if (n == 0) {
    return 1;
  }
  else {
    return 2 * pow2n (n - 1);
  }
}
