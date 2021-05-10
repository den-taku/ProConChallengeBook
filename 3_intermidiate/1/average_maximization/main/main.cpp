#include "average_maximization.h"
#include "lib/proconlib.h"
#include <algorithm>
#include <climits>
#include <iostream>
#include <tuple>
#include <vector>
// using namespace std;

int main() {
  using namespace std;
  int n = 0, k = 0;
  cin >> n >> k;
  vector<tuple<int, int>> v_w = {};
  int tmp_v = 0, tmp_w = 0;
  for (int i = 0; i < n; ++i) {
    cin >> tmp_v >> tmp_w;
    v_w.push_back(make_tuple(tmp_v * 1000, tmp_w));
  }

  cout << calculate(v_w, k, binary_search(v_w, k, 0, INT_MAX)) / 1000 << endl;
  return 0;
}
