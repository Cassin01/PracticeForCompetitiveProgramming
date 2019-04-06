//https: //qiita.com/drken/items/a5e6fe22863b7992efdb
// 最大和問題
#include <iostream>
#include <algorithm>

using namespace std;

// 入力
int n;
int a[10010]; // MAXよりも少し多めにとる

// DPテーブル
int dp[10010];

int main(void) {
  cin >> n;
  for (int i = 0; i < n; ++i) cin >> a[i];

  dp[0] = 0;
  for (int i = 0; i < n; ++i) {
    dp[i + 1] = max(dp[i], dp[i] + a[i]);
  }
  cout << dp[n] << endl;
  return 0;
}