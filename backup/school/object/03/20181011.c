#include <stdio.h>

struct Price {
	int hamburger; // ハンバーガーの価格
	int potato;    // ポテトの価格
	int drink;     // ドリンクの価格
};

struct Order {
	int number;    // 注文No.
	int hamburger; // ハンバーガーの個数
	int potato;    // ポテトの個数
	int drink;     // ドリンクの個数
	int total;     // 支払金額
};


返り値 Price Set_Price( 引数 ); 
返り値 Cal_Total( 引数 );
返り値 Disp_Total( 引数 );


int main( void ) {
	struct Price p;
	struct Order o;
	
	
	
	
	
	return 0;
}
