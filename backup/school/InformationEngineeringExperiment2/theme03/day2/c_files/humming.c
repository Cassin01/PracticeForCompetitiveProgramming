#include <stdio.h>
#include <stdlib.h>

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

int show_start(int w[]) {
  int i;

  printf("符号語: ");
  for (i = 0 ; i < 7; i++) {
    printf("%d", w[i]);
  }
  printf("\n");

  int input_num;
  printf("誤りを付加させるのは7ビット中何ビット目?(1 ~ 7)\n");
  scanf("%d", &input_num);
  int e_num = input_num - 1;

  return e_num;
}

void show_received(int y[], int n) {
  printf("受信語: ");
  int i;
  for (i = 0; i < n; i++) {
    printf("%d", y[i]);
  }
  printf("\n");
}

void show_decoded(int min_hum_pattern, int bis[16][7], int min_hum) {
  printf("%d番目の符号語がハミング距離最小です\n", min_hum_pattern);

  printf("推定符号語\n");
  int i;
  for (i = 0; i < 7; i++) {
    printf("%d", bis[min_hum_pattern][i]);
  }
  printf("\n");

  printf("復号後のビット誤り数は%dです\n", min_hum);
}

void hum(int bis[16][7], int y[]) {
  int min_hum = 100000;
  int min_hum_pattern = 0;
  int i, j;

  printf("符号語 ハミング距離\n");
  for (i = 0; i < 16; i++) {
    int hum_num = 0;

    // ハミング距離計算
    for (j = 0; j < 7; j++) {
      if (bis[i][j] != y[j]) {
        hum_num++;
      }
    }

    if (min_hum > hum_num) {
      min_hum         = hum_num;
      min_hum_pattern = i;
    }

    // バイナリパターン表示
    printf("%2d) ", i);
    for (j = 0; j < 7; j++) {
      printf("%d", bis[i][j]);
    }
    printf(" ");

    // ハミング距離表示
    printf("%d", hum_num);
    printf("\n");
  }

  // 復号結果表示
  show_decoded(min_hum_pattern, bis, min_hum);
}


int main(void) {

  int bis[16][7];
  make_bits(bis);


  srand(100);
  double ran = rand();

  int n = 7;
  int k = 4;

  int w[n];
  int e[n];
  int y[n];

  int i;

  for (i = 0; i < k; i++) {
    double rnd = rand();
    if (rnd / RAND_MAX <= 0.5) {
      w[i] = 1;
    }
    else {
      w[i] = 0;
    }
  }

  int c_1 = w[0] ^ w[1] ^ w[2];
  int c_2 = w[1] ^ w[2] ^ w[3];
  int c_3 = w[0] ^ w[1] ^ w[3];

  w[4] = c_1;
  w[5] = c_2;
  w[6] = c_3;

  // エラー番号取得
  int e_num = show_start(w);

  for (i = 0; i < n; i++) {
    if (i == e_num) {
      e[i] = 1;
    }
    else {
      e[i] = 0;
    }
  }

  for (i = 0; i < n; i++) {
    y[i] = w[i] ^ e[i];
  }

  //受信結果表示
  show_received(y, n);

  // 推定符号語計算
  hum(bis, y);

  return 0;
}
