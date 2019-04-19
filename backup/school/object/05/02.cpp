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

    void Input(int *, const char*);

  public:
    Order();
    virtual void Cal_Total();
    void Disp_Totoal();
    void Set_Hamburger();
    void Set_Potato();
    void Set_Drink();
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

void Order::Input(int *item, const char *str) {
  int num;
  do {
    cout << str << "の個数は１桁の正の整数値で入力して下さい。";
    cin >> num;
  } while (num < 0 || num > 9);
  *item = num; 
}

void Order::Set_Hamburger() {
  Input(&hamburger, "ハンバーガ");
}

void Order::Set_Potato() {
  Input(&potato, "ポテト");
}

void Order::Set_Drink() {
  Input(&drink, "ドリンク");
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
    void Set_cheeseburer();
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
  cout << p_cheeseburger << cheeseburger << endl;
  total += p_cheeseburger * cheeseburger;
}

void NewOrder::Set_cheeseburer() {
  Input(&cheeseburger, "チーズバーガ");
}

int NewOrder::p_cheeseburger;

int main(void) {
  NewOrder o;
  NewOrder::Set_Price();
  o.Set_Hamburger();
  o.Set_Potato();
  o.Set_Drink();
  o.Set_cheeseburer();
  o.Cal_Total();
  o.Disp_Totoal();

  return 0;
}
