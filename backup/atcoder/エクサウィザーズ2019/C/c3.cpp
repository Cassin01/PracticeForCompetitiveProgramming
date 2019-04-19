#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
 
using namespace std;
 
// x番目のゴーレムをk番目のマスに移動させられるかシミュレートして確認する
bool checkLeft(int x, int k, string &s, vector<char> &t, vector<char> &d)
{
    for(int i = 0; i < t.size(); ++i)
    {
        if(t[i] == s[x])
        {
            x += (d[i] == 'L' ? -1 : 1);
        }
 
        if(s[x] == '_')
        {
            // マスの端に到達してもx == kが成り立つとは限らないことに注意する
            return x == k;
        }
    }
 
    return false;
}
 
int main()
{
    int n, q;
    string s;
 
    cin >> n >> q;
    cin >> s;
 
    // 消滅判定を簡単にするためにマスの両端に'_'をくっつけておく
    s = "_" + s + "_";
 
    vector<char > t(q);
    vector<vector<char> > d(2, vector<char>(q));
    for(int i = 0; i < q; ++i)
    {
        cin >> t[i] >> d[0][i];
        // マスの配置を反転させると操作も反転することに注意する
        d[1][i] = (d[0][i] == 'L' ? 'R' : 'L');
    }
 
    int golem = n;
    for(int i = 0; i < 2; ++i)
    {
        // 0マス目に到達する最も右のゴーレムの位置を二分探索する
        int lb = 0, m, ub = n + 1;
        while(ub - lb > 1)
        {
            m = (ub + lb) / 2;
            (checkLeft(m, 0, s, t, d[i]) ? lb : ub) = m;
        }

        // lbの位置にいるゴーレムより左にいるゴーレムは消滅する
        golem -= lb; 

        // 列を反転させてからもう一度同じ操作を行う
        reverse(s.begin(), s.end());
    }
 
    cout << golem << endl;
}
