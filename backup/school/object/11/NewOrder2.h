#ifndef _INC_NEWORDER2    //まだ読み込まれていなければ以下の処理をする
#define _INC_NEWORDER2

#include "Order.h"
#include <iostream>
using namespace std;


class NewOrder2: virtual public Order {
  protected:
	static int p_fishburger;
	int fishburger;
	
  public:
	NewOrder2();
	void Set_Fishburger( int num );
	virtual void Cal_Total();
	static void Set_Price() {
		Order::Set_Price();
		p_fishburger = 300;
	};
};

NewOrder2::NewOrder2() {
	fishburger = 0;
}

void NewOrder2::Cal_Total() {
	Order::Cal_Total();
	total += p_fishburger * fishburger;
}

void NewOrder2::Set_Fishburger( int num ) {
	Input( &fishburger, num );
}

int NewOrder2::p_fishburger;

#endif
