#include <iostream>
#include <vector>
#include <string>

using namespace std;

int main(void) {
  int N;
  cin >> N;

  int Atmp;
  vector<int> A;

  for (int i = 0; i < N; i++) {
    cin >> Atmp;
    A.push_back(Atmp);
  }



  return 0;
}

vector<int> serch(vector<int> A) {
  int sum = 0;
  for (int i = 0; i < A.size(); i++) {
    sum = sum + A[i];
  }
  return A;
}
