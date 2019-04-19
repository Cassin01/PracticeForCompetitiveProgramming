#include <iostream>

using namespace std;

class Order{
  private:
    int p_hamburger; // ハンバーガーの価格
    int p_potato;    // ポテトの価格
    int p_drink;     // ドリンクの価格
  public:
    int number;    // 注文No.
    int hamburger; // ハンバーガーの個数
    int potato;    // ポテトの個数
    int drink;     // ドリンクの個数
    int total;     // 支払金額
    void Set_Price();  // 各アイテムの価格を設定する関数
    void Cal_Total();  // 支払額を計算する関数
    void Disp_Total(); // 支払額を表示する関数
};


void Order::Disp_Total() {
  cout <<  "支払金額:" << total << "円 \n";
}


void Order::Set_Price() {
  p_hamburger = 200;
  p_potato    = 180;
  p_drink     = 150;
  return;
}

void Order::Cal_Total() {
  hamburger = 1;
  potato    = 1;
  drink     = 1;
  total     =
    p_hamburger * hamburger +
    p_potato    * potato    +
    p_drink     * drink;
  return;
}


// [ 各アイテムの価格を関数 Set_Price で設定 ハンバーガー200円、ポテト180円、ドリンク150円 ]
// [ ハンバーガー 1個、ポテト 1個、ドリンク 1個 を変数に設定 ];
// [ 支払額を関数 Cal_Total で計算 ];
// [ 支払額を関数 Disp_Total で表示 ];
int main(void) {
  Order o;
  o.Set_Price();
  o.Cal_Total();
  o.Disp_Total();
  return 0;
}
