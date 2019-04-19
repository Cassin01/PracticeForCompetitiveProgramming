#ifndef _INC_ORDER    //まだ読み込まれていなければ以下の処理をする
#define _INC_ORDER

#include <iostream>
using namespace std;

class Order{
	protected:
		static int p_hamburger; // ハンバーガーの価格
		static int p_potato;    // ポテトの価格
		static int p_drink;     // ドリンクの価格
	
		int number;    // 注文No.
		int hamburger; // ハンバーガーの個数
		int potato;    // ポテトの個数
		int drink;     // ドリンクの個数
		int total;     // 支払金額
		void Input( int *item, int num );
	
	public:
		Order();
		void Cal_Total();
		void Disp_Total();
		void Set_Hamburger( int num );
		void Set_Potato( int num );
		void Set_Drink( int num );
		static void Set_Price() {
			p_hamburger = 200;
			p_potato = 180;
			p_drink = 150;
		};
};

Order::Order() {
	hamburger = 0;
	potato = 0;
	drink = 0;
	total = 0;
}

void Order::Cal_Total() {
	total = p_hamburger * hamburger + p_potato * potato + p_drink * drink;
}
	
void Order::Disp_Total() {
	cout <<  "支払金額:" << total << "円 \n";
}

void Order::Input( int *item, int num ) {
	if ( num >= 0 && num < 10 ) {
		*item = num;
	} else {
		cout << "個数は1桁の正の整数値で入力して下さい" << endl;
	}
}

void Order::Set_Hamburger( int num ) {
	Input( &hamburger, num );
}

void Order::Set_Potato( int num ) {
	Input( &potato, num );
}

void Order::Set_Drink( int num ) {
	Input( &drink, num );
}

int Order::p_hamburger;
int Order::p_potato;
int Order::p_drink;

#endif
