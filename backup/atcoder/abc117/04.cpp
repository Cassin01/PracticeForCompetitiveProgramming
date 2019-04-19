#include <iostream>
#include <vector>
using namespace std;

int main(void) {
  int N, K;
  int tp;
  vector<int> v;
  cin >> N;
  cin >> K;

  for(int i = 0; i < N; i++) {
    cin >> tp;
    v.push_back(tp);
  }

  int max_ = 0;
  for(int i = 0; i <= K; i++) {
    int f = 0;
    for(int j = 0; j < N; j++) {
    }
    if (max_ < f) {
      max_ = f;
    }
  }
  cout << max_ << endl;
  return 0;
}


vector<int> d2b(int d) {
  vector<int> b;
  while (d > 0) {
    b.push_back(d % 2);
    d /= 2;
  }
  return b;
}
