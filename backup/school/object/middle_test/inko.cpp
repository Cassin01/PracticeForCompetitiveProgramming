#include <iostream>
#include <vector>

using namespace std;

int check(vector <int>sums) {
  if (sums.size() == 0) {
    return -1;
  } else {
    int sum = sums.back();
    sums.pop_back();
    for (int i = 0; i < sums.size(); ++i) {
      if (sum == sums[i]) {
        return sum;
      }
    }
    return check(sums);
  }
}

void solution(vector<int>sums, vector<int> as, vector<int> bs, vector<int>cs, int num) {
  cout << "答えは" << endl;
  for (int i = 0; i < sums.size(); ++i) {
    if (num == sums[i]) {
      if (as[i] < cs[i] && bs[i] < cs[i]) {
        cout << as[i] << " " << bs[i] << " " << cs[i] << endl;
      }
    }
  }
}

int main(void) {
  vector<int> sums;
  vector<int> as;
  vector<int> bs;
  vector<int> cs;

  for (int a = 1; a <= 36; ++a) {
    for (int b = a; b <= 36; ++b) {
      for (int c = b; c <= 36; ++c) {
        if (a * b * c == 36) {
          sums.push_back(a + b + c);
          as.push_back(a);
          bs.push_back(b);
          cs.push_back(c);
        }
      }
    }
  }

  if (check(sums) == -1) {
    cout << "解は一様に存在します。" << endl;
  } else {
    cout << "解は複数存在します。" << endl;
  }

  solution(sums, as, bs, cs, check(sums));

  return 0;
}
