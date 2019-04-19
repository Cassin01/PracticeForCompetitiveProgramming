#include <iostream>

using namespace std;

int main(void) {
  int a;
  int b;
  cin >> a;
  cin >> b;

  if (a * b % 2 == 1 ){
    cout << "Odd" << endl;
  } else {
    cout << "Even" << endl;
  }
  return 0;
}
