#include <stdio.h>
#include <random>

void n2b(int n, int bi[], int bi_len) {
	if (n == 0) {
		return;
	}
	else {
		bi[bi_len++] = n % 2;
		n2b(n / 2, bi, bi_len);
	}
}

void init_bi(int bi[], int n) {
	int i;
	for (i = 0; i < n; i++) {
		bi[i] = 0;
	}
}

void reverse4(int bi[], int n) {
	int tmp[7];

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
	bi[4] = bi[0] ^ bi[1] ^ bi[2]; // c_0
	bi[5] = bi[1] ^ bi[2] ^ bi[3]; // c_1
	bi[6] = bi[0] ^ bi[1] ^ bi[3]; // c_2
}

void cpy_bit2bits(int bits[16][7], int bi[], int i, int n) {
	int j;
	for (j = 0; j < n; j++) {
		bits[i][j] = bi[j];
	}
}

void make_bits(int bits[16][7]) {
	int n = 7;
	int bi[7];
	int i;
	for (i = 0; i < 16; i++) {
		init_bi(bi, n);
		n2b(i, bi, 0);
		reverse4(bi, n);
		add_c(bi);
		cpy_bit2bits(bits, bi, i, n);
	}
}

int hum(int bis[16][7], int y[], int w[]) {
	int min_hum = 100000;
	int min_hum_pattern = 0;
	int i, j;

	for (i = 0; i < 16; i++) {
		int hum_num = 0;

		for (j = 0; j < 7; j++) {
			if (bis[i][j] != y[j]) {
				hum_num++;
			}
		}

		if (min_hum > hum_num) {
			min_hum         = hum_num;
			min_hum_pattern = i;
		}

	}
	return min_hum_pattern;
}

int count_e(int bis[16][7], int w[], int p_num) {
	int i;
	int e_num = 0;
	for (i = 0; i < 4; i++) {
		if (bis[p_num][i] != w[i]) {
			e_num++;
		}
	}
  if (e_num > 0) {
    e_num = 1;
  }
	return e_num;
}

int main(void) {
	std::mt19937 mt(100);
	std::uniform_real_distribution<double> r_rand(0.0, 1.0);

	double ran = r_rand(mt);

	double xi = 0.001;
	int n = 7;
	int k = 4;
	int i;
	int w[7];
	int e[7];
	int y[7];
	int bis[16][7];
	make_bits(bis);

	int sim = 100000;
	int t;
	int all_e = 0;

	int r;
  printf("eps      P_e\n");
	for (r = 0; r < 10; r++) {
		all_e = 0;
		for (t = 0; t < sim; t++) {
			for (i = 0; i < k; i++) {
				if (r_rand(mt) <= 0.5) {
					w[i] = 1;
				}
				else {
					w[i] = 0;
				}
			}

			w[4] = w[0] ^ w[1] ^ w[2]; // c_0
			w[5] = w[1] ^ w[2] ^ w[3]; // c_1
			w[6] = w[0] ^ w[1] ^ w[3]; // c_2

			for (i = 0; i < n; i++) {
				if (r_rand(mt) <= xi) {
					e[i] = 1;
				}
				else {
					e[i] = 0;
				}
			}

			for (i = 0; i < n; i++) {
				y[i] = w[i] ^ e[i];
			}

			// 推定符号語計算
			int min_hum_pattern = hum(bis, y, w);
			all_e = all_e + count_e(bis, w, min_hum_pattern);
		}

		//printf("%f %f\n", xi, (double)all_e / (double)k / sim); // ビット誤り率
		printf("%f %f\n", xi, (double)all_e / sim); // ビット誤り率
		xi = xi + 0.001;
	}
	return 0;
}
