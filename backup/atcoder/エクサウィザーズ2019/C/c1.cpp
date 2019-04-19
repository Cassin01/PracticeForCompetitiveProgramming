#include <iostream>
#include <vector>
#include <string>
#include <map>
using namespace std;

int main(void) {
  vector<int> v;
  vector<int> q0;
  vector<int> q1;
  int N, Q;
  string s;
  cin >> N;
  cin >> Q;
  cin >> s;
  for (int i = 0; i < N; i++) {
    v.push_back(1);
  }
  for (int i = 0; i < Q; i++) {
    char tmp;
    cin >> tmp;
    q0.push_back(tmp);
    cin >> tmp;
    q1.push_back(tmp);
  }

  map<char, vector<int> > mp;
  for (int i = 0; i < N; i++) {
    auto itr = mp.find(s[i]);
    if (itr == mp.end()) {
      for (int j = 0; j < N; j++) {
        if (s[i] == s[j]) {
          mp[s[i]].push_back(j);
        }
      }
    }
  }

  for (int i = 0; i < Q; i++) {
    auto itr = mp.find(q0[i]);
    if (itr != mp.end()) {
      for (auto j: mp[q0[i]]) {
        if(v[j] != 0) {
          if (j != 0 && j != N - 1) {
            if (q1[i] == 'L') {
              v[j - 1] += v[j];
              v[j] = 0;
            } else {
              v[j + 1] += v[j];
              v[j] = 0;
            }
          } else if (q1[i] == 'L' && j == N - 1) {
            v[j - 1] += v[j];
            v[j] = 0;
          } else if (q1[i] == 'R' && j == 0) {
            v[j + 1] += v[j];
            v[j] = 0;
          } else {
            v[j] = 0;
          }
        }
      }
    }
  }

  int ans = 0;
  for (auto vc: v) {
    ans += vc;
  }
  cout << ans << endl;
  return 0;
}