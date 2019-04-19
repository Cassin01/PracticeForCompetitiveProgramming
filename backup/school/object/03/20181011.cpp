#include <iostream>

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


// 返り値 Price Set_Price( 引数 );
// 返り値 Cal_Total( 引数 );
// 返り値 Disp_Total( 引数 );

int Price_Set_Price(struct Price *price);
int Cal_Total(struct Order *order);
int Disp_Total(struct Order order);

int Price_Set_Price (struct Price *price) {
  price -> hamburger = 200;
  price -> potato = 180;
  price -> drink = 150;

  return 0;
}

int Cal_Total (struct Order *order, struct Price price) {
  std::cout << "ハンバーガーを何個買いますか" << std::endl;
  std::cin >> order -> hamburger;

  std::cout << "ポテトを何個買いますか" << std::endl;
  std::cin >> order -> potato;

  std::cout << "ドリンクを何個買いますか" << std::endl;
  std::cin >> order -> drink;

  order -> total =
    price.hamburger * order -> hamburger +
    price.potato    * order -> potato    +
    price.drink     * order -> drink;

  return 0;
}

int Disp_Total (struct Order order) {
  std::cout << "全部で" << std::endl;
  std::cout << order.total << std::endl;
  std::cout << "です" << std::endl;

  return 0;
}


int main (void) {
  struct Price p;
  struct Order o;

  Price_Set_Price(& p);
  Cal_Total(& o, p);
  Disp_Total(o);

  return 0;
}
