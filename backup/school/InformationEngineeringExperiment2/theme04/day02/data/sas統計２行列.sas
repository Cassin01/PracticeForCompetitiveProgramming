proc iml;

reset print; *作成した行列がoutputに出力される;


/* 手順4 */
y = {10, 20, 20, 40, 40, 15};
x = {1 1, 1 2, 1 3, 1 3, 1 5, 1 2};

x0 = j(nrow(y),1,1); *xのベクトル表記2;
x1 = {1,2,3,3,5,2};
x = x0 || x1;

*手順５;
*(a)xの平均;
xmean1 = mean(x1); *xの平均1;
xmean2 = x1[:]; *xの平均2;

*(b)yの平均;
ymean1 = mean(y); *xの平均;
ymean2 = y[:];

*(c)xの偏差平方和;
xTSS = ssq(x1 - xmean1);

*(d)xの偏差平方和;
yTSS = ssq(y - ymean1);

*手順６;
*(a)最小二乗推定量;
b = inv(t(x) * x) * t(x) * y;

*(b)予測値;
yhat = x * b;

*(c)残差;
e = y - yhat;

*手順7;
*射影行列;
H = x * inv(x` * x) * x`; *`は転置の意味;

*(a)対称性;
HT = H`;

*(b)べき等性;
HH = H * H;

*(c)trace(H) = 2;
trH = trace(H);

*手順8(TSSがYTSSと一致しているか確認);
MSS = ssq(yhat - ymean1); *モデル平方和;
RSS = ssq(e); *残差平方和;
TSS = MSS + RSS;

*手順9(寄与率);
R2 = MSS / TSS;
R = sqrt(R2);

quit;
