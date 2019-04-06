#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>

using namespace std;

template<class T> inline bool chmax(T& a, T b) { if (a < b) { a = b; return true; } return false; }

long long w[110];
long long v[110];

long long dp[110][100010];


int main(int argc, char const *argv[])
{
    int N; cin >> N;
    int W; cin >> W;
    for(int i = 0; i < N; i++)
    {
        cin >> w[i];
        cin >> v[i];
    }

    for (int i = 0; i < 110; i++) {
        for (int j = 0; j < 100010; j++) {
            dp[i][j] = 0;
        }
    }

    for(int i = 0; i < N; i++)
    {
        for(int k = 0; k < N; k++)
        {
            chmax(dp[i+1][W], dp[i][W ] + v[k]);
        }
    }

    long long res = 0;

    for (int j = 0;  j < W; ++j) chmax(res, dp[N][j]);
    cout << res << endl;
    return 0;
}
