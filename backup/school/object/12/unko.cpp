#include <iostream>
#include <cmath>
using namespace std;
class Porigon {
	// Progonクラスを記述する
    virtual void set_sides(int x, int y, int z) = 0;
    virtual void set_sides(int x, int y) = 0;
    virtual void set_sides()     = 0;
    virtual void perimeter()     = 0;
    virtual void area()           = 0;
};

class Triangle : public Porigon { // Triangleクラスを記述する
	// Triangleクラスのメンバ関数 set_sides() で三辺の長さをセット、perimeter() で外周の長さを求め表示、area()で面積を求め表示できるようにする。
  private:
    int _x;
    int _y;
    int _z;
  public:
    void set_sides(int x, int y, int z){
      _x = x;
      _y = y;
      _z = z;
    }
    void set_sides(int x, int y){
      cout << "ERR: Triangle.setsides(int x, int y) is not supproted" << endl;
    }
    void set_sides(){}
    void perimeter() {
      cout << "Triangle:perimeter: "
           << _x + _y + _z << endl;
    }
    void area() {
      double s = (double)(_x + _y + _z) / 2;
      cout << "Triangle:area: "
           << sqrt(s * (s - _x) * (s - _y) * (s - _z)) << endl;
    }
};

class Rectangle : public Porigon {
	// Rectangleクラスを記述する
	// Rectangleクラスのメンバ関数 set_sides() で長辺、短辺の長さをセット、perimeter() で外周の長さを求め表示、area()で面積を求め表示できるようにする。
  private:
    int _x;
    int _y;
  public:
    void set_sides(int x, int y, int z){
      cout << "ERR: Rectangle.setsides(int x, int y, int z) is not supproted" << endl;
    }
    void set_sides(int x, int y){
      _x = x;
      _y = y;
    }
    void set_sides(){}
    void perimeter() {
      cout << "Rectangle:perimeter: "
           << (_x + _y) * 2 << endl;
    }
    void area() {
      cout << "Rectangle:area: "
           << _x * _y << endl;
    }
};

int  main( void ) {
	Triangle sankaku;
	Rectangle shikaku;
	sankaku.set_sides( 4, 5, 6 );
	sankaku.perimeter();
	sankaku.area();
	shikaku.set_sides( 5, 7 );
	shikaku.perimeter();
	shikaku.area();
	return 0;
}
