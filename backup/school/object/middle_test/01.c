#include <stdio.h>

struct Price {
  int hamburger;
  int potato;
  int drink;
};

struct Order {
  int number;
  int hamburger;
  int potato;
  int drink;
  int total;
};

struct Price Set_Price(struct Price p); 
void Cal_Total(struct Price p, struct Order *o);

struct Price Set_Price(struct Price p) {
  p.hamburger = 200; 
  p.potato    = 180;
  p.drink     = 150;
  return p;
}

void Cal_Total(struct Price p, struct Order *o) {
  o -> total =
    p.hamburger * o -> hamburger +
    p.potato    * o -> potato    +
    p.drink     * o -> drink;
}

void Disp_Total(struct Order o) {
  printf("支払金額: %d円\n", o.total);
}

void  Set_Hamburger(struct Order *o, int num) {
  o -> hamburger = num;
}

void Set_Potato(struct Order *o, int num) {
  o -> potato = num;
}

void Set_Drink(struct Order *o, int num) {
  o -> drink = num;
}

int main(void) {
  struct Price p;
  struct Order o;
  p = Set_Price(p);
  Set_Hamburger(&o, 1);
  Set_Potato(&o, 2);
  Set_Drink(&o, 3);
  Cal_Total(p, &o);
  Disp_Total(o);
  return 0;
}
