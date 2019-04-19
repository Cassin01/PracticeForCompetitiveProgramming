#include <iostream>

using namespace std;

class Order{
  private:
    static int p_hamburger; // ハンバーガーの価格
    static int p_potato;    // ポテトの価格
    static int p_drink;     // ドリンクの価格

    int number;    // 注文No.
    int hamburger; // ハンバーガーの個数
    int potato;    // ポテトの個数
    int drink;     // ドリンクの個数
    int total;     // 支払金額

  public:
    // 適切な関数を設定する
    Order();
    static void Set_Price() {
      p_hamburger = 200;
      p_potato    = 180;
      p_drink     = 150;
    }
    void Set_Number();
    void Cal_Total();
    void Disp_Total();
};

int Order::p_hamburger;
int Order::p_potato;
int Order::p_drink;

void Order::Set_Number() {
  hamburger = 1;
  potato    = 1;
  drink     = 1;
}

Order::Order() {
  hamburger = 0;
  potato    = 0;
  drink     = 0;
  total     = 0;
  return;
}

void Order::Cal_Total() {
  total =
    p_hamburger * hamburger +
    p_potato    * potato    +
    p_drink     * drink;
  return;
}

void Order::Disp_Total() {
  cout <<  "支払金額:" << total << "円 \n";
  return;
}

int  main( void ) {
  Order o;
// [ 各アイテムの価格を設定 ハンバーガー200円、ポテト180円、ドリンク150円 ];
// [ ハンバーガー 1個、ポテト 1個、ドリンク 1個 を設定 ];
// [ 支払額を計算 ];
// [ 支払額を表示 ];
  o.Set_Price();
  o.Set_Number();
  o.Cal_Total();
  o.Disp_Total();
  return 0;
}
