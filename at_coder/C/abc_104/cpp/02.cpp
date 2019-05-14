#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <array>
#include <cstdlib> // abs, atoi
#include <string>
#include <bitset>

using namespace std;

void sol() {
    int d = 3;
    for (int i = 0; i < pow(2, d); i++) {
        for (int j = 0; j < d; j++ ) {
            // jがiに含まれるもの
            if (i & (1 << j))  {
                cout << (i & (i << j)) << endl;
            }
        }
    }
}

int main(int argc, char const *argv[])
{
    sol();
    return 0;
}
