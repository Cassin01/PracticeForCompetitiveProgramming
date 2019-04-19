// newton2.cpp : �R���\�[�� �A�v���P�[�V�����̃G���g�� �|�C���g���`���܂��B
//

#include "stdafx.h"
#include<stdio.h>
#include<math.h>
#include<stdlib.h>
#include<string.h>
#include<limits.h>
#include <iostream>

using namespace std;

#define  eps 0.00000001

double comp_val(double x[], double M[3][3], double c[], double A[5][3]);
void show(double g[], double f_x, int k);
void make_d(double d[], double M[3][3], double x[], double c[], double g[], double A[5][3]);
void cmp_dfx(double x[], double g[], double c[], double A[5][3]);
double line_search(double x[], double d[], double alpha, double M[3][3], double c[], double g[], double A[5][3]);
double cmp_base(double x[], double A[5][3]);

int _tmain(int argc, _TCHAR* argv[])
{
	double A[5][3];
	int i, j, k;
	double x[3];
	char fname[128];
	errno_t error;
	FILE *infile;
	
	//Original
	double alpha;
	double d[3];
	double g[3];
	double M[3][3];
	double c[3];

	printf("input filename:");
	fgets(fname, sizeof(fname), stdin);
	fname[strlen(fname) - 1] = '\0';
	fflush(stdin);

	if (error = fopen_s(&infile, fname, "r") != 0){ printf("�t�@�C��������܂���\n"); }
	else {
		/*�t�@�C������f�[�^��ǂݍ���*/
		for (i = 0; i<5; i++){
			for (j = 0; j<3; j++){
				fscanf_s(infile, " %lf", &A[i][j]);
			}
		}
		for (i = 1; i <= 2; i++){ fscanf_s(infile, " %lf", &x[i]); }
		fclose(infile);

		/* ��v�����F�e���쐬*/
		//k������
		k = 0;

		//d������
		cmp_dfx(x, g, c, A);

		while (sqrt(pow(g[1], 2) + pow(g[2], 2)) >= eps) {
			
	   		show(g, comp_val(x, M, c, A), k);

			if (k == 1000) {
				break;
			}


			// d�̍X�V
			if (cmp_base(x, A) < eps) {
				cout << "�t�s�񐶐��ł��܂���" << endl;
				return 1;
			}
			else {
				make_d(d, M, x, c, g, A);
			}

			// x�̍X�V
			x[1] = x[1] + d[1];
			x[2] = x[2] + d[2];
		
			// g�̍X�V
			cmp_dfx(x, g, c, A); 

			// k�̍X�V
			k = k + 1;
		}
	}

	return 0;
}


double dx1dx1(double x[], double A[5][3]) {
	return 12 * A[4][0] * pow(x[1], 2) + 6 * A[3][0] * x[1] + 2 * A[2][0];
}

double dx1dx2(double x[], double A[5][3]) {
	return 2 * A[0][2];
}

double dx2dx2(double x[], double A[5][3]) {
	return A[1][1];
}

double cmp_base(double x[], double A[5][3]) {
	return dx1dx1(x, A) * dx1dx2(x, A) - dx2dx2(x, A) * dx1dx2(x, A);
}

void make_d(double d[], double M[3][3], double x[], double c[], double g[], double A[5][3]) {
	d[1] = (       dx1dx2(x, A) * g[1] - dx1dx2(x, A) * g[2]) / cmp_base(x, A) * (-1);
	d[2] = ((-1) * dx2dx2(x, A) * g[1] + dx1dx1(x, A) * g[2]) / cmp_base(x, A) * (-1);
	return;
}

/* �_x�ɂ�����ړI�֐��l���v�Z */
double comp_val(double x[], double M[3][3], double c[], double A[5][3])
{	
	double fx = A[4][0] * pow(x[1], 4) + A[3][0] * pow(x[1], 3) + A[2][0] * pow(x[1], 2) +
		        A[1][0] * x[1] + A[1][1] * x[1] * x[2] + A[0][2] * pow(x[2], 2) + A[0][1] * x[2];
	//double fx = (M[1][1] * pow(x[1], 2) + 2 * M[1][2] * x[1] * x[2] + M[2][2] * pow(x[2], 2)) / (double)2 + (c[1] * x[1] + c[2] * x[2]) + exp(pow((x[1] - x[2]), 2));
	return fx;
}

/* �_x, �ɂ����ĒT������d �ɑ΂���A���~�z�T���Cg = x ���z�x�N�g�� */

void cmp_dfx(double x[], double g[], double c[], double A[5][3]) {
	g[1] = 4 * A[4][0] * pow(x[1], 3) + 3 * A[3][0] * pow(x[1], 2) + 2 * A[2][0] * x[1] + A[1][0] + A[1][1] * x[2];
	g[2] = A[1][1] * x[1] + 2 * A[0][2] + A[0][1];
	//dfx[1] = (M[1][1] * x[1] + M[1][2] * x[2]) + c[1] + 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]);
	//dfx[2] = (M[2][2] * x[2] + M[1][2] * x[1]) + c[2] + 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]) * (-1);
	return;
}

void show(double g[], double f_x, int k) {
	cout << "k = " << k << endl;
	cout << "gT = " << "(" << g[1] << ", " << g[2] << ")" << endl;
	cout << "f(x) = " << f_x << endl;
	cout << endl;
	return;
}