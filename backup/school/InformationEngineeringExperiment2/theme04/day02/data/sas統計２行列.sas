proc iml;

reset print; *�쐬�����s��output�ɏo�͂����;


/* �菇4 */
y = {10, 20, 20, 40, 40, 15};
x = {1 1, 1 2, 1 3, 1 3, 1 5, 1 2};

x0 = j(nrow(y),1,1); *x�̃x�N�g���\�L2;
x1 = {1,2,3,3,5,2};
x = x0 || x1;

*�菇�T;
*(a)x�̕���;
xmean1 = mean(x1); *x�̕���1;
xmean2 = x1[:]; *x�̕���2;

*(b)y�̕���;
ymean1 = mean(y); *x�̕���;
ymean2 = y[:];

*(c)x�̕΍������a;
xTSS = ssq(x1 - xmean1);

*(d)x�̕΍������a;
yTSS = ssq(y - ymean1);

*�菇�U;
*(a)�ŏ���搄���;
b = inv(t(x) * x) * t(x) * y;

*(b)�\���l;
yhat = x * b;

*(c)�c��;
e = y - yhat;

*�菇7;
*�ˉe�s��;
H = x * inv(x` * x) * x`; *`�͓]�u�̈Ӗ�;

*(a)�Ώ̐�;
HT = H`;

*(b)�ׂ�����;
HH = H * H;

*(c)trace(H) = 2;
trH = trace(H);

*�菇8(TSS��YTSS�ƈ�v���Ă��邩�m�F);
MSS = ssq(yhat - ymean1); *���f�������a;
RSS = ssq(e); *�c�������a;
TSS = MSS + RSS;

*�菇9(��^��);
R2 = MSS / TSS;
R = sqrt(R2);

quit;
