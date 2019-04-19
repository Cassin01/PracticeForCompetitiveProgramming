#include <iostream>

using namespace std;

int main(void) {
  int a;
  cin >> a;
  int ans = 0;
  for (int i = 0; i < 3; i++) {
    if (a % 10 == 1) {
      ans++;
    }
    a = a / 10;
  }
  cout << ans << endl;
  return 0;
}
