#include <stdio.h>
#include <stdlib.h>
#include <random>
#define gyo 7 //�n�~���O����
#define retu 4 //�n�~���O����
#define k 3 //�V���h���[���̒���
#define SIM 100000 //���s��
#define probability 11 //���オ��

int main(){
	int G[gyo][retu];//�����s��
	int H[gyo][k]; //�����s��
	int w[retu];  //���n��
	int x[gyo];  //���M�n��
	int y[gyo];  //��M�n��
	int e[gyo];  //��蕄��
	int s[gyo]; //�V���h���[������
	double ran;
	int i, j,l,m;
	int tmp;
	int miss;
	double delta_plus = 0.01; //�͂��߂̌�
	double delta;
	double syoki = 0.0025;
	int miss_count;
	double answer;

	//������������
	std::mt19937 mt(41);
	std::uniform_real_distribution<double> r_rand(0.0, 1.0);


	//�����s��ŕK�v�ȔC�ӂɂ��߂�Ƃ���̐���
	G[0][0] = 1; G[0][1] = 0; G[0][2] = 0; G[0][3] = 0;
	G[1][0] = 0; G[1][1] = 1; G[1][2] = 0; G[1][3] = 0;
	G[2][0] = 0; G[2][1] = 0; G[2][2] = 1; G[2][3] = 0;
	G[3][0] = 0; G[3][1] = 0; G[3][2] = 0; G[3][3] = 1;
	G[4][0] = 1; G[4][1] = 1; G[4][2] = 1; G[4][3] = 0;
	G[5][0] = 1; G[5][1] = 1; G[5][2] = 0; G[5][3] = 1;
	G[6][0] = 1; G[6][1] = 0; G[6][2] = 1; G[6][3] = 1;

	H[0][0] = 1; H[0][1] = 1; H[0][2] = 1;
	H[1][0] = 1; H[1][1] = 1; H[1][2] = 0;
	H[2][0] = 1; H[2][1] = 0; H[2][2] = 1;
	H[3][0] = 0; H[3][1] = 1; H[3][2] = 1;
	H[4][0] = 1; H[4][1] = 0; H[4][2] = 0;
	H[5][0] = 0; H[5][1] = 1; H[5][2] = 0;
	H[6][0] = 0; H[6][1] = 0; H[6][2] = 1;

	/*
	//�����m�F
	for (i = 0; i < retu; i++){
	for (j = 0; j < gyo; j++){
	printf("%3d", G[j][i]);
	}
	printf("\n");
	}
	printf("\n\n");

	for (i = 0; i < k; i++){
	for (j = 0; j < gyo; j++){
	printf("%3d", H[j][i]);
	}
	printf("\n");
	}
	printf("\n\n");
	*/
	for (m = 0; m < probability; m++){
		delta = syoki + delta_plus * m;
		miss_count = 0;
		for (l = 0; l < SIM; l++){
			//w�̐���
			for (i = 0; i < retu; i++){
				ran = r_rand(mt);
				if (ran < 0.5){
					w[i] = 0;
				}
				else{
					w[i] = 1;
				}
			}
			//w�̏o��
			/*
			printf("����:");
			for (i = 0; i < retu; i++){
			printf("%3d", w[i]);
			}
			printf("\n");
			*/
			//x�̐���
			for (i = 0; i < gyo; i++){
				tmp = 0;
				for (j = 0; j < retu; j++){
					tmp += w[j] * G[i][j];
				}
				if (tmp % 2 == 0){
					x[i] = 0;
				}
				else{
					x[i] = 1;
				}
			}
			//x�̊m�F
			/*
			printf("������:");
			for (i = 0; i < gyo; i++){
				printf("%3d", x[i]);
			}
			printf("\n");
			*/
			//���e�̐���
			for (i = 0; i < gyo; i++){
				ran = r_rand(mt);
				if (ran < delta){
					e[i] = 1;
				}
				else{
					e[i] = 0;
				}
			}


			/*for (i = 0; i < gyo; i++){
				e[i] = 0;
				}
				printf("��点��͉̂����(1~7)�H:"); scanf("%d", &miss);
				e[miss - 1] = 1;
				*/
			//���M�s��Ɍ��e��������,���M�s�������
			for (i = 0; i < gyo; i++){
				y[i] = (x[i] + e[i]) % 2;
			}
			//y�̊m�F
			/*
			printf("��M��");
			for (i = 0; i < gyo; i++){
			printf("%3d", y[i]);
			}
			printf("\n");
			*/

			//s�̐���
			for (i = 0; i < k; i++){
				s[i] = 0;
				for (j = 0; j < gyo; j++){
					s[i] += y[j] * H[j][i];
				}
				if (s[i] % 2 == 1){
					s[i] = 1;
				}
				else{
					s[i] = 0;
				}
			}
			/*
			printf("�V���h���[��");
			for (i = 0; i < k; i++){
			printf("%3d", s[i]);
			}
			printf("\n");
			*/
			//�ǂ����]�����邩�̔���
			if (s[0] == 1 && s[1] == 1 && s[2] == 1){
				miss = 1;
			}
			else if (s[0] == 1 && s[1] == 1 && s[2] == 0){
				miss = 2;
			}
			else if (s[0] == 1 && s[1] == 0 && s[2] == 1){
				miss = 3;
			}
			else if (s[0] == 0 && s[1] == 1 && s[2] == 1){
				miss = 4;
			}
			else if (s[0] == 1 && s[1] == 0 && s[2] == 0){
				miss = 5;
			}
			else if (s[0] == 0 && s[1] == 1 && s[2] == 0){
				miss = 6;
			}
			else if (s[0] == 0 && s[1] == 0 && s[2] == 1){
				miss = 7;
			}
			else{
				miss = 0;
			}

			/*printf("%d�r�b�g�ڂ𔽓]�����܂�\n", miss);*/

			if (y[miss - 1] == 0){
				y[miss - 1] = 1;
			}
			else{
				y[miss - 1] = 0;
			}
			/*
			printf("����㕄����");
			for (i = 0; i < gyo; i++){
			printf("%3d", y[i]);
			}
			printf("\n");
			printf("������̃r�b�g��萔��0�ł�\n");
			*/
			for (i = 0; i < retu; i++){
				if (y[i] != x[i]){
					miss_count++;
				}
			}
		}
		answer = (double)miss_count / (SIM * retu);
		printf("�Â�%f�̂Ƃ�:%f\n", delta,answer);
	}
}