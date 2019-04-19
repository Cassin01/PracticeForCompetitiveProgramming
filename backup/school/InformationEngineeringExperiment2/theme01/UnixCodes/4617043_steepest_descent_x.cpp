// steepest_descent.cpp : コンソール アプリケーションのエントリ ポイントを定義します。

//#include "stdafx.h"
#include <cstdio>
#include <cstdlib>
#include<stdio.h>
#include<iostream>
#include<math.h>
#include<stdlib.h>
#include<string.h>
#include<limits.h>
#define  eps 0.000001

using namespace std;

double M[3][3];
double c[3];

void cmp_dfx(double x[], double dfx[], double M[3][3], double c[]);

/* 点xにおける目的関数値を計算 */
double comp_val(double x[], double M[3][3], double c[]);


/* 点x, において探索方向d に対するアルミホ探索，g = x 勾配ベクトル */
double line_search(double x[], double d[], double alpha, double M[3][3], double c[], double g[]);

void show(double g[], double f_x, int k);
void show2(double v);
void show3(double x[]);

//int _tmain(int argc, _TCHAR* argv[])
int main(void) {
  int i, j, k;
  //int cont;
  double alpha;
  //double val, norm, expval;
  // double newx[3];
  double x[3],  d[3], g[3];
  char fname[128];
  errno_t error;
  FILE *infile;


  printf("input filename:");
  fgets(fname, sizeof(fname), stdin);
  fname[strlen(fname) - 1] = '\0';
  fflush(stdin);

  //if (error = fopen_s(&infile, fname, "r") != 0){ printf("ファイルがありません\n"); }
  infile = std::fopen(fname, "r");
  if(!infile) {
    std::perror("File opening failed");
    return EXIT_FAILURE;
  }
  else {
    /*ファイルからデータを読み込む*/
    for (i = 1; i <= 2; i++){
      for (j = 1; j <= 2; j++){ fscanf(infile, " %lf", &M[i][j]); }
    }
    for (i = 1; i <= 2; i++){ fscanf(infile, " %lf", &c[i]); }
    for (i = 1; i <= 2; i++){ fscanf(infile, " %lf", &x[i]); }
    fclose(infile);

    /* 主要部分：各自作成*/
    k = 0;
    cmp_dfx(x, g, M, c);

    //cout << (pow(g[1], 2) + pow(g[2], 2)) << endl;
    while (sqrt(pow(g[1], 2) + pow(g[2], 2)) >= eps) {
      //cout << sqrt(pow(g[1], 2) + pow(g[2], 2)) << endl;
      //cout << k << " " << d[1] << ", " << d[2] << endl;
      //show(g, comp_val(x, M, c), k);
      //show2(comp_val(x, M, c));
      show3(x);

      d[1] = (-1) * g[1];
      d[2] = (-1) * g[2];// dの計算
      alpha = line_search(x, d, 1, M, c, g);
      x[1] = x[1] + alpha * d[1];
      x[2] = x[2] + alpha * d[2];
      cmp_dfx(x, g, M, c); // gの更新
      k = k + 1;
      if (k == 1000) {
        break;
      }
    }
    //	std::cout << alpha << std::endl;
  }

  return 0;
}

/* 点xにおける目的関数値を計算 */
double comp_val(double x[], double M[3][3], double c[])
{
  double fx = (M[1][1] * pow(x[1], 2) + 2 * M[1][2] * x[1] * x[2] + M[2][2] * pow(x[2], 2)) / (double)2 + (c[1] * x[1] + c[2] * x[2]) + exp(pow((x[1] - x[2]), 2));
  return fx;
}

/* 点x, において探索方向d に対するアルミホ探索，g = x 勾配ベクトル */
double line_search(double x[], double d[], double alpha, double M[3][3], double c[], double g[])
{
  // std::cout << alpha << std::endl;
  //cout << "alpha" << alpha << endl;
  double xk[3];
  xk[1] = x[1] + alpha * d[1];
  xk[2] = x[2] + alpha * d[2];
  double kusi = 0.25;
  double tau = 0.5;
  double left;
  double right;
  left = comp_val(xk, M, c);
  right = comp_val(x, M, c) + kusi * alpha * (g[1] * d[1] + g[2] * d[2]);
  if (left <= right) {
    return alpha;
  }
  else {
    return line_search(x, d, alpha * tau, M, c, g);
  }
}

void cmp_dfx(double x[], double dfx[], double M[3][3], double c[]) {
  dfx[1] = (M[1][1] * x[1] + M[1][2] * x[2]) + c[1] + 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]);
  dfx[2] = (M[2][2] * x[2] + M[1][2] * x[1]) + c[2] + 2 * exp(pow(x[1] - x[2], 2)) * (x[1] - x[2]) * (-1);
  return;
}

void show(double g[], double f_x, int k) {
  cout << "k = " << k << endl;
  cout << "gT = " << "(" << g[1] << ", " << g[2] << ")" << endl;
  cout << "f(x) = " << f_x << endl;
  cout << endl;
  return;
}

void show2(double v) {
  cout << v << endl;
}

void show3(double x[]) {
  cout << x[2] << endl;
}
