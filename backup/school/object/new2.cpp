#include <iostream>
using namespace std;

class Order {
  protected:
    static int p_hamburger;
    static int p_potato;
    static int p_drink;

    int number;
    int hamburger;
    int potato;
    int drink;
    int total;

    void Input (int*, const char*);

  public:
    Order();
    virtual void Cal_Total();
    void Disp_Total();
    void Set_Hamburger();
    void Set_Potato();
    void Set_Drink();
    static void Set_Price() {
      p_hamburger = 200;
      p_potato = 180;
      p_drink = 150;
    }
};

Order::Order() {
  hamburger = 0;
  potato = 0;
  drink = 0;
  total = 0;
}

void Order::Cal_Total() {
  total = number    * p_hamburger
        + hamburger * p_potato
        + potato    * p_drink;
}

void Order::Disp_Total() {
  cout << "支払金額:" << total << "円 " << endl;
}

void Order::Input(int *item, const char *str) {
  int num;
  do {
    cout << str << "の個数は１桁の正の整数値で入力して下さい";
    cin >> num;
  } while (num > 0);
  *item = num;
}
