#include <iostream>
#include <vector>
#include <cstdlib>
using namespace std;

int A, B, C, N;
const int max_n = 8;
vector<int> l(max_n + 1, 0);
int rtn = 2000000000;

void calc(vector<char> v) {
  int mp = 0;
  int cnta = 0, cntb = 0, cntc = 0;
  for (int i = 1; i <= N; i++) {
    if (v[i] == 'A') {
      if (cnta != 0) mp += 10;
      cnta += l[i];
    } else if (v[i] == 'B') {
      if (cntb != 0) mp += 10;
      cntb += l[i];
    } else if (v[i] == 'C') {
      if (cntc != 0) mp += 10;
      cntc += l[i];
    }
  }
  if (cnta == 0 || cntb == 0 || cntc == 0) return;
  mp += abs(A - cnta) + abs(B - cntb) + abs(C - cntc);
  rtn = min(rtn, mp)l
}