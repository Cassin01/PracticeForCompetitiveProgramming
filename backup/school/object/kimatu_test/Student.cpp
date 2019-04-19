#include "Pupil.h"

class Student : public Pupil {
    protected:
      int Eng;
    public:
      virtual void SetEng();
      virtual void SetEng(int);
      virtual void AverageScore();
      virtual void DispSocre();
      Student();
};

Student::Student() {
  Eng = 0;
}

void Student::SetEng() {
  Eng = SetScore();
}

void Student::SetEng(int score) {
  Eng = score;
}

void Student::AverageScore() {
  double average = 0.0;
  average = (double)Math +
            (double)Jap  +
            (double)Sci  +
            (double)Soc  +
            (double)Eng;
  average /= 5.0;
  cout << "Average:" << average << endl;
}

void Student::DispSocre() {
  cout << "Mathematics:" << Math << endl;
  cout << "Japanese:"    << Jap  << endl;
  cout << "Science:"     << Sci  << endl;
  cout << "Social:"      << Soc  << endl;
  cout << "English:"     << Eng  << endl;
}

int main(void) {
  Student student[5];
  for (int k = 0; k < 5; ++k) {
    student[k].DispID();
    student[k].SetJap(k * 10);
    student[k].SetMath(k * 20);
    student[k].SetSoc(100 / (k + 1));
    student[k].SetEng(15 * k);
    student[k].AverageScore();
  }
  return 0;
}
