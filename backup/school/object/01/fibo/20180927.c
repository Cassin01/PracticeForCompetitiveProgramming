#include <stdio.h>

long fibo (int n) {
  long f1 = 1;
  long f0 = 0;
  long f2;
  int i;
  for (i = n - 1 ; i > 0; --i) {
    f2 = f0 + f1;
    f0 = f1;
    f1 = f2;
  }
  return f2;
}

int main(void) {
  int k;
  printf("Please input number:");
  scanf("%d", &k);
  printf("Fibo(%d)=%ld\n", k, fibo(k));
  return 0;
}
