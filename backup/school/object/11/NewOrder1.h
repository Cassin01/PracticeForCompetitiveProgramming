#ifndef _INC_NEWORDER1    //まだ読み込まれていなければ以下の処理をする
#define _INC_NEWORDER1

#include "Order.h"
#include <iostream>
using namespace std;

class NewOrder1: virtual public Order {
  protected:
	static int p_cheeseburger;
	int cheeseburger;
	
  public:
	NewOrder1();
	void Set_Cheeseburger( int num );
	virtual void Cal_Total();
	static void Set_Price() {
		Order::Set_Price();
		p_cheeseburger = 250;
	};
};

NewOrder1::NewOrder1() {
	cheeseburger = 0;
}

void NewOrder1::Cal_Total() {
	Order::Cal_Total();
	total += p_cheeseburger * cheeseburger;
}

void NewOrder1::Set_Cheeseburger( int num ) {
	Input( &cheeseburger, num );
}

int NewOrder1::p_cheeseburger;

#endif
