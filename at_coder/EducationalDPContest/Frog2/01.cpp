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
long long h[10010];

long long dp[10010];

int main(int argc, char const *argv[])
{
  int N, K; cin >> N >> K;
  for(int i = 0; i < N; i++)
  {
    cin >> h[i];
  }

  for(int i = 0; i < 100100; i++)
  {
    dp[i] = INF;
  }

  dp[0] = 0;

  for(int i = 0; i < N; i++)
  {
    for(int j = 0; j <= K; j++)
    {
      chmin(dp[i+j], dp[i] + abs(h[i] - h[i+j]));
    }
    cout << dp[N-1] << endl;
  }
  return 0;
}
