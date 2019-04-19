#include <iostream>
using namespace std;

class Pupil {
  protected:
    static int num;
    int StudentID;
    int Jap;
    int Math;
    int Sci;
    int Soc;
  public:
    virtual void DispID();
    virtual int SetScore();
    virtual void SetJap();
    virtual void SetMath();
    virtual void SetSci();
    virtual void SetSoc();
    virtual void SetJap(int);
    virtual void SetMath(int);
    virtual void SetSci(int);
    virtual void SetSoc(int);
    virtual void DispScore();
    virtual void AverageScore();
    Pupil();
    ~Pupil();
};

int Pupil::num = 0;

Pupil::Pupil() {
  StudentID = ++num;
  Jap = 0;
  Math = 0;
  Sci = 0;
  Soc = 0;
}

Pupil::~Pupil() {
  --num;
}

void Pupil::DispID() {
  cout << "StudentID:" << StudentID << endl;
}

int Pupil::SetScore() {
  int score;
  do {
    cout << "Please input score:";
    cin >> score;
  } while (score < 0 || score > 100);
  return score;
}

void Pupil::SetJap() {
  Jap = SetScore();
}

void Pupil::SetJap(int score) {
  Jap = score;
}

void Pupil::SetMath() {
  Math = SetScore();
}


void Pupil::SetMath(int score) {
  Math = score;
}

void Pupil::SetSci() {
  Sci = SetScore();
}

void Pupil::SetSci(int score) {
  Sci = score;
}

void Pupil::SetSoc() {
  Soc = SetScore();
}

void Pupil::SetSoc(int score) {
  Soc = score;
}

void Pupil::DispScore() {
  cout << "Mathematics:" << Math << endl;
  cout << "Japanese:"    << Jap  << endl;
  cout << "Science:"     << Sci  << endl;
  cout << "Social:"      << Soc  << endl;
}

void Pupil::AverageScore() {
  double average = 0.0;
  average = (double)Math + (double)Jap + (double)Sci + (double)Soc;
  average /= 4.0;
  cout << "Average:" << average << endl;
}

int main(void) {
  Pupil pupil[10];
  for (int k = 0; k < 10; k++) {
    pupil[k].DispID();
  }
}
/*
*/

/*
int main(void) {
  Pupil pupil[10];
  pupil[5].SetJap(50);
  pupil[5].SetMath(80);
  pupil[5].SetSci(70);
  pupil[5].SetSoc();
  pupil[5].SetJap();
  pupil[5].DispScore();
  pupil[5].AverageScore();

  return 0;
}
*/

/*
(1)
  コンストラクタ
    オブジェクト作成時に初期化をする関数

    クラス内にクラス名と同じ名前の関数を用意すると
    オブジェクトが生成される際に自動実行される

(2)
  デストラクタ
    スタック上に作成されたオブジェクトが
    自動的に破棄されるときや、
    ヒープ上に作成されたオブジェクトを
    delete 演算子により明示的に破棄するときに、
    呼び出される関数


    クラス内にクラス名の先頭に「～」を付けた関数を
    用意するとオブジェクトがメモリから破棄される際に
    呼び出される

(3)
  静的メンバ変数
    クラスのオブジェクトに属するメンバではなくクラス
    そのものの属するメンバ
(4)
実行結果は以下のようになる。
  Please input score:2
  Please input score:3
  Mathematics:80
  Japanese:3
  Science:70
  Social:2
  Average:38.75

Pupilクラスからpupilオブジェクトが10個配列で生成され
その際各教科は0点で初期化される。そのうち
配列番号5番のオブジェクトのみ五教科の点数が再設定される。
この時社会と日本語の引数がないため標準入力から値を受け取る
そしてDispScore関数でそれぞれの値を出力し、AverageScore関数で
平均値を出力する。

(6)
  実行結果は以下のようになる。
  StudentID:1
  StudentID:2
  StudentID:3
  StudentID:4
  StudentID:5
  StudentID:6
  StudentID:7
  StudentID:8
  StudentID:9
  StudentID:10


Pupilクラスから10個のpupiloオブジェクトが配列として生成される
さいpupilコントラクタが呼ばれ、
そのとき静的メンバ変数numが前置インクリメントされStudentIDに
代入されるため。

(7)
*/
