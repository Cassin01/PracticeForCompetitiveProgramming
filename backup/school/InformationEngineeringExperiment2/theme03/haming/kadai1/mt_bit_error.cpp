#include <stdio.h>
#include <stdlib.h>
#include <random>
#define SIM 1000000
#define K 4
#define probability 11
#define delta_plus 0.001

int main(){
	int miss_count;
	int Transmission[K];
	int Reception[K];
	int noise[K];
	double ran;
	int i;
	int j;
	int k;
	double delta = 0;
	std::mt19937 mt(100);
	std::uniform_real_distribution<double> r_rand(0.0, 1.0);

	for (k = 1; k < probability; k++){
		miss_count = 0;
		delta = delta + delta_plus;
		for (j = 0; j < SIM; j++){
			//printf("Transmission:");
			//乱数４つ生成:送るデータのやつ
			for (i = 0; i < K; i++){
				ran = r_rand(mt);
				if (ran < 0.5){
					Transmission[i] = 0;
				}
				else{
					Transmission[i] = 1;
				}
				//printf("%d  ", Transmission[i]);
			}
			//printf("\n");
			//雑音の乱数４つ
			for (i = 0; i < K; i++){
				ran = r_rand(mt);
				if (ran < delta){
					noise[i] = 1;
				}
				else{
					noise[i] = 0;
				}
			}

			//送られてくるデータ
			//printf("Reception   :");
			for (i = 0; i < K; i++){
				if (noise[i] == 0){
					Reception[i] = Transmission[i];
				}
				else{
					Reception[i] = abs(Transmission[i] - 1);
				}
				//printf("%d  ", Reception[i]);

				if (Reception[i] != Transmission[i]){
					miss_count++;
				}
			}
			//printf("\n\n");
		}
		printf("deltaが%fのときのビット誤り率は%f\n", delta, (double)miss_count / (double)(K * SIM));
	}
	return 0;
}