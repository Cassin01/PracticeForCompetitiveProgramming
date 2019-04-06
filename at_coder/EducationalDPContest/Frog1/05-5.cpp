#include <iostream>
#include <vector>
#include <cstdlib>

using namespace std;

template<class T> inline bool chmin(T& a, T b) { if (a > b) { a = b; return true; } return false; }
template<class T> inline bool chmax(T& a, T b) { if (a < b) { a = b; return true; } return false; }

const long long INF = 1LL << 60;

int N;
long long h[100010];

long long dp[100010];

int main(int argc, char const *argv[])
{
  int N; cin >> N;
  for(int i = 0; i < N; i++)
  {
    cin >> h[i];
  }

  for(int i = 0; i < 100010; i++)
  {
    dp[i] = INF;
  }

  for(int i = 0; i < N; i++)
  {
    chmin(dp[i + 1], dp[i] + abs(h[i] - h[i + 1]));
    chmax(dp[i + 2], dp[i] + abs(h[i] - h[i + 2]));
  }
  cout << dp[N-1] << endl;
  return 0;
}
