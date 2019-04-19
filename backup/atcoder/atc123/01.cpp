#include <iostream>

using namespace std;

int kiri(int i, int j) {
  return (i + j - 1) / j;
}

int main(void) {
  cout << kiri(7, 3) << endl;
  return 0;
}
