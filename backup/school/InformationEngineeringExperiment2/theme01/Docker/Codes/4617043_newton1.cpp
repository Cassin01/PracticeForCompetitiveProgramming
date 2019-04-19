// newton1.cpp : コンソール アプリケーションのエントリ ポイントを定義します。
//

#include "stdafx.h"
#include<stdio.h>
#include<math.h>
#include<stdlib.h>
#include<string.h>
#include<limits.h>
#include <iostream>
#define  eps 0.00000001

using namespace std;

double comp_val(double x[], double M[3][3], double c[]);
void show(double g[], double f_x, int k);
void make_d(double d[], double M[3][3], double x[], double c[], double g[]);
void cmp_dfx(double x[], double g[], double M[3][3], double c[]);
double cmp_complex(double x[]);
double cmp_base(double M[3][3], double x[]);

int _tmain(int argc, _TCHAR* argv[])
{
	double M[3][3];
	double c[3];
	int i, j, k;
	double x[3];
	char fname[128];
	errno_t error;
	FILE *infile;

	// ori
	double d[3];
	double g[3];

	printf("input filename:");
	fgets(fname, sizeof(fname), stdin);
	fname[strlen(fname) - 1] = '\0';
	fflush(stdin);

	if (error = fopen_s(&infile, fname, "r") != 0){ printf("ファイルがありません\n"); }
	else{
		/*ファイルからデータを読み込む*/
		for (i = 1; i <= 2; i++){
			for (j = 1; j <= 2; j++){ fscanf_s(infile, " %lf", &M[i][j]); }
		}
		for (i = 1; i <= 2; i++){ fscanf_s(infile, " %lf", &c[i]); }
		for (i = 1; i <= 2; i++){ fscanf_s(infile, " %lf", &x[i]); }
		fclose(infile);

		/* 主要部分：各自作成*/
		k = 0;
		cmp_dfx(x, g, M, c);

		while (sqrt(pow(g[1], 2) + pow(g[2], 2)) >= eps) {
		    show(g, comp_val(x, M, c), k);

			if (k == 1000) {
				break;
			}

			// dの計算
			if (abs(cmp_base(M, x)) < eps) {
				cout << "逆行列生成できません" << cmp_base(M, x) << endl;
				return 1;
			}
			else {
				make_d(d, M, x, c, g);
			}

			// xの更新
			x[1] = x[1] + d[1];
			x[2] = x[2] + d[2];

			// gの更新
			cmp_dfx(x, g, M, c);
			k = k + 1;	
		}
	}
	return 0;
}

double cmp_complex(double x[]) {
	return (1 + 2 * pow( (x[1] - x[2]), 2))  *  exp(pow((x[1] - x[2]), 2));
}

double dx1dx1(double M[3][3], double x[]) {
	return M[1][1] + 2 * cmp_complex(x);
}

double dx1dx2(double M[3][3], double x[]) {
	return M[1][2] - 2 * cmp_complex(x);
}

double dx2dx2(double M[3][3], double x[]) {
	return M[2][2] + 2 * cmp_complex(x);
}

double cmp_base(double M[3][3], double x[]) {
	return dx1dx2(M, x) * dx1dx1(M, x) - dx2dx2(M, x) * dx1dx2(M, x);
}

void make_d(double d[], double M[3][3], double x[], double c[], double g[]) {
	d[1] = (-1) *        (dx1dx2(M, x) * g[1] - dx1dx2(M, x) * g[2]) / cmp_base(M, x);
	d[2] = (-1) * ((-1) * dx2dx2(M, x) * g[1] + dx1dx1(M, x) * g[2]) / cmp_base(M, x);
	return;
}

/* 点xにおける目的関数値を計算 */
double comp_val(double x[], double M[3][3], double c[]) {
	return (M[1][1] * pow(x[1], 2) + 2 * M[1][2] * x[1] * x[2] + M[2][2] * pow(x[2], 2)) / (double)2 + (c[1] * x[1] + c[2] * x[2]) + exp(pow((x[1] - x[2]), 2));
}

/* 点x, において探索方向d に対するアルミホ探索，g = x 勾配ベクトル */


void cmp_dfx(double x[], double g[], double M[3][3], double c[]) {
	g[1] = (M[1][1] * x[1] + M[1][2] * x[2]) + c[1] + 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]);
	g[2] = (M[2][2] * x[2] + M[1][2] * x[1]) + c[2] - 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]);
	return;
}

void show(double g[], double f_x, int k) {
	cout << "k = " << k << endl;
	cout << "gT = " << "(" << g[1] << ", " << g[2] << ")" << endl;
	cout << "f(x) = " << f_x << endl;
	cout << endl;
	return;
}
