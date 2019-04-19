#include <iostream>
#include <string>
using namespace std;
int main() {
    int N;
    string A;
    string B;
    string C;

    cin >> N;
    cin >> A;
    cin >> B;
    cin >> C;
    int count=0;

    for (int i = 0; i < N; ++i) {
        if (A[i] == B[i] && A[i] == C[i]) {
        } else if (A[i] == B[i] || B[i] == C[i] || C[i] == A[i]) {
            count = count + 1;
        } else {
            count = count + 2;
        }
    }
    cout << count << endl;
}
