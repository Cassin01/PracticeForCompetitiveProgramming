#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>
using namespace std;
template<class T> inline bool chmin(T& a, T b) { if (a > b) { a = b; return true; } return false; }
template<class T> inline bool chmax(T& a, T b) { if (a < b) { a = b; return true; } return false; }

const long long INF = 1LL << 60;

int N;
long long h[100010];

long long dp[100010];

long long rec(int i) {
  if (dp[i] < INF) return dp[i];

  if (i == 0) return 0;

  long long res = INF;

  chmin(res, rec(i-1) + abs(h[i] - h[i - 1]));
  if (i > 1) chmin(res, rec(i-2) + abs(h[i] - h[i-2]));
  return dp[i] = res;
}

int main(int argc, char const *argv[])
{
  int N; cin >> N;
  for (int i = 0; i < N; ++i) cin >> h[i];

  for (int i = 0; i < 100010; ++i) dp[i] = INF;
  cout << rec(N-1) << endl;
  return 0;
}