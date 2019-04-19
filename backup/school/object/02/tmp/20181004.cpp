// 学籍番号：4617043 名前：神保光洋
#include <iostream>
#include <vector>

using namespace std;

int take (int num, int limit_weight, int sol[], int w[], int v[]);
vector<int> num2bit (int n, int num);
int pow2n (int n);

int main( void ) {
  const int num = 6;
  // 荷物の総個数
  char object[num]={'A', 'B', 'C', 'D', 'E', 'F'}; // 荷物の名前
  int v[num] = {700, 400, 800, 1000, 300, 500}; // 価値
  int w[num] = {3, 2, 4, 5, 2, 1}; // 重量
  int limit_weight = 10; // 最大重量
  int choice[num]; // 荷物を選択したかどうか（1...選択, 0...非選択）
  int sol[num]; // 選択した荷物の答え
  int max_value=0; // 総価値
  int value=0; // 価値
  int weight=0; // 重量
  int i;

  // ここに答を記述する（関数を利用しても良い）
  max_value = take (num, limit_weight, sol, w, v);

  printf("Max value = %d \n", max_value );
  for ( i=0; i<num; i++ ) {
    if ( sol[i] == 1 ) {
      printf("%c ", object[i] );
    }
  }
  printf("\n");
  return 0;
}

int showbits (vector <int> bits) {
  for (int i = 0; i < bits.size(); ++i) {
    cout << bits[i];
  }
  cout << endl;
  return 0;
}

int take (int num, int limit_weight, int sol[], int w[], int v[]) {
  int max_val = 0;
  for (int i = 1; i <= pow2n(num) - 1; ++i) {
    vector<int> bits = num2bit (i, num);
    showbits(bits);
    int sum_val = 0;
    int sum_wei = 0;

    for (int i = 0; i < num; ++i) {
      if (bits[i] == 1) {
        sum_val = sum_val + v[i];
        sum_wei = sum_wei + w[i];
      }
    }

    if (sum_wei <= limit_weight) {
      if (sum_val > max_val) {
        max_val = sum_val;
        for (int i = 0; i < num; i++) {
          sol[i] = bits[i];
        }
      }
    }
  }
  return max_val;
}

// 10 -> 2
vector<int> num2bit (int n, int num) {
    vector<int> bits;
    for (int i = 0; i < num; ++i) {
      if (n > 0) {
         bits.push_back(n % 2);
         n = n / 2;
      }
      else {
         bits.push_back(0);
      }
    }
    std::reverse(bits.begin(), bits.end());
    return bits;
}

int pow2n (int n) {
  if (n == 0) {
    return 1;
  }
  else {
    return 2 * pow2n (n - 1);
  }
}
