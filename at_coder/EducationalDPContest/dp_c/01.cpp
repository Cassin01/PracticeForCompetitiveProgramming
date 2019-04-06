#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>

using namespace std;

const long long INF = 1LL << 60;

long long abcs[3][100010];

long long dp[100010];

int max_index(long long max3, unsigned int j) {
        for(int i = 0; i < 3; i++)
        {
            if (max3 == abcs[i][j]) {
                return i;
            }
        }
        return 8;
}


int main(int argc, char const *argv[])
{
    for (unsigned int i = 0; i < 100010; i++)
    {
        dp[i] = INF;
    }

    int N;
    cin >> N;
    for (unsigned int j = 0; j < N; j++)
    {
        for (int i = 0; i < 3; i++)
        {
            cin >> abcs[i][j];
        }
    }


    dp[0] = 0;
    int used = -1;
    for (unsigned int i = 1; i <= N; i++)
    {
            if (used == 0) {
                long long ma = max(abcs[1][i - 1], abcs[2][i - 1]);
                dp[i] = dp[i - 1] + ma;
                used = max_index(ma, i - 1);
            } else if (used == 1) {
                long long ma = max(abcs[0][i - 1], abcs[2][i - 1]);
                dp[i] = dp[i - 1] + ma;
                used = max_index(ma, i - 1);
            } else if (used == 2) {
                long long ma = max(abcs[0][i - 1], abcs[1][i - 1]);
                dp[i] = dp[i - 1] + ma;
                used = max_index(ma, i - 1);
            } else {
                long long ma = max(max(abcs[0][i - 1], abcs[1][i - 1]), abcs[2][i - 1]);
                dp[i] = dp[i - 1] + ma;
                used = max_index(ma, i - 1);
            }
    }
    cout << dp[N] << endl;

    return 0;
}
