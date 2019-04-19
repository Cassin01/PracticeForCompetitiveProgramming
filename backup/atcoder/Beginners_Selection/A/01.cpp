#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  int a;
  int b;
  int c;
  string s;

  cin >>a;
  cin >>b;
  cin >>c;
  cin >>s;

  string ans1 = to_string(a + b + c);

  cout << ans1 << " " << s << endl;

  return 0;
}
