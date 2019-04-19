#include<iostream>
#include<string>

using namespace std;

int main(void) {
  string S;
  cin >> S;
  int wnum = 0;
  int bnum = 0;
  for (int i = S.size() - 1; i >= 0; i--) {
    if (S[i] == 'B') {
      bnum++;
    }
  }

  int ans = 0;
  for (int i = S.size() - 1; i >= 0; i--) {
    if (S[i] == 'W') {
      wnum++;
    } else {
      ans = ans + bnum * wnum;
      bnum--;
      wnum = 0;
    }
  }
  cout << ans << endl;
  return 0;
}

