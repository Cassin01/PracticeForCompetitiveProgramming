#include<iostream>
#include<string>

using namespace std;


int main(void) {
  string S;
  cin >> S;
  int ans = 0;
  int till = 0;
  for (int i = S.size() - 1; i >= 0; ++i) {
    if (S[i] == 'B') {
      till++;
    } else {
      break;
    }
  }

  while (true) {
    char pw = S[0];
    int changed = 0;
    for(int i = 1; i < S.size() - till; ++i) {
      if (S[i] == 'W' && pw == 'B') {
        changed = 1;
        S[i] = 'B';
        S[i - 1] = 'W';
        ans++;
      }
      pw = S[i];
    }
    if (changed == 0) break;
  }
  cout << ans << endl;
  return 0;
}
