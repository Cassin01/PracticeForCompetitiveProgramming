#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

#include <iomanip>
#include <sstream>

using namespace std;

void make_number(int p, int y);

int main(void) {
  int N, M;
  cin >> N >> M;
  vector <vector<int> > PS(N);

  for (int i = 0; i < M; ++i) {
    int p, y;
    cin >> p;
    cin >> y;
    PS[p - 1].push_back(y);
  }
  for (int i = 0; i < PS.size(); ++i) {
    if (!PS.empty()) {
      sort(PS[i].begin(), PS[i].end());
    }
  }
  for (int i = 0; i < PS.size(); ++i) {
    for (int j = 0; j < PS[i].size(); ++j) {
      make_number(i + 1, j + 1);
    }
  }
  return 0;
}

void make_number(int p, int y) {
  std::ostringstream s0, s1;
  s0 << std::setw(6) << std::setfill('0') << p;
  s1 << std::setw(6) << std::setfill('0') << y;
  std::string s(s0.str() + s1.str());
  std::cout << s << endl;
}