#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  vector<int> a;
  int b;
  std::cin >> b;
  a.push_back(b);
  cin >> b;
  a.push_back(b);
  cin >> b;
  a.push_back(b);
	if (a[2] == a[0] && a[0] == a[1]) {
		cout << "Yes" << endl;
	} else {
		cout << "No" << endl;
	}
  return 0;
}