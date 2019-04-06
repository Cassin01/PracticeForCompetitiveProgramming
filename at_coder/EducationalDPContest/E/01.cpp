#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>

template<class T> inline bool chmax(T& a, T b) { if (a < b) { a = b; return true; } return false; }

using namespace std;

int N, W;
long long dp[100][1000000000];
long long w[1000000000];
long long v[1000000000];

int main(int argc, char const *argv[])
{
    cin >> N;
    cin >> W;
    for(int i = 0; i < N; i++) cin >> w[i] >> v[i];

    for (int i = 0; i <= N; i++) {
        for (int sum_w = 0; sum_w <= W; sum_w++) {
            if (sum_w - w[i] >= 0) {
                chmax(dp[i+1][sum_w], dp[i][sum_w - w[i]] + v[i]);
            }
            chmax(dp[i+1][sum_w], dp[i][sum_w]);
        }
    }

    return 0;
}