#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>

template<class T> inline bool chmin(T& a, T b) { if (a > b) { a = b; return true; } return false; }

template<class T> inline bool chmax(T& a, T b) { if (a < b) { a = b; return true; } return false; }

using namespace std;

const int MAX_V = 100100;

const long long INF = 1LL << 60;

int N, W;
long long dp[110][MAX_V];
long long w[110];
long long v[110];

int main(int argc, char const *argv[])
{
    cin >> N >> W;
    for (int i = 0; i < N; i++) cin >> w[i] >> v[i];

    for (int i = 0; i < 110; ++i) for (int j = 0; j < MAX_V; j++) dp[i][j] = INF;

    dp[0][0] = 0;

    for (int i = 0; i <= N; i++) {
        for (int sum_v = 0; sum_v <= MAX_V; sum_v++) {
            if (sum_v - v[i] >= 0) {
                chmin(dp[i+1][sum_v], dp[i][sum_v - v[i]] + w[i]);
            }
            chmin(dp[i+1][sum_v], dp[i][sum_v]);
        }
    }

    long long ref = 0;
    for(int i = 0; i < MAX_V; i++)
    {
        if (dp[N][i] <= W) {
            ref = i;
        }
    }
    cout << ref << endl;

    return 0;
}