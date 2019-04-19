#include <stdio.h>

void n2b(int n, int bi[], int bi_len) {
  if (n == 0) {
    return;
  }
  else {
    bi[bi_len++] = n % 2;
    n2b(n / 2, bi, bi_len);
  }
}

void show_bi(int bi[], int n) {
  int i;
  for (i = 0; i < n; i++) {
    printf("%d", bi[i]);
  }
  printf("\n");
}

void init_bi(int bi[], int n) {
  int i;
  for (i = 0; i < n; i++) {
    bi[i] = 0;  
  }
}

void reverse4(int bi[], int n) {
  int tmp[n]; 

  int i;
  for (i = 0; i < n; i++) {
    tmp[i] = bi[i];  
  }
  init_bi(bi, n);
  bi[3] = tmp[0];
  bi[2] = tmp[1];
  bi[1] = tmp[2];
  bi[0] = tmp[3];
}

void add_c(int bi[]) {
  int c_0  = bi[0] ^ bi[1] ^ bi[2];
  int c_1  = bi[1] ^ bi[2] ^ bi[3];
  int c_2  = bi[0] ^ bi[1] ^ bi[3];

  bi[4] = c_0;
  bi[5] = c_1;
  bi[6] = c_2;
}

void cpy_bi2bits(int bits[16][7], int bi[], int i, int n) {
  int j;
  for(j = 0; j < n; j++) {
    bits[i][j] = bi[j];
  }
}

void make_bits(int bits[16][7]) {
  int n = 7;
  int bi[n];
  int i;

  for (i = 0; i < 16; i++) {
    init_bi(bi, n);
    n2b(i, bi, 0);
    reverse4(bi, n);
    add_c(bi);
    cpy_bi2bits(bits, bi, i, n);
  }
}

int main(void) {
  int bits[16][7];
  make_bits(bits);
  int i;
  int n = 7;
  for (i = 0; i < 16; i++) {
    show_bi(bits[i], n);
  }

  return 0;
}
