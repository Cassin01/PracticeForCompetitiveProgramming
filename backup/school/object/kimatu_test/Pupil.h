#include <iostream>
using namespace std;

class Pupil {
  protected:
    static int num;
    int StudentID;
    int Jap;
    int Math;
    int Sci;
    int Soc;
  public:
    virtual void DispID();
    virtual int SetScore();
    virtual void SetJap();
    virtual void SetMath();
    virtual void SetSci();
    virtual void SetSoc();
    virtual void SetJap(int);
    virtual void SetMath(int);
    virtual void SetSci(int);
    virtual void SetSoc(int);
    virtual void DispScore();
    virtual void AverageScore();
    Pupil();
    ~Pupil();
};

int Pupil::num = 0;

Pupil::Pupil() {
  StudentID = ++num;
  Jap = 0;
  Math = 0;
  Sci = 0;
  Soc = 0;
}

Pupil::~Pupil() {
  --num;
}

void Pupil::DispID() {
  cout << "StudentID:" << StudentID << endl;
}

int Pupil::SetScore() {
  int score;
  do {
    cout << "Please input score:";
    cin >> score;
  } while (score < 0 || score > 100);
  return score;
}

void Pupil::SetJap() {
  Jap = SetScore();
}

void Pupil::SetJap(int score) {
  Jap = score;
}

void Pupil::SetMath() {
  Math = SetScore();
}

void Pupil::SetMath(int score) {
  Math = score;
}

void Pupil::SetSci() {
  Sci = SetScore();
}

void Pupil::SetSci(int score) {
  Sci = score;
}

void Pupil::SetSoc() {
  Soc = SetScore();
}

void Pupil::SetSoc(int score) {
  Soc = score;
}

void Pupil::DispScore() {
  cout << "Mathematics:" << Math << endl;
  cout << "Japanese:"    << Jap  << endl;
  cout << "Science:"     << Sci  << endl;
  cout << "Social:"      << Soc  << endl;
}

void Pupil::AverageScore() {
  double average = 0.0;
  average = (double)Math + (double)Jap + (double)Sci + (double)Soc;
  average /= 4.0;
  cout << "Average:" << average << endl;
}

/*
int main(void) {
  Pupil pupil[10];
  for (int k = 0; k < 10; k++) {
    pupil[k].DispID();
  }
}
*/

/*
int main(void) {
  Pupil pupil[10];
  pupil[5].SetJap(50);
  pupil[5].SetMath(80);
  pupil[5].SetSci(70);
  pupil[5].SetSoc();
  pupil[5].SetJap();
  pupil[5].DispScore();
  pupil[5].AverageScore();

  return 0;
}
*/
