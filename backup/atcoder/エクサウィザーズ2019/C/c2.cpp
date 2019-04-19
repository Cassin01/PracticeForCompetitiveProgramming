#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <algorithm>
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

  map<char, vector<int>> mp;
  for (int i = 0; i < N; i++)
  {
    auto itr = mp.find(s[i]);
    if (itr == mp.end())
    {
      for (int j = 0; j < N; j++)
      {
        if (s[i] == s[j])
        {
          mp[s[i]].push_back(j);
        }
      }
      sort(mp[s[i]].begin(), mp[s[i]].end() + mp[s[i]].size());
    }
  }

  int di = 0;
  int l = 0;
  int r = N - 1;
  for (int i = Q - 1; i >= 0; i--) {
    auto itr = mp.find(q0[i]);
    if (itr != mp.end()) {
      for (int j = mp[q0[i]].size() - 1; j >= 0; j--) {
        if (mp[q0[i]][j] == l && q1[i] == 'L') {
          l++;
          di++;
        }
      }
      for (int j = 0; j < mp[q0[i]].size(); j++) {
        if (mp[q0[i]][j] == r && q1[i] == 'R') {
          r--;
          di++;
        }
      }
    }
  }

  std::cout << N - di << endl;
  return 0;
}