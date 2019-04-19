#include "NewOrder1.h"
#include "NewOrder2.h"
#include <iostream>
using namespace std;

class FinalOrder : public NewOrder1, public NewOrder2 {
  public:
	  virtual void Cal_Total();
    static void Set_Price() {
      NewOrder1::Set_Price();
      NewOrder2::Set_Price();
    };


	// FinalOrder クラスの定義
	
};

void FinalOrder::Cal_Total() {
	Order::Cal_Total();
	total += p_cheeseburger * cheeseburger;
	total += p_fishburger   * fishburger;
}


int  main( void ) {
	// main関数の定義
	// ハンバーガ200円、ドリンク180円、ポテト150円、チーズバーガ250円、フィッシュバーガ300円
	// 全てのアイテムを1個ずつ購入すると1080円
	FinalOrder o;
  FinalOrder::Set_Price();
  o.Set_Hamburger(1);
  o.Set_Potato(1);
  o.Set_Drink(1);
  o.Set_Cheeseburger(1);
  o.Set_Fishburger(1);
  o.Cal_Total();
  o.Disp_Total();
	
	return 0;
}
