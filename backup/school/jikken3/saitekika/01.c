#include <stdio.h>
#include <stdlib.h>
#include <math.h>
// -lm オプショん

const int max_len = 10000;
const int max_num = 10000;

int euc2d(int i, int j, int x[max_len], int y[max_len]) {
    double xd, yd;

    xd = x[i] - x[j];
    yd = y[i] - y[j];
    return (int) (sqrt(xd*xd+yd*yd) + 0.5);
}

int min(int x[max_len], int y[max_len], int i, int used[max_len]) {
    int j = 0;
    int min = max_num;
    int min_i = 0;
    for (int j = 0; j < 76; ++j) {
        if (min > euc2d(i, j, x, y) && j != i && used[j] == 0) {
            min = euc2d(i, j, x, y);
            min_i = j;
        }
    }
    return min_i;
}


int main(void) {
    int x[max_len];
    int y[max_len];
    int new_x[max_len];
    int new_y[max_len];
    int used[max_len];
    int ans[max_len];
    for (int i = 0; i < 1000; i++) {
        used[i] = 0;
    }
    int s;
    FILE *f;
    char file_name[] = "01.tsp";

    /* ファイルオープン */
    if ((f = fopen(file_name, "r")) == NULL)
    {
        fprintf(stderr, "%s\n", "error: can't read file.");
        return EXIT_FAILURE;
    }

    int t= 0;
    int i=0;
    while (fscanf(f, "%d", &s) != EOF)
    {
        if (t % 3 == 0) {
        } else if (t % 3 == 1) {
            x[i] = s;
        } else {
            y[i] = s;
            i++;
        }
        t++;
    }

    fclose(f);
    int n = 0;
    for (int i = 0; i < 76; i++) {
        n = min(x, y, n, used);
        used[n] = 1;
        ans[i] = n;
    }
    for (int i = 0; i < 76; i++) {
        printf("%d ", i + 1);
        printf("%d ", x[ans[i]]);
        printf("%d\n", y[ans[i]]);
    }
    printf("%d ", 76);
    printf("%d ", x[ans[0]]);
    printf("%d\n", y[ans[0]]);
    return 0;
}
