#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

#include <iomanip>
#include <sstream>

using namespace std;

class CityInfo {
  public:
    // デフォルトコントラクタ
    CityInfo() {}
    // カスタムコントラクタ
    CityInfo(int num0, int p0,  int y0, int rank0) : num(num0), y(y0), p(p0), rank(rank0) {}
    // コピーコントラクタ
    //CityInfo(const CityInfo &rhs) : num(rhs.num), y(rhs.y), p(rhs.p), rank(rhs.rank) {}
    // デストラクタ
    ~CityInfo() {}

    static bool cmp_y(CityInfo &a, CityInfo &b) {
      return a.y < b.y;
    }

    void update_rank(int const new_rank) {
      rank = new_rank;
    }
    int get_rank() const {
      return rank;
    }
    int get_num() const {
      return num;
    }
    int get_p() const {
      return p;
    }
  private:
    int num, p, y, rank;
};

  void make_number(int p, int y);

int main(void)
{
  int N, M;
  cin >> N >> M;
  vector<CityInfo> city_infos;
  vector<vector<CityInfo>> p_has_city(N);
  for (int i = 0; i < M; ++i)
  {
    int p, y;
    cin >> p >> y;
    CityInfo city_info1 = CityInfo(i, p, y, 0);
    city_infos.push_back(move(city_info1));
    CityInfo city_info2 = CityInfo(i, p, y, 0);
    p_has_city[p].push_back(move(city_info2));
  }

  for (auto ph : p_has_city)
  {
    sort(ph.begin(), ph.end(), CityInfo::cmp_y);
  }

  for (auto ph : p_has_city)
  {
    for (auto &phh : ph)
    {
      city_infos[phh.get_num()].update_rank(phh.get_rank());
    }
  }

  for (auto &ci : city_infos)
  {
    make_number(ci.get_p(), ci.get_rank() + 1);
  }
  return 0;
}


void make_number(int p, int y) {
  std::ostringstream s0, s1;
  s0 << std::setw(6) << std::setfill('0') << p;
  s1 << std::setw(6) << std::setfill('0') << y;
  std::string s(s0.str() + s1.str());
  std::cout << s << endl;
}