(1)
a struct Price
b return p
c p
d p

(2)
e &total = p.hamburger * o -> hamburger +
           p.potato    * o -> potato    +
           p.drink     * o -> drink

f p, &o

(3)
g &o.total

h o

(4)
i struct Order *o
j Struct Order *o
k Struct Order *o
l struct Order *o
m o -> hamburger
n Struct Order *o
o o -> potato
p Struct Order *o
q o -> drink
r &o
s &o
t &o

(5)
支払金額: 1100円

-- 200 * 1 + 180 * 2 + 150 * 3
-- 200 + 360 + 450
-- 560 + 450
-- 1100

[2]
(1)
コントラクタ
インスタンス作成時に実行される

(2)
p_hamburger * hamburger +
p_potato    * potato    +
p_drink     * drink

(3)
i num >= 0 && num < 10
u &item = num

(4)
e  &hamburger
o  &potato
ka &drink

(5)
protectedは派生クラスから
参照可能だが
publicは派生クラスから
参照することができない。

副作用を最小限に抑えることができ、
バグがへる。

(6)
virtual は指定する関数が
仮想関数
オーバーライド可能であることを示す意味。

virtual は指定する関数が
仮想関数
オーバーライド可能であることを示す意味で
39行目そのオーバライド
可能な定義であるから

(7)
静的記憶期間を持つ点でことなる。
つまりプログラム終了まで値を保持する。

[3]
(1)



