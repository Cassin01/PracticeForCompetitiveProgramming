#include <stdio.h>

const int MAX_N = 1000; // nの最大値
const int MAX_W = 5000; // Wの最大値

// 入力
int n, W;
int w[MAX_N], v[MAX_N];

// メモ化テーブル。
// dp[i][j]はi番目以降の品物から重さの和がj以下なるように選んだときの価値の和の最大値を表す。
// -1なら値が未決定であることを表す
int dp[MAX_N + 1][MAX_W + 1];

int max1(int a, int b) {
    if (a > b)
        return a;
    else
        return b;
}

// i番目以降の品物から重さの和がj以下なるように選んだときの、
// 取りうる価値の総和の最大値を返す関数
int rec_dp(int i, int j) {
  if (dp[i][j] != -1) {
    // すでに調べたことがあるならその結果を再利用
    return dp[i][j];
  }
  int res;
  if (i == n) {
    // 品物がもう残っていないときは、価値の和の最大値は0で確定
    res = 0;
  } else if (j < w[i]) {
    // 残りの容量が足りず品物iを入れられないので、入れないパターンだけ処理
    res = rec_dp(i + 1, j);
  } else {
    // 品物iを入れるか入れないか選べるので、両方試して価値の和が大きい方を選ぶ
    res = max1(
        rec_dp(i + 1, j),
        rec_dp(i + 1, j - w[i]) + v[i]
    );
  }
  // 結果をテーブルに記憶する
  return dp[i][j] = res;
}

void solve_dp() {
  for (int i = 0; i < MAX_N + 1; i++)
    for (int j = 0; j < MAX_W + 1; j++)
      dp[i][j] = -1;
  
  // 0番目以降で容量W以下の場合の結果を表示する
  int ans = rec_dp(0, W);
  printf("%d\n", ans);
}
