#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib> // abs, atoi
#include <string>
#include <array>
#include <cmath>

static const int N = 100100;

using namespace std;

int div_roundout(int a, int b) {
    return (a + b - 1) / b;
}

int main() {
    int d, g;
    array<int, N> p, c;

    cin >> d >> g;
    for (int i = 0; i < d; i++) {
        cin >> p[i] >> c[i];
    }

    int min_np = 2000000000;

    for (int i = 0; i < pow(2, d); i++) {
        int new_np = 0, new_sc = 0;
        // 完全回答の場合
        for (int j = 0; j < d; j++) {
            
        }
    }
}
