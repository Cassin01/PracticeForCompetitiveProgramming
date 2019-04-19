#include <stdio.h>
#include <stdlib.h>

int main(void) {
  double ran;

  srand(100);

  ran = rand();

  int w[4];
  int e[4];
  int y[4];

  int i;
  double xi = 0.00001;
  double rnd;

  int j;
  double all_err = 0;

  int k = 4;
  int sim = 100000;
  int t;


  for (int j = 0; j < 10; j++) {
    for (int t = 0; t < sim; t++) {

      for (i = 0; i < k; i++) {
        rnd = rand();
        if (rnd / RAND_MAX <= xi) {
          w[i] = 1;
        }
        else {
          w[i] = 0;
        }
      }


      for (i = 0; i < k; i++) {
        rnd = rand();
        if (rnd / RAND_MAX <= xi) {
          e[i] = 1;
        }
        else {
          e[i] = 0;
        }
      }

      y[0] = w[0] ^ e[0];
      y[1] = w[1] ^ e[1];
      y[2] = w[2] ^ e[2];
      y[3] = w[3] ^ e[3];

      double err = 0;
      for (i = 0; i < k; i++) {
        if (y[i] != w[i]) {
          err++;
        }
      }
      all_err = all_err + err;
    }
    double p_e = all_err / (k * sim);
    printf("p_e = %f, xi = %f\n", p_e, xi);
    xi = xi + 0.00001;
    all_err = 0;
  }

  return 0;
}
