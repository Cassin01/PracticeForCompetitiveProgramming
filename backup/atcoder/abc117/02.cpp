#include <iostream>
#include <vector>

using namespace std;
bool other(int i, vector<int> v, int N) {
  int sum = 0;
  for (int j = 0; j < N; j++) {
    if (j == i) {
    } else {
      sum = sum + v[j];
    }
  }
  if (v[i] >= sum) {
    return false;
  } else {
    return true;
  }
}

int main(void) {
  int N;
  vector<int> v;
  int tp;
  cin >> N;

  for (int i = 0; i < N; i++) {
    cin >> tp;
    v.push_back(tp);
  }

  for (int i = 0; i < N; i++) {
    if (!other(i, v, N)) {
      cout << "No" << endl;
      return 0;
    }
  }
  cout << "Yes" << endl;
  return 0;
}

