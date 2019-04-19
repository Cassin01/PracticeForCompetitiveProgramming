#include <stdio.h>

unsigned long powme (unsigned long int m, unsigned long int e) {
  if (e == 0) {
    return 1;
  }
  else {
    return m * powme (m, e - 1);
  }
}

void input(unsigned long int* pp, unsigned long int* pq, unsigned long int* pe) {
  printf("３桁以上の出来るだけ大きい素数p ,qを入力してください。\n");
  printf("p: ");
  scanf("%lu", pp);
  printf("q: ");
  scanf("%lu", pq);
  printf("eを入力してください。");
  scanf("%lu", pe);
}

unsigned long int make_d(int e, unsigned long int phi) {
  for (unsigned long int d = 1; d < 1000000; d++) {
    if (e * d % phi == 1) {
      return d;
    }
  }
  return 1;
}

unsigned long int make_c(unsigned long int m, unsigned long int e, unsigned long int n) {
  return powme(m, e) % n;
}

int main(void) {
  // 公開鍵を作成する
  unsigned long int p = 67280421310721;
  unsigned long int q = 2147483647;
  unsigned long int e = 3;
  // input(&p, &q, &e);
  unsigned long int n = p * q;
  unsigned long int phi = (p - 1) * (q - 1);
  unsigned long int d = make_d(e, phi);

  // 暗号化
  int m = 14;
  unsigned long int c = make_c(m, e, n);

  // 復号
  m = powme(c, d) % 15;
  printf("%d\n", m);

  return 0;
}
