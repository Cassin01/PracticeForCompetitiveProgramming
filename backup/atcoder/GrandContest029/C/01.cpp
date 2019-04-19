#include <iostream>
#include <vector>
#include <string>

using namespace std;

void show(int i, int k, int l) {
  cout << i << " " << k << " " << l << endl;
}

int main(void) {
  int N;
  cin >> N;
  vector<int> S;
  for (int i = 0; i < N; i++) {
    int c;
    cin >> c;
    S.push_back(c);
  }

  int l = 0;
  int use = 1;
  int pn = S[0];
  for (int i = 1; i < N; i++) {
    if (pn >= S[i]) {
      l++;
    }
    if (use < l) {
      use++;
    }
    pn = S[i];
  }

  cout <<  use << endl;
  return 0;
}
