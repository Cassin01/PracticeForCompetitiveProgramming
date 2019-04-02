#include <iostream>
#include <vector>
#include <string>
using namespace std;
int main(void)
{
  string S, T;
  cin >> S >> T;
  vector<int> start(26, -1);
  vector<int> goal(26, -1);
  for (int i = 0; i < S.size(); i++)
  {
    int a = S[i] - 'a';
    cout << a << endl;
    int b = T[i] - 'a';
    if (start[a] != -1 || goal[b] != -1) {
      if (start[a] != b || goal[b] != a) {
        cout << "No" << endl;
        return 0;
      }
    } else {
      start[a] = b;
      goal[b] = a;
    }
  }
  cout << "Yes" << endl;
}