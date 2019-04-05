#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>

using namespace std;

template<class T> inline bool chmin(T& a, T b) { if (a > b) { a = b; return true; } return false; }


const long long INF = 1LL << 60;

long long h[100010];

long long rec(int i) {
  // 足場 0 のコストは 0
  if (i == 0) return 0;

  // i - 1, i - 2 それぞれ試す
  long long res = INF;
  chmin(res, rec(i-1) + abs(h[i] - h[i - 1]));
  chmin(res, rec(i-2) + abs(h[i] - h[i - 2]));
  return res;
}

int main(int argc, char const *argv[])
{
  int N; cin >> N;
  for(int i = 0; i < N; i++)
  {
    cin >> h[i];
  }
  /* code */

  return 0;
}
