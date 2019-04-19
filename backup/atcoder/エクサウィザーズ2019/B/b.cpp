#include <iostream>
#include <string>

using namespace std;

int main(void) {
  int N;
  string s;
  std::cin >> N;
  std::cin >> s;
  int r_num =0;
  int b_num =0;

  for (auto c: s) {
    if (c == 'R') {
      r_num++;
    } else {
      b_num++;
    }
  }
  if (r_num > b_num) {
    cout << "Yes" << endl;
  } else {
    cout << "No" << endl;
  }
  return 0;
}
