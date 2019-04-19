#include <iostream>
#include <vector>

using namespace std;

int main() {
    int N;
    cin >> N;
    int A[N];
    int B[N];
    bool eatp[N];
    vector<int> pre_eat;
    for (int i = 0; i < N; i++) {
        cin >> A[i];
        cin >> B[i];
        eatp[i] = true;
    }

    long long int ap = 0;
    long long int bp = 0;
    vector<int> eat;
    int eated = 0;

    while (eated < N) {
        int amax = A[0];
        for (int i = 0; i < N; ++i) {
            if (eatp[i]) {
                if (amax < A[i]) {
                    eat.clear();
                    eat.push_back(i);
                    amax = A[i];
                } else if (amax == A[i]) {
                    eat.push_back(i);
                }
            }
        }
        if (eat.size() > 1) {
            int b_i = 0;
            int bmax_tmp = B[eat[0]];
            for (int i = 0; i < eat.size(); i++) {
                if (bmax_tmp < B[eat[i]]) {
                    bmax_tmp = B[eat[i]];
                    b_i = eat[i];
                }
            }
            eated++;
            eatp[b_i] = false;
            ap = ap + A[b_i];
        } else {
            eated++;
            eatp[eat[0]] = false;
            ap = ap + A[eat[0]];
        }

        eat.clear();
        if (eated < N) {
            int bmax = B[0];
            for (int i = 0; i < N; ++i) {
                if (eatp[i]) {
                    if (bmax < B[i]) {
                        eat.clear();
                        eat.push_back(i);
                        bmax = B[i];
                    } else if (bmax == B[i]) {
                        eat.push_back(i);
                    }
                }
            }

            if (eat.size() > 1) {
                int a_i = 0;
                int amax_tmp = A[eat[0]];
                for (int i = 0; i < eat.size(); i++) {
                    if (amax_tmp < A[eat[i]]) {
                        amax_tmp = A[eat[i]];
                        a_i = eat[i];
                    }
                }
                eated++;
                eatp[a_i] = false;
                bp = bp + B[a_i];
            } else {
                eated++;
                eatp[eat[0]] = false;
                bp = bp + B[eat[0]];
            }
        }
    }
    cout << ap - bp << endl;
    return 0;
}