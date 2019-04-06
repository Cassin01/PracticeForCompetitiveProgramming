#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>

using namespace std;

template<class T> inline bool chmax(T& a, T b) { if (a < b) { a = b; return true; } return false; }
template<class T> inline bool chmin(T& a, T b) { if (a > b) { a = b; return true; } return false; }

int N;
long long a[100010][3];

long long dp[100010][3];

int main(int argc, char const *argv[])
{
    int N; cin >> N;
    for (int i = 0; i < N; ++i) for (int j = 0; j < 3; ++j) cin >> a[i][j];

    for(int i = 0; i < N; i++)
    {
        for(int j = 0; j < 3; j++)
        {
            for(int k = 0; k < 3; k++)
            {
                if (j == k) continue;
                chmax(dp[i+1][k], dp[i][j] + a[i][k]);
            }
        }
    }

    long long res = 0;
    for (int j = 0; j < 3; ++j) chmax(res, dp[N][j]);
    cout << res << endl;
    return 0;
}
