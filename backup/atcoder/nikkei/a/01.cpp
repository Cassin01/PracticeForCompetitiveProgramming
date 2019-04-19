#include <iostream>
using namespace std;

int main() {
  int max, min;
  int A, X, Y;
  cin >> A;
  cin >> X;
  cin >> Y;
  if (X > Y) {
    max = Y;
  } else {
    max = X;
  }

  min = X + Y - A;
  if (min <= 0) {
    min = 0;
  }

  cout << max << " " << min << endl;

}

