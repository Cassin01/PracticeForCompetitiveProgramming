#include <iostream>
#include <vector>

using namespace std;

vector<int> num2bit (int n);

int main(void) {

  vector<int> bits = num2bit (3);
  for (int i = 0; i < bits.size(); ++i) {
    cout << bits[i] << endl;
  }
  return 0;
}

vector<int> num2bit (int n) {
    vector<int> bits;
    while (n > 0) {
        bits.push_back(n % 2);
       n = n / 2;
    }
    std::reverse(bits.begin(), bits.end());
    return bits;
}
