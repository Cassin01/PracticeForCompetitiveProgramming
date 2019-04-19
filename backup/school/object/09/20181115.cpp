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

  public:
    Order();
    virtual void Cal_Total();
    void Disp_Totoal();
    void Set_Hamburger(int num);
    void Set_Potato(int num);
    void Set_Drink(int num);
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
  total =
    p_hamburger * hamburger +
    p_potato    * potato    +
    p_drink     * drink;
}

void Order::Disp_Totoal() {
  cout << "支払金額:" << total << "円" << endl;
}

void Order::Set_Hamburger(int num) {
  hamburger = num;
}

void Order::Set_Potato(int num) {
  potato = num;
}

void Order::Set_Drink(int num) {
  drink = num;
}

int Order::p_hamburger;
int Order::p_potato;
int Order::p_drink;

class NewOrder: public Order {
  private:
    static int p_cheeseburger;
    int cheeseburger;

  public:
    NewOrder();
    void Set_cheeseburer(int num);
    void Cal_Total();
    static void Set_Price() {
      Order::Set_Price();
      p_cheeseburger = 250;
    };
};

NewOrder::NewOrder() {
  cheeseburger = 0;
}

void NewOrder::Cal_Total() {
  Order::Cal_Total();
  total += p_cheeseburger * cheeseburger;
}

void NewOrder::Set_cheeseburer(int num) {
  cheeseburger = num;
}

int NewOrder::p_cheeseburger;

int main(void) {
  NewOrder o;
  NewOrder::Set_Price();
  o.Set_Hamburger(1);
  o.Set_Potato(1);
  o.Set_Drink(1);
  o.Set_cheeseburer(1);
  o.Cal_Total();
  o.Disp_Totoal();

  return 0;
}
