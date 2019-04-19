// quasinewton1.cpp : コンソール アプリケーションのエントリ ポイントを定義します。
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

double M[3][3];
double c[3];

/* 点xにおける目的関数値を計算 */
//double comp_val(double x[]);
double cmp_val(double x[], double M[3][3], double c[]);

/* 点x, において探索方向d に対するアルミホ探索，g = x 勾配ベクトル */
//double line_search(double x[], double g[], double d[]);
double line_search(double t, double x[], double d[], double alpha, double M[3][3], double c[], double g[]);


void show(double g[], double f_x, int k);
void init_B(double B[3][3]);
void cmp_g(double x[], double g[], double M[3][3], double c[]);
void cmp_pg(double pg[], double p[]);
void cmp_px(double px[], double x[]);
void cmp_B(double B[3][3], double x[], double px[], double g[], double pg[], double M[3][3], double c[]);
void cmp_d(double d[], double g[], double B[3][3]);

int _tmain(int argc, _TCHAR* argv[])
{
	int i, j, k;
	double x[3];
	char fname[128];
	errno_t error;
	FILE *infile;

	// original
	double alpha = 1;
	double B[3][3];
	double g[3];
	double d[3];

	double px[3];
	double pg[3];

	printf("input filename:");
	fgets(fname, sizeof(fname), stdin);
	fname[strlen(fname) - 1] = '\0';
	fflush(stdin);

	if (error = fopen_s(&infile, fname, "r") != 0){ printf("ファイルがありません\n"); }
	else {
		/*ファイルからデータを読み込む*/
		for (i = 1; i <= 2; i++){
			for (j = 1; j <= 2; j++){ fscanf_s(infile, " %lf", &M[i][j]); }
		}
		for (i = 1; i <= 2; i++){ fscanf_s(infile, " %lf", &c[i]); }
		for (i = 1; i <= 2; i++){ fscanf_s(infile, " %lf", &x[i]); }
		fclose(infile);

		/* 主要部分：各自作成*/
		init_B(B);

		// k, g 初期化
		k = 0;
		cmp_g(x, g, M, c);


		while (sqrt(pow(g[1], 2) + pow(g[2], 2)) >= eps) {
			show(g, cmp_val(x, M, c), k);

			if (k == 1000) {
				break;
			}

			// g の更新
			cmp_g(x, g, M, c);
		
			// d の更新
			if (k == 0) {
				cmp_d(d, g, B);
			}
			else {
				cmp_B(B, x, px, g, pg, M, c);
				cmp_d(d, g, B);
			}

			alpha = line_search(0, x, d, 1, M, c, g);

			// pg, px の更新
			cmp_pg(pg, g);
			cmp_px(px, x);

			// xの更新
			x[1] = x[1] + alpha * d[1];
			x[2] = x[2] + alpha * d[2];

			// k の更新
			k = k + 1;
		}
		cout << "計算終了しました。" << endl;
	}
	return 0;
}

/* 点xにおける目的関数値を計算 */
double cmp_val(double x[], double M[3][3], double c[]) {
	return (M[1][1] * pow(x[1], 2) + 2 * M[1][2] * x[1] * x[2] + M[2][2] * pow(x[2], 2)) * 0.5 + (c[1] * x[1] + c[2] * x[2]) + exp(pow((x[1] - x[2]), 2));
}

void init_B(double B[3][3]) {
	B[1][1] = 1;
	B[1][2] = 0;
	B[2][1] = 0;
	B[2][2] = 1;
}

/* 点x, において探索方向d に対するアルミホ探索，g = x 勾配ベクトル */
double line_search(double t, double x[], double d[], double alpha, double M[3][3], double c[], double g[])
{
	double xk[3];
	xk[1] = x[1] + alpha * d[1];
	xk[2] = x[2] + alpha * d[2];
	double kusi = 0.25;
	double tau = 0.5;
	double left;
	double right;
	left = cmp_val(xk, M, c);
	right = cmp_val(x, M, c) + kusi * alpha * (g[1] * d[1] + g[2] * d[2]);
	if (left <= right || t >= 1000) {
		return alpha;
	}
	else {
		return line_search(t + 1, x, d, alpha * tau, M, c, g);
	}
}

void cmp_g(double x[], double g[], double M[3][3], double c[]) {
	g[1] = (M[1][1] * x[1] + M[1][2] * x[2]) + c[1] + 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]);
	g[2] = (M[2][2] * x[2] + M[1][2] * x[1]) + c[2] - 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]);
}

void cmp_pg(double pg[], double g[]) {
	pg[1] = g[1];
	pg[2] = g[2];
} 

void cmp_px(double px[], double x[]) {
	px[1] = x[1];
	px[2] = x[2];
}

void cmp_y(double y[], double g[], double pg[]) {
	y[1] = g[1] - pg[1];
	y[2] = g[2] - pg[2];
}

void  cmp_s(double s[], double x[], double px[]) {
	s[1] = x[1] - px[1];
	s[2] = x[2] - px[2];
}

void cmp_Bs(double Bs[], double B[3][3], double s[]) {
	Bs[1] = B[1][1] * s[1] + B[1][2] * s[2];
	Bs[2] = B[2][1] * s[1] + B[2][2] * s[2];
}

double cmp_right(double y[], double s[]) {
	return (pow(y[1], 2) + pow(y[2], 2)) / (s[1] * y[1] + s[2] * y[2]);
}

double cmp_mid(double Bs[], double s[]) {
	return (pow(Bs[1], 2) + pow(Bs[2], 2)) / (s[1] * Bs[1] + s[2] * Bs[2]);
}

void cmp_B(double B[3][3], double x[], double px[], double g[], double pg[], double M[3][3], double c[]) {
	double Bs[3];
	double s[3];
	double y[3];

	double mid;
	double right;

	cmp_s(s, x, px);
	cmp_y(y, g, pg);
	cmp_Bs(Bs, B, s);

	mid   = cmp_mid(Bs, s);
	right = cmp_right(y, s);

	B[1][1] = B[1][1] - mid + right;
	B[2][2] = B[2][2] - mid + right;
}

double cmp_BT_base(double B[3][3]) {
	return B[1][1] * B[2][2] - B[1][2] * B[2][1];
}

void cmp_BT(double BT[3][3], double B[3][3]) {
	double tmpB[3][3];
	tmpB[1][1] = B[1][1];
	tmpB[1][2] = B[1][2];
	tmpB[2][1] = B[2][1];
	tmpB[2][2] = B[2][2];

	BT[1][1] = tmpB[2][2] / cmp_BT_base(tmpB);
	BT[2][2] = tmpB[1][1] / cmp_BT_base(tmpB);
	BT[1][2] = 0;
	BT[2][1] = 0;
	cout << BT[1][1] << BT[1][2] << endl;
	cout << BT[2][1] << BT[2][2] << endl;
}

void cmp_d(double d[], double g[],  double B[3][3]) {
	double BT[3][3];
	cmp_BT(BT, B);

	d[1] = (-1) * (BT[1][1] * g[1] + BT[1][2] * g[2]);
	d[2] = (-1) * (BT[2][1] * g[1] + BT[2][2] * g[2]);
}

void show(double g[], double f_x, int k) {
	cout << "k = "    << k   << endl;
	cout << "gT = "   << "(" << g[1] << ", " << g[2] << ")" << endl;
	cout << "f(x) = " << f_x << endl;
	cout << endl;
}